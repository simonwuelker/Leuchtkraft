use crate::diagnostics::{Context, Diagnostic, DisplayDiagnostic};
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
pub fn run_repl<I, W>(i: &mut Interpreter, source: I, context: Context, writer: &mut W)
where
    I: Iterator<Item = String>,
    W: termcolor::WriteColor,
{
    source.enumerate().for_each(|(ix, line)| {
        let lineno = ix + 1;
        match i.execute(&line) {
            Ok(response) => {
                if let Some(text) = response.text() {
                    println!("=> {}", text);
                }

                response
                    .warnings()
                    .iter()
                    .map(|warning| Diagnostic::from((warning, line.as_ref())))
                    .for_each(|diagnostic| writer.render(diagnostic, lineno, &context).unwrap());
            }
            Err(err) => {} // util::print_snippet(err, &line, lineno, source_name),
        }
    });
}
