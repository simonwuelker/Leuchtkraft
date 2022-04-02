#![doc = include_str!("../README.md")]

mod cli;
mod debug;
mod diagnostics;
mod interpreter;
mod logic;
mod parser;
mod repl;
mod util;

use debug::panic;
use interpreter::Interpreter;
use repl::{run_repl, Context, Repl};
use std::fs;
use termcolor::{ColorChoice, StandardStream};

fn main() {
    // panic::init(); // Initialize custom panic handler

    if let Some(options) = cli::Options::from_args() {
        let colors = if options.no_color {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        };
        let mut stdout = StandardStream::stdout(colors);

        let mut i = Interpreter::new();
        if let Some(filename) = options.file_name {
            let file = fs::read_to_string(&filename).unwrap();

            run_repl(
                &mut i,
                file.lines().map(|l| l.to_owned()),
                Context::File(filename),
                &mut stdout,
            );

            if options.interactive {
                run_repl(&mut i, Repl::new(), Context::Repl, &mut stdout);
            }
        } else {
            // Enter a REPL
            run_repl(&mut i, Repl::new(), Context::Repl, &mut stdout);
        }
    }
}
