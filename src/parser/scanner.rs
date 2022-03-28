use crate::debug::error::{Error, ErrorVariant};
use crate::debug::annotation::InputLocation;

pub struct Scanner<'a> {
    buffer: &'a str,
    pub pos: usize,
}

impl<'a> Scanner<'a> {
    /// Create a new scanner with an underlying buffer
    pub fn new(buffer: &'a str) -> Self {
        Self {
            buffer: buffer,
            pos: 0,
        }
    }

    pub fn is_done(&self) -> bool {
        self.pos == self.buffer.len()
    }

    /// Advance the scanner position by one
    pub fn advance(&mut self) {
        self.pos += 1;
    }

    /// Advance the scanner position by n
    pub fn advance_by(&mut self, n: usize) {
        self.pos += n;
    }

    /// Return the current scanner position
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Return a single character and advance the reader position
    pub fn consume(&mut self) -> Option<char> {
        let c = self.buffer.chars().nth(self.pos);
        self.advance();
        c
    }

    /// Read exactly n characters and advance the reader position accordingly
    pub fn consume_exact(&mut self, n: usize) -> &str {
        let slice = &self.buffer[self.pos..self.pos + n];
        self.advance_by(n);
        slice
    }

    /// Read as long as the read character satisfies the given predicate
    /// and advance the reader position accordingly
    pub fn consume_while<P: 'static>(&mut self, predicate: P) -> Option<&str>
    where &'static mut P: FnMut(&'a char,) -> bool + FnOnce(&'a char,) -> bool,
    P: FnMut(&'a char,) -> bool {
        todo!();
        // let len = self.buffer.chars().skip(self.pos).take_while(&mut predicate).count();
        // if len == 0 {
        //     None
        // } else {
        //     Some(self.consume_exact(len))
        // }
    }


    /// Return a single character without advancing the reader position
    pub fn peek(&self) -> Option<char> {
        self.buffer.chars().nth(self.pos)
    }

    /// Read exactly n characters without advancing the reader position
    pub fn peek_exact(&self, n: usize) -> &str {
       &self.buffer[self.pos..self.pos + n]
    }

    pub fn take(&mut self, expected: char) -> Result<(), Error> {
        let read = self.consume();

        match read {
            Some(found) => {
                if found == expected {
                    Ok(())
                } else {
                    Err(Error::new(ErrorVariant::UnexpectedCharacter{ found: found, expected: Some(vec![expected]) }, InputLocation::Pos(self.pos())))
                }
            },
            None => Err(Error::new(ErrorVariant::UnexpectedEndOfInput, InputLocation::Pos(self.pos()))),
        }
    }
}
