use crate::interpreter::Interpreter;
use crate::util;
use std::io::Write;

/// Iterator over lines from stdin
pub struct Repl;

impl Repl {
    pub fn new() -> Self {
        println!("Leuchtkraft version {}", env!("CARGO_PKG_VERSION"));
        println!("Type 'quit' to exit the shell");
        Repl
    }
}

impl Iterator for Repl {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("> ");
        std::io::stdout().flush().expect("Cannot flush stdout");
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Cannot read from stdin");

        buffer.pop(); // last char is always a newline

        match buffer.as_str() {
            "quit" => None,
            _ => Some(buffer),
        }
    }
}

/// Start a interactive Leuchtkraft shell
pub fn run_repl<I>(i: &mut Interpreter, source: I)
where
    I: Iterator<Item = String>,
{
    source.for_each(|line| 
        match i.execute(&line) {
            Ok(response) => {
                if let Some(text) = response.text() {
                    println!("=> {}", text);
                }
                response.warnings().iter().for_each(util::print_snippet);
            }
            Err(err) => util::print_snippet(err),
        }
    );
}
