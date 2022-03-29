use super::position::Positioned;
use super::token::Token;
use super::tokenizer::Tokenizer;
use crate::debug::error::Error;

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
    Forall(Vec<&'a str>),
    /// (indentation level, rule)
    IndentedRule(usize, Rule),
    /// Unindented (free) rule
    Rule(Rule),
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
    pub fn line(&self) -> Option<Result<Positioned<Token<'a>>, Error>> {
        // Look at how nice PEG grammars look!
        if let Some(stmt) = self.forall(0) {
        } else if let Some(stmt) = self.indented_rule(0) {
        } else if let Some(stmt) = self.rule(0) {
        } else if let Some(stmt) = self.empty(0) {
        }
        unimplemented!()
    }

    /// Expect the next token to be a specific token type.
    fn expect(
        &self,
        token: Token,
        position: &mut usize,
    ) -> Result<Positioned<Token>, Result<Positioned<Token>, Error>> {
        Ok(self.tokenizer.read_next_token(position).unwrap().unwrap())
    }

    fn forall(&self, mut pos: usize) -> Option<Result<Positioned<Vec<&'a str>>, Error>> {
        self.expect(Token::Forall, &mut pos);
        unimplemented!()
    }

    fn indented_rule(&self, mut pos: usize) -> Option<Result<Positioned<Vec<&'a str>>, Error>> {
        unimplemented!()
    }

    fn rule(&self, mut pos: usize) -> Option<Result<Positioned<Vec<&'a str>>, Error>> {
        unimplemented!()
    }

    fn empty(&self, mut pos: usize) -> Option<usize> {
        unimplemented!()
    }
}
