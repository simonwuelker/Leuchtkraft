#![feature(slice_take)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod cli;
mod error;
mod interpreter;
mod logic;
mod parser;
mod util;

use interpreter::Interpreter;
use std::fs;
use std::io::Write;
use structopt::StructOpt;

fn main() {
    env_logger::init();

    let options = cli::Options::from_args();

    let mut i = Interpreter::new();
    if let Some(filename) = options.file_name {
        let file = fs::read_to_string(filename).expect("cannot read file");

        for line in file.lines() {
            println!("{:?}", line);
            match i.execute(line) {
                Ok(Some(out)) => println!("=> {}", out),
                Err(err) => {} // util::print_parse_error(err, &line, &filename),
                _ => {}
            }
        }

        if options.interactive {
            run_repl(i);
        }
    } else {
        // Enter a REPL
        run_repl(i);
    }
}

fn run_repl(mut i: Interpreter) {
    let mut buffer = String::new();

    println!("Leuchtkraft Version {}", env!("CARGO_PKG_VERSION"));
    println!("Type 'quit' to exit the shell");

    loop {
        print!("> ");
        std::io::stdout().flush().expect("Cannot flush stdout");
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Cannot read from stdin");
        buffer.pop(); // last char is always a newline

        match buffer.trim() {
            "quit" => break,
            "info" => println!("type 'quit' to exit"),
            _ => match i.execute(&buffer) {
                Ok(Some(out)) => println!("=> {}", out),
                Err(err) => util::print_parse_error(err, &buffer, "REPL"),
                _ => {}
            },
        }
        buffer.clear();
    }
}
