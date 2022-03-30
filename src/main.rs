#![doc = include_str!("../README.md")]

mod cli;
mod debug;
mod interpreter;
mod logic;
mod parser;
mod repl;
mod util;

use anyhow::{Context, Result};
use debug::panic;
use interpreter::Interpreter;
use repl::{run_repl, Repl};
use std::fs;
use structopt::StructOpt;

fn main() -> Result<()> {
    panic::init();

    let options = cli::Options::from_args();

    let mut i = Interpreter::new();
    if let Some(filename) = options.file_name {
        let file =
            fs::read_to_string(&filename).with_context(|| format!("Cannot open {:?}", filename))?;

        run_repl(
            &mut i,
            file.lines().map(|l| l.to_owned()),
            filename.to_str(),
        );

        if options.interactive {
            run_repl(&mut i, Repl::new(), None);
        }
    } else {
        // Enter a REPL
        run_repl(&mut i, Repl::new(), None);
    }
    Ok(())
}
