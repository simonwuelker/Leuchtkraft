use super::scanner::Scanner;
use super::token::{Token, Keyword};
use super::span::Spanned;
use crate::debug::error::{Error, ErrorVariant};
use crate::debug::annotation::InputLocation;
use std::collections::VecDeque;
use std::iter::{Peekable, Enumerate};

/// A parser for right-linear (not sure if thats the right word) grammars
pub struct Lexer<'a, T: Iterator<Item = char> {
    scanner: Peekable<Enumerate<T>>,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer from an input buffer
    pub fn new(buffer: &'a str) -> Self {
        Self {
            scanner: buffer.chars().enumerate().peekable(),
        }
    }

    fn read_next_token(&'a mut self) -> Result<Option<Spanned<Token<'a>>>, Error> {
        match self.scanner.peek() {
            Some((initial_pos, c)) => {
                match c {
                    Some('a'..='z' | 'A'..='Z' | '_' ) => {
                        let ident = self.scanner.consume_while(|c: &char| c.is_alphanumeric() || c == &'_').unwrap();
                        let span = (initial_pos, initial_pos + ident.len());
                        match ident {
                            "forall" => Spanned::new(Token::Keyword(Keyword::Forall), span),
                            "true" => Spanned::new(Token::Keyword(Keyword::True), span),
                            "false" => Spanned::new(Token::Keyword(Keyword::False), span),
                            _ => Spanned::new(Token::Ident(&ident),
                        }
                    },
                    Some(' ') => {
                        self.scanner.advance();
                        Token::Indent
                    },
                    Some('=') => {
                        self.scanner.advance();
                        match self.scanner.take('>') {
                            Ok(_) => Token::Implication,
                            Err(e) => unimplemented!(),
                        }
                    },
                    Some('(') => {
                        self.scanner.advance();
                        Token::OpeningParen
                    },
                    Some(')') => {
                        self.scanner.advance();
                        Token::ClosingParen
                    },
                    Some(x) => {
                        return Err(Error::new(
                            ErrorVariant::UnexpectedCharacter{ found: x, expected: None },
                            InputLocation::Pos(self.scanner.pos()),
                        ));
                    }
                }
            }
            None => Ok(None),
        }

        // Following a token, there can be an arbitrary number of whitespaces
        self.scanner.consume_while(|c: &char| c == &' ');

        let span = (initial_pos, self.scanner.pos());
        Ok(Some(Spanned::new(token, span)))
    }

    pub fn tokenize(&'a mut self) -> Result<VecDeque<Spanned<Token<'a>>>, Error> {
        let mut tokens = VecDeque::new();
        while let Some(token) = self.read_next_token()? {
            tokens.push_back(token);
        }
        Ok(tokens)
    }
}
