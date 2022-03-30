use super::error::{ParseError, ParseErrorVariant};
use super::span::{Span, Spanned};
use super::token::Token;
use super::tokenizer::Tokenizer;
use crate::debug::warning::Warning;
use crate::interpreter::Ident;
use crate::util::calculate_hash;

/// The characters used for indenting a line
pub enum Indentation {
    /// Fixed number of spaces per indentation level
    Spaces(usize),
    /// Tab character
    Tabs,
}

struct Predicate;

type Rule = Vec<Vec<Predicate>>;

pub enum Line {
    /// Forall keyword with list of freed identifiers
    Forall(Vec<Ident>),
    /// (indentation level, rule)
    IndentedRule(Rule),
    /// Unindented (free) rule
    Rule(Rule),
    /// Empty Line
    Empty,
}

/// A data structure that creates a program from tokens
///
/// This is where the last context-free warnings are created.
pub struct Parser<'a> {
    buffer: &'a str,
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            buffer: buffer,
            tokenizer: Tokenizer::new(buffer),
        }
    }

    /// Try to parse the internal buffer as a line
    pub fn line(&self, warnings: &mut Vec<Warning>) -> Result<Line, ParseError> {
        // Look at how nice PEG grammars look!
        let forall = self.forall(0, warnings)?;
        Ok(Line::Forall(forall))
        // if let Ok(stmt) = self.forall(0, warnings) {
        // } else if let Ok(stmt) = self.rule(0, warnings) {
        // } else if self.empty(0).is_ok() {
        // }
    }

    /// Expect the next token to be a specific token type.
    /// Return Some(token) if the token matched
    /// Return Err(Ok(token)) if a different token was read
    /// Return Err(Err()) if no token could be read
    fn expect(&self, position: &mut usize, expected: Token) -> Result<Spanned<Token>, ParseError> {
        match self.tokenizer.try_read(position, expected) {
            Some(positioned) => {
                self.skip_filler(position);
                Ok(positioned)
            }
            None => Err(ParseError::new(
                Span::position(*position),
                ParseErrorVariant::UnexpectedToken { expected: expected },
            )),
        }
    }

    /// Skip any constructs that can always appear inbetween tokens
    /// like /* comments */ or whitespaces
    fn skip_filler(&self, position: &mut usize) {
        let mut found = self.tokenizer.try_read(position, Token::Space);
        while let Some(_) = found {
            found = self.tokenizer.try_read(position, Token::Space);
        }
    }

    /// Parse a line containing a forall statement, returning
    /// the vec of freed identifiers
    fn forall(
        &self,
        mut pos: usize,
        warnings: &mut Vec<Warning>,
    ) -> Result<Vec<Ident>, ParseError> {
        self.expect(&mut pos, Token::Forall)?;

        let initial_token = self.expect(&mut pos, Token::Ident)?;
        let initial_token_str = self.read_span(initial_token.span());
        let mut idents = vec![initial_token.map(initial_token_str)];

        while self.expect(&mut pos, Token::Comma).is_ok() {
            let token = self.expect(&mut pos, Token::Ident)?;
            let token_str = self.read_span(token.span());

            // Make sure to warn the user if an identifier is freed twice
            // TODO: forall x,y,y doesnt get flagged
            let maybe_duplicate = idents.iter().find(|i| i.as_inner() == &token_str);
            if let Some(duplicate) = maybe_duplicate {
                warnings.push(Warning::DuplicateScopedVariable {
                    ident: token_str.to_owned(),
                    first_declaration: duplicate.span(),
                    second_declaration: token.span(),
                });
            }
            idents.push(token.map(initial_token_str));
        }
        self.expect(&mut pos, Token::End)?;

        // Remove source code annotations
        let idents_clean = idents
            .into_iter()
            .map(|x| x.into_inner())
            .map(|x| calculate_hash(&x))
            .collect();
        Ok(idents_clean)
    }

    /// Return a boolean indicating whether or not the line was indented
    /// and the rule itself
    fn rule(
        &self,
        mut pos: usize,
        warnings: &mut Vec<Warning>,
    ) -> Result<(Spanned<Vec<Ident>>, bool), ParseError> {
        let indented = self.expect(&mut pos, Token::Ident).is_ok();
        unimplemented!()
    }

    fn empty(&self, mut pos: usize) -> Result<(), ParseError> {
        self.expect(&mut pos, Token::End)?;
        Ok(())
    }

    fn read_span(&self, span: Span) -> &str {
        &self.buffer[span.0..span.1]
    }
}
