use super::error::TokenNotFound;
use super::span::{Span, Spanned};
use super::token::Token;
use super::tokenizer::Tokenizer;
use crate::debug::warning::Warning;
use crate::interpreter::Ident;
use crate::util::calculate_hash;

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
    pub fn line(&self, warnings: &mut Vec<Warning>) -> Result<Line, TokenNotFound> {
        // Look at how nice PEG grammars look!
        let mut expected = match self.forall(0, warnings) {
            Ok(forall) => return Ok(Line::Forall(forall)),
            Err(e) => e,
        };

        match self.empty(0) {
            Ok(_) => return Ok(Line::Empty),
            Err(e) => expected.join(e),
        }

        Err(expected)
    }

    /// Expect the next token to be a specific token type.
    fn expect(
        &self,
        position: &mut usize,
        expected: Token,
    ) -> Result<Spanned<Token>, (usize, Token)> {
        match self.tokenizer.try_read(position, expected) {
            Some(positioned) => {
                self.skip_filler(position);
                Ok(positioned)
            }
            None => Err((*position, expected)),
        }
    }

    /// Skip any constructs that can always appear inbetween tokens
    /// like /* comments */ or whitespaces
    fn skip_filler(&self, position: &mut usize) {
        while let Some(_) = self.tokenizer.try_read(position, Token::Space) {}
    }

    /// Check whether or not the position is a valid line ending
    fn line_end(&self, position: &mut usize) -> Result<(), TokenNotFound> {
        // A line **can** end with a comment, but if no comment
        // is there, don't suggest one (:
        if self.expect(position, Token::Comment).is_err() {
            self.expect(position, Token::End)?;
        }
        Ok(())
    }

    /// Parse a line containing a forall statement, returning
    /// the vec of freed identifiers
    fn forall(
        &self,
        mut pos: usize,
        warnings: &mut Vec<Warning>,
    ) -> Result<Vec<Ident>, TokenNotFound> {
        self.expect(&mut pos, Token::Forall)?;

        let initial_token = self.expect(&mut pos, Token::Ident)?;
        let initial_token_str = self.read_span(initial_token.span());
        let mut idents = vec![initial_token.map(initial_token_str)];

        while self.expect(&mut pos, Token::Comma).is_ok() {
            let token = self.expect(&mut pos, Token::Ident)?;
            let token_str = self.read_span(token.span());

            // Make sure to warn the user if an identifier is freed twice
            let maybe_duplicate = idents.iter().find(|i| i.as_inner() == &token_str);

            if let Some(duplicate) = maybe_duplicate {
                warnings.push(Warning::DuplicateScopedVariable {
                    ident: token_str.to_owned(),
                    first_declaration: duplicate.span(),
                    second_declaration: token.span(),
                });
            }
            idents.push(token.map(token_str));
        }

        self.line_end(&mut pos)?;

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
    ) -> Result<(Spanned<Vec<Ident>>, bool), TokenNotFound> {
        let indented = self.expect(&mut pos, Token::Ident).is_ok();
        unimplemented!()
    }

    fn empty(&self, mut pos: usize) -> Result<(), TokenNotFound> {
        self.line_end(&mut pos)
    }

    fn read_span(&self, span: Span) -> &str {
        &self.buffer[span.0..span.1]
    }
}
