use crate::diagnostics::{Diagnostic, DisplayDiagnostic};
use crate::interpreter::Interpreter;
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

/// The source of code
pub enum Context {
    /// User inputted code line by line
    Repl,

    /// Code read from a file with provided filename
    File(std::path::PathBuf),
}

impl Context {
    /// Determine whether or not the repl should exit after an error is encountered
    pub fn exit_on_error(&self) -> bool {
        match self {
            Context::Repl => false,
            Context::File(_) => true,
        }
    }
}

/// Start a interactive Leuchtkraft shell
pub fn run_repl<I, W>(i: &mut Interpreter, source: I, ctx: Context, writer: &mut W)
where
    I: Iterator<Item = String>,
    W: termcolor::WriteColor,
{
    for (ix, line) in source.enumerate() {
        let lineno = ix + 1;
        let (warnings, result) = i.execute(&line);

        // Print all the warnings
        warnings
            .iter()
            .map(|warning| Diagnostic::from((warning, line.as_ref())))
            .for_each(|diagnostic| writer.render(diagnostic, lineno, &ctx).unwrap());

        // Print either the result (if any) or the errors that occured
        match result {
            Ok(Some(text)) => {
                println!("=> {}", text);
            }
            Ok(None) => {}
            Err(error) => {
                writer.render(error, lineno, &ctx).unwrap();

                if ctx.exit_on_error() {
                    return;
                }
            }
        }
    }
}
