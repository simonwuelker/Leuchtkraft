extern crate pest;
#[macro_use]
extern crate pest_derive;

mod util;
mod ast;
mod interpreter;
mod parser;
mod error;
mod logic;

use std::fs;
use interpreter::Interpreter;

fn main() {
    env_logger::init();
    let filename = "script.le";
    let unparsed_file = fs::read_to_string(filename).expect("cannot read file");
    match parser::parse_str(&unparsed_file) {
        Ok(ast) => {
            let mut i = Interpreter::new();
            i.traverse(ast).expect("cannot run script");
        },
        Err(error) => {
            util::print_parse_error(error, &unparsed_file, &filename);
        }
    }
}

