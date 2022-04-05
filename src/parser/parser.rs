//! Converts from tokens to symbols

use super::error::TokenNotFound;
use super::span::{Span, Spanned};
use super::symbol::{Atom, Line};
use super::token::Token;
use super::tokenizer::Tokenizer;
use crate::debug::warning::Warning;
use crate::util::calculate_hash;

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
    pub fn line(
        &self,
        warnings: &mut Vec<Warning>,
    ) -> Result<Option<Spanned<Line>>, TokenNotFound> {
        // Look at how nice PEG grammars look!
        let mut expected = match self.forall(0, warnings) {
            Ok(forall_stmt) => return Ok(Some(forall_stmt)),
            Err(e) => e,
        };

        match self.rule(0, warnings) {
            Ok(rule) => return Ok(Some(rule)),
            Err(e) => expected.join(e),
        }

        match self.empty(0) {
            Ok(_) => return Ok(None),
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
                // After a token, any amount of "filler tokens" are allowed
                self.skip_filler(position);
                Ok(positioned)
            }
            None => Err((*position, expected)),
        }
    }

    /// Read the next token, if it matches any of the expected tokens.
    /// If none of the expected tokens are found, return a corresponding
    /// error.
    fn expect_either(
        &self,
        position: &mut usize,
        expected: Vec<Token>,
    ) -> Result<Spanned<Token>, TokenNotFound> {
        // Check for the first token
        let mut expected_tokens = match self.expect(position, expected[0]) {
            Ok(found) => {
                return Ok(found);
            }
            Err(expected) => TokenNotFound::from(expected),
        };

        // Check the remaining tokens
        for token in expected.into_iter().skip(1) {
            match self.expect(position, token) {
                Ok(found) => {
                    return Ok(found);
                }
                Err(expected) => expected_tokens.join_raw(expected),
            }
        }
        Err(expected_tokens)
    }

    /// Skip any constructs that can always appear inbetween tokens
    /// like whitespaces
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
    ) -> Result<Spanned<Line>, TokenNotFound> {
        self.expect(&mut pos, Token::Forall)?;

        let initial_token = self.expect(&mut pos, Token::Ident)?;
        let start_pos = initial_token.span().0;
        let initial_token_str = self.read_span(initial_token.span());
        let mut idents = vec![initial_token.map(initial_token_str)];

        let mut end_pos = 0;
        while self.expect(&mut pos, Token::Comma).is_ok() {
            let token = self.expect(&mut pos, Token::Ident)?;
            let token_str = self.read_span(token.span());
            end_pos = token.span().1;

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
        let forall = Spanned::new(Line::Forall(idents_clean), Span(start_pos, end_pos));
        Ok(forall)
    }

    /// Return a boolean indicating whether or not the line was indented
    /// and the rule itself
    fn rule(
        &self,
        mut pos: usize,
        warnings: &mut Vec<Warning>,
    ) -> Result<Spanned<Line>, TokenNotFound> {
        let is_indented = self.expect(&mut pos, Token::Indent).is_ok();

        let first_atom = self.read_atom(&mut pos, warnings)?;
        let start = first_atom.span().0;

        let mut atoms = vec![vec![first_atom]];

        // Every rule can only end after having found an implication
        let mut found_implication = false;
        let end = loop {
            let expected = if found_implication {
                vec![Token::And, Token::Implication, Token::End, Token::Comment]
            } else {
                vec![Token::And, Token::Implication]
            };

            let connector = self.expect_either(&mut pos, expected)?;
            match connector.as_inner() {
                Token::Implication => {
                    atoms.push(vec![]);
                    found_implication = true;
                }
                Token::And => {}
                Token::End | Token::Comment => break connector.span().0,
                _ => unreachable!(),
            }
            atoms
                .last_mut()
                .unwrap()
                .push(self.read_atom(&mut pos, warnings)?);
        };

        // Perform some sanity checks and generate warnings
        let mut contains_non_literal = false;
        atoms.iter().enumerate().for_each(|(block_ix, and_chain)| {
            // Check for redundant trues (x and true => y)
            if let Some(true_symbol) = and_chain.iter().find(|atom| atom.as_inner() == &Atom::True)
            {
                if and_chain.len() != 1 {
                    warnings.push(Warning::RedundantTrue {
                        span: true_symbol.span(),
                    });
                }
            }

            // Check for nullifying falses
            if let Some(false_symbol) = and_chain
                .iter()
                .find(|atom| atom.as_inner() == &Atom::False)
            {
                if and_chain.len() != 1 {
                    warnings.push(Warning::NullifyingFalse {
                        span: false_symbol.span(),
                    });
                } else if and_chain.len() == 1 && block_ix != atoms.len() - 1 {
                    warnings.push(Warning::RedundantFalse {
                        span: false_symbol.span(),
                    });
                }
            }

            contains_non_literal |= and_chain
                .iter()
                .find(|atom| !atom.as_inner().is_literal())
                .is_some();
        });

        // remove atom annotations
        let atoms_clean = atoms
            .into_iter()
            .map(|and_chain| {
                and_chain
                    .into_iter()
                    .map(|atom| atom.into_inner())
                    .collect()
            })
            .collect();
        let rule = Spanned::new(Line::Rule(is_indented, atoms_clean), Span(start, end));

        if !contains_non_literal {
            warnings.push(Warning::PurelyLiteralClause { span: rule.span() });
        }

        Ok(rule)
    }

    fn read_atom(
        &self,
        pos: &mut usize,
        _warnings: &mut Vec<Warning>,
    ) -> Result<Spanned<Atom>, TokenNotFound> {
        let found = self.expect_either(pos, vec![Token::True, Token::False, Token::Ident])?;
        match found.as_inner() {
            Token::True => Ok(found.map(Atom::True)),
            Token::False => Ok(found.map(Atom::False)),
            Token::Ident => {
                let ident = calculate_hash(&self.read_span(found.span()));

                let after_ident =
                    self.expect_either(pos, vec![Token::OpeningParen, Token::Questionmark])?;
                match after_ident.as_inner() {
                    Token::Questionmark => {
                        let atom = Atom::Unknown(ident);
                        Ok(Spanned::new(
                            atom,
                            Span(found.span().0, after_ident.span().1),
                        ))
                    }
                    Token::OpeningParen => {
                        // Read the functions arguments
                        let mut ident_strs = vec![];
                        let first =
                            self.expect_either(pos, vec![Token::Ident, Token::ClosingParen])?;
                        let symbol_end = match first.as_inner() {
                            Token::Ident => {
                                ident_strs.push(self.read_span(first.span()));
                                loop {
                                    let next = self.expect_either(
                                        pos,
                                        vec![Token::Comma, Token::ClosingParen],
                                    )?;
                                    match next.as_inner() {
                                        Token::Comma => {
                                            let arg = self.expect(pos, Token::Ident)?;
                                            ident_strs.push(self.read_span(arg.span()));
                                        }
                                        Token::ClosingParen => break next.span().1,
                                        _ => unreachable!("{:?}", next.as_inner()),
                                    }
                                }
                            }
                            Token::ClosingParen => first.span().1, // function has no args, done
                            _ => unreachable!("{:?}", first.as_inner()),
                        };

                        let idents = ident_strs.iter().map(|i| calculate_hash(i)).collect();
                        let atom = Atom::Predicate(ident, idents);
                        Ok(Spanned::new(atom, Span(found.span().0, symbol_end)))
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!("{:?}", found.as_inner()),
        }
    }

    /// Try to read an empty line
    fn empty(&self, mut pos: usize) -> Result<(), TokenNotFound> {
        self.line_end(&mut pos)
    }

    /// Read the buffer contents from a given span
    fn read_span(&self, span: Span) -> &str {
        &self.buffer[span.0..span.1]
    }
}
