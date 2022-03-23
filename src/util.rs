// use pest::error::{Error, LineColLocation, ErrorVariant};
use crate::error::*;
use colored::*;

struct Printer {
    file_context: bool,
    line_nr: usize,
    prefix: ColoredString,
}

impl Printer {
    pub fn new() -> Self {
        Self {
            line_nr: 0,
            file_context: false,
            prefix: "".white(),
        }
    }

    pub fn annotate(&mut self, line_col: &LineColLocation, msg: &str) {
        match line_col {
            LineColLocation::Pos((row, col)) => {
                self.print(&format!(
                    "{}{}{}",
                    " ".repeat(*col),
                    "^___ ".red(),
                    msg.red()
                ));
            }
            LineColLocation::Span(start, end) => {
                self.print(&format!(
                    "{}{}{}{}",
                    " ".repeat(start.1 - 1),
                    "^".repeat(end.1 - start.1).red(),
                    "___ ".red(),
                    msg.red(),
                ));
            }
        }
    }

    pub fn print_with_line_nr(&mut self, msg: &str) {
        println!("{}{}", self.prefix(true), msg);
    }

    fn prefix(&mut self, show_line_nr: bool) -> ColoredString {
        if self.file_context {
            if show_line_nr {
                self.line_nr += 1;
                format!("{} |     ", self.line_nr - 1).blue()
            } else {
                format!(
                    "{} |     ",
                    " ".repeat(self.line_nr.to_string().chars().count())
                )
                .blue()
            }
        } else {
            "".white()
        }
    }

    pub fn print<T: std::fmt::Display>(&mut self, msg: T) {
        println!("{}{}", self.prefix(false), msg);
    }

    pub fn start_context(&mut self, filename: &str, line: usize, col: usize) {
        println!(" {} {}:{}:{}", "-->".blue(), filename, line, col);
        self.file_context = true;
        self.line_nr = line;
        self.print("");
    }

    pub fn end_context(&mut self) {
        self.file_context = false;
    }
}

pub fn print_parse_error(err: Error, file: &str, filename: &str) {
    let mut printer = Printer::new();
    printer.print(&err.name().red().bold());

    match &err.line_col {
        LineColLocation::Pos(pos) => {
            printer.start_context(filename, pos.0, pos.1);
            let faulty_line = file.lines().nth(pos.0 - 1).unwrap();
            printer.print_with_line_nr(faulty_line);
            printer.annotate(&err.line_col, &err.details());
            printer.end_context();
            printer.print("");
        }
        LineColLocation::Span(start, end) => {
            if start.0 == end.0 {
                printer.start_context(filename, start.0, start.1);
                let faulty_line = file.lines().nth(start.0 - 1).unwrap();
                printer.print_with_line_nr(faulty_line);
                printer.annotate(&err.line_col, &err.details());
                printer.end_context();
                printer.print("");
            } else {
                unimplemented!("multiline annotations");
            }
        }
    }
}
