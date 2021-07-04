mod interpreter;

use std::fs;
use anyhow::Result;
use interpreter::{Interpreter, Token};

fn main() -> Result<()> {
    let contents = fs::read_to_string("script.ap")?;
    let mut i = Interpreter::new(contents);

    loop {
        match i.next_token() {
            Token::Comment => {},
            Token::EOF => break,
        }
    }

    Ok(())
}
