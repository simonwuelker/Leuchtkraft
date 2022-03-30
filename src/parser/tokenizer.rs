use super::span::{Span, Spanned};
use super::token::Token;

/// A tokenizer that converts an input stream of bytes into an output
/// stream of tokens, received via the [read_next_token] function.
pub struct Tokenizer<'a> {
    buffer: &'a str,
}

impl<'a> Tokenizer<'a> {
    /// Create a new Lexer from an input buffer
    pub fn new(buffer: &'a str) -> Self {
        Self { buffer: buffer }
    }

    pub fn try_read(&'a self, pos: &mut usize, token: Token) -> Option<Spanned<Token>> {
        match token {
            Token::Ident => self
                .consume_while(pos, |c: &char| c.is_alphanumeric() || c == &'_')
                .map(|o| o.map(Token::Ident)),
            Token::OpeningParen => self.take(pos, "(").map(|o| o.map(Token::OpeningParen)),
            Token::ClosingParen => self.take(pos, ")").map(|o| o.map(Token::OpeningParen)),
            Token::Implication => self.take(pos, "=>").map(|o| o.map(Token::Implication)),
            Token::Questionmark => self.take(pos, "?").map(|o| o.map(Token::Questionmark)),
            Token::Comma => self.take(pos, ",").map(|o| o.map(Token::Comma)),
            Token::Forall => self.take(pos, "forall").map(|o| o.map(Token::Forall)),
            Token::True => self.take(pos, "true").map(|o| o.map(Token::True)),
            Token::False => self.take(pos, "false").map(|o| o.map(Token::False)),
            Token::SinglelineComment => self
                .take(pos, "//")
                .map(|o| o.map(Token::SinglelineComment)),
            Token::MultilineCommentOpen => self
                .take(pos, "/*")
                .map(|o| o.map(Token::MultilineCommentOpen)),
            Token::MultilineCommentClose => self
                .take(pos, "*/")
                .map(|o| o.map(Token::MultilineCommentClose)),
            Token::Space => self
                .consume_if(pos, |c| c.is_whitespace())
                .map(|o| o.map(Token::Space)),
            Token::Tab => self.take(pos, "\t").map(|o| o.map(Token::Tab)),
            Token::Character => self.consume(pos).map(|o| o.map(Token::Character)),
            Token::Indent => self
                .consume_while(pos, |c| c.is_whitespace())
                .map(|o| o.map(Token::Ident)),
            Token::End => {
                if *pos == self.buffer.len() {
                    Some(Spanned::new(Token::End, Span::position(*pos)))
                } else {
                    None
                }
            }
        }
    }

    /// Return a single character and advance the reader position
    fn consume(&'a self, pos: &mut usize) -> Option<Spanned<char>> {
        let c = self.buffer.chars().nth(*pos)?;
        let res = Spanned::new(c, Span::position(*pos));
        *pos += 1;
        Some(res)
    }

    /// Consume if the next character matches the given predicate.
    fn consume_if<P: 'static>(&'a self, pos: &mut usize, predicate: P) -> Option<Spanned<char>>
    where
        P: FnOnce(&char) -> bool,
    {
        let initial_pos = *pos;
        let c = self.buffer.chars().skip(*pos).nth(0);
        match c {
            Some(character) => {
                if predicate(&character) {
                    *pos += 1;
                    Some(Spanned::new(character, Span(initial_pos, *pos)))
                } else {
                    *pos = initial_pos; // revert reader
                    None
                }
            }
            None => None,
        }
    }

    /// Return a single character and advance the reader position
    ///
    /// # Panic
    /// Panics if the end position is outside the buffer range
    fn consume_exact(&'a self, pos: &mut usize, n: usize) -> Option<Spanned<&'a str>> {
        let end = *pos + n;
        if self.buffer.len() < n {
            None
        } else {
            let res = Spanned::new(&self.buffer[*pos..end], Span(*pos, end));
            *pos += n;
            Some(res)
        }
    }

    /// Read as long as the read character satisfies the given predicate
    /// and advance the reader position accordingly
    fn consume_while<P: 'static>(&self, pos: &mut usize, predicate: P) -> Option<Spanned<&str>>
    where
        P: FnMut(&char) -> bool,
    {
        let len = self.buffer.chars().skip(*pos).take_while(predicate).count();
        if len == 0 {
            None
        } else {
            self.consume_exact(pos, len)
        }
    }

    /// Return a single character without advancing the reader position
    fn peek(&self, pos: &usize) -> Option<char> {
        self.buffer.chars().nth(*pos)
    }

    /// Read exactly n characters without advancing the reader position
    fn peek_exact(&self, pos: usize, n: usize) -> &str {
        &self.buffer[pos..pos + n]
    }

    fn take(&self, pos: &mut usize, expected: &str) -> Option<Spanned<()>> {
        let initial_pos = *pos;

        for expected_char in expected.chars() {
            let read = self.consume(pos);

            match read {
                Some(found) => {
                    let found_c = found.into_inner();
                    if found_c != expected_char {
                        return None;
                    }
                }
                None => return None,
            }
        }
        Some(Spanned::new((), Span(initial_pos, *pos)))
    }
}
