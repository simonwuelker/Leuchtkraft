use super::error::{ParseError, ParseErrorVariant};
use super::position::{Position, Positioned};
use super::token::Token;
use super::tokenizer::Tokenizer;
use crate::debug::warning::Warning;

type ParseResult<T> = Result<(T, Vec<Warning>), ParseError>;

/// The characters used for indenting a line
pub enum Indentation {
    /// Fixed number of spaces per indentation level
    Spaces(usize),
    /// Tab character
    Tabs,
}

struct Predicate;

type Rule = Vec<Vec<Predicate>>;

pub enum Line<'a> {
    /// Forall keyword with list of freed identifiers
    Forall(Vec<&'a str>),
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
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(buffer),
        }
    }

    /// Try to parse the internal buffer as a line
    pub fn line(&self) -> ParseResult<Positioned<Token>> {
        // Look at how nice PEG grammars look!
        if let Ok(stmt) = self.forall(0) {
        } else if let Ok(stmt) = self.rule(0) {
        } else if self.empty(0).is_ok() {
        }
        unimplemented!()
    }

    /// Expect the next token to be a specific token type.
    /// Return Some(token) if the token matched
    /// Return Err(Ok(token)) if a different token was read
    /// Return Err(Err()) if no token could be read
    fn expect(
        &self,
        position: &mut usize,
        expected: Token,
    ) -> Result<Positioned<Token>, ParseError> {
        match self.tokenizer.try_read(position, expected) {
            Some(positioned) => Ok(positioned),
            None => Err(ParseError::new(
                Position::Pos(*position),
                ParseErrorVariant::UnexpectedToken { expected: expected },
            )),
        }
    }

    /// Skip any constructs that can always appear inbetween tokens
    /// like /* comments */ or whitespaces
    fn skip_filler(&self, position: &mut usize) {}

    /// Parse a line containing a forall statement, returning
    /// the vec of freed identifiers
    fn forall(&self, mut pos: usize) -> ParseResult<Vec<Positioned<Token>>> {
        self.expect(&mut pos, Token::Forall)?;

        let mut idents = vec![];
        idents.push(self.expect(&mut pos, Token::Ident)?);

        while self.expect(&mut pos, Token::Comma).is_ok() {
            idents.push(self.expect(&mut pos, Token::Ident)?);
        }
        self.expect(&mut pos, Token::End)?;
        Ok((idents, vec![]))
    }

    /// Return a boolean indicating whether or not the line was indented
    /// and the rule itself
    fn rule(&self, mut pos: usize) -> ParseResult<(Positioned<Vec<&'a str>>, bool)> {
        unimplemented!()
    }

    fn empty(&self, mut pos: usize) -> Result<(), ParseError> {
        self.expect(&mut pos, Token::End)?;
        Ok(())
    }
}
