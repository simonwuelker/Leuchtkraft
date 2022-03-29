use crate::interpreter::Interpreter;
use crate::util;
use std::io::Write;

/// Iterator over lines from stdin,
/// buffers the most recent line
pub struct Repl {
    buffer: String,
}

impl Repl {
    pub fn new() -> Self {
        println!("Leuchtkraft version {}", env!("CARGO_PKG_VERSION"));
        println!("Type 'quit' to exit the shell");
        Repl {
            buffer: String::new(),
        }
    }
}

impl Iterator for Repl {
    type Item<'a> = &'a str;

    fn next(&'a mut self) -> Option<Self::Item> {
        print!("> ");
        std::io::stdout().flush().expect("Cannot flush stdout");
        std::io::stdin()
            .read_line(&mut self.buffer)
            .expect("Cannot read from stdin");

        self.buffer.pop(); // last char is always a newline

        match self.buffer.as_str() {
            "quit" => None,
            _ => Some(&self.buffer),
        }
    }
}

/// Start a interactive Leuchtkraft shell
pub fn run_repl<'a, I>(i: &mut Interpreter, source: I)
where
    I: Iterator<Item = &'a str>,
{
    source.for_each(|line| match i.execute(line) {
        Ok(response) => {
            if let Some(text) = response.text() {
                println!("=> {}", text);
            }
            response.warnings().iter().for_each(util::print_snippet);
        }
        Err(err) => util::print_snippet(err),
    });
}
