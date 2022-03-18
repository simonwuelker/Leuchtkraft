extern crate pest;
#[macro_use]
extern crate pest_derive;

mod util;
mod ast;
mod interpreter;
mod parser;

use std::fs;
use interpreter::Interpreter;

fn main() {
    let filename = "simple.le";
    let unparsed_file = fs::read_to_string(filename).expect("cannot read file");
    match parser::parse_str(&unparsed_file) {
        Ok(ast) => {
            println!("Parsed Successfully");
            let mut i = Interpreter::new();
            i.traverse(ast);
        },
        Err(error) => {
            util::print_parse_error(error, &unparsed_file, &filename);
        }
    }
}

