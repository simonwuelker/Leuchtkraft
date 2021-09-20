mod interpreter;

use std::fs;
use anyhow::Result;
use interpreter::{Lexer, Token};

fn main() -> Result<()> {
    let contents = fs::read_to_string("script.le")?;
    let mut i = Lexer::new(contents);
    i.read_tokens();

    Ok(())
}
