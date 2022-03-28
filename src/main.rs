#![doc = include_str!("../README.md")]

mod cli;
mod interpreter;
mod logic;
mod debug;
mod parser;
mod util;

use interpreter::Interpreter;
use std::fs;
use std::io::Write;
use structopt::StructOpt;
use debug::panic;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    panic::init();

    let options = cli::Options::from_args();

    let mut i = Interpreter::new();
    if let Some(filename) = options.file_name {
        let file = fs::read_to_string(&filename)
            .with_context(|| format!("Cannot open {:?}", filename))?;

        for line in file.lines() {
            match i.execute(line) {
                Ok(response) => {
                    if let Some(text) = response.text() {
                        println!("=> {}", text);
                    } 
                    response.warnings().iter().for_each(util::print_snippet);
                },
                Err(err) => util::print_snippet(err),
            }
        }

        if options.interactive {
            run_repl(i)?;
        }
    } else {
        // Enter a REPL
        run_repl(i)?;
    }
    Ok(())
}

/// Start a interactive Leuchtkraft shell
fn run_repl(mut i: Interpreter) -> Result<()> {
    let mut buffer = String::new();

    println!("Leuchtkraft version {}", env!("CARGO_PKG_VERSION"));
    println!("Type 'quit' to exit the shell");

    loop {
        print!("> ");
        std::io::stdout().flush().context("Cannot flush stdout")?;
        std::io::stdin()
            .read_line(&mut buffer).context("Cannot read from stdin")?;

        buffer.pop(); // last char is always a newline

        match buffer.trim() {
            "quit" => break,
            _ => match i.execute(&buffer) {
                Ok(response) => {
                    if let Some(text) = response.text() {
                        println!("=> {}", text);
                    } 
                    response.warnings().iter().for_each(util::print_snippet);
                },
                Err(err) => util::print_snippet(err),
            },
        }
        buffer.clear();
    }
    Ok(())
}
