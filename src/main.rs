mod interpreter;

use std::fs;
use anyhow::Result;
use interpreter::{Interpreter, Token};

fn main() -> Result<()> {
    let contents = fs::read_to_string("script.ap")?;
    let i = Interpreter::new(contents);
    let tokens = i.tokenize();

    Ok(())
}
