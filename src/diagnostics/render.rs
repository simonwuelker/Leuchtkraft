use termcolor::{Color, ColorSpec, WriteColor};

use super::annotation_type::AnnotationType;
use super::diagnostic::Diagnostic;
use crate::repl::Context;
use std::io;

pub fn annotation_color(annotation_type: &AnnotationType) -> ColorSpec {
    let mut spec = ColorSpec::new();
    let color = match annotation_type {
        AnnotationType::Error => Color::Red,
        AnnotationType::Warning => Color::Yellow,
        AnnotationType::Info => Color::Blue,
        AnnotationType::Note => Color::Green,
        AnnotationType::Help => Color::Cyan,
    };
    spec.set_fg(Some(color));
    spec
}

pub fn annotation_name(annotation_type: &AnnotationType) -> &'static str {
    match annotation_type {
        AnnotationType::Error => "error",
        AnnotationType::Warning => "warning",
        AnnotationType::Info => "info",
        AnnotationType::Note => "note",
        AnnotationType::Help => "help",
    }
}

pub trait DisplayDiagnostic {
    fn render(
        &mut self,
        diagnostic: Diagnostic,
        lineno: usize,
        context: &Context,
    ) -> io::Result<()>;
}

impl<W: WriteColor> DisplayDiagnostic for W {
    fn render(
        &mut self,
        diagnostic: Diagnostic,
        lineno: usize,
        context: &Context,
    ) -> io::Result<()> {
        // ================= Diagnostic Message ===========
        // "Error[001]: Forgot to implement the important thing
        self.set_color(annotation_color(&diagnostic.annotation_type).set_bold(true))?;
        write!(self, "{}", annotation_name(&diagnostic.annotation_type))?;

        if let Some(code) = diagnostic.code {
            write!(self, "[{:03}]", code)?;
        }

        // Message is written in bold white
        self.set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))?;
        writeln!(self, ": {}", diagnostic.msg)?;

        // Context indentation depends on the line number
        // TODO: replace with log10 + 1 once the integer_log features gets stabilized
        let lineno_len = format!("{}", lineno).len();

        // =========== Quote Header ===============
        // Context is written as blue "-->" with the info afterwards
        self.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
        write!(self, "{}--> ", " ".repeat(lineno_len))?;
        self.reset()?;

        match context {
            Context::Repl => write!(self, "Repl")?,
            Context::File(path) => write!(self, "{}", path.display())?,
        }
        writeln!(self, ":{}", lineno)?;
        self.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;

        // =========== Quoted Code ===============
        writeln!(self, "{} |", " ".repeat(lineno_len))?;
        write!(self, "{} | ", lineno)?;

        // Quoted code is writtern in default color
        self.reset()?;
        writeln!(self, "{}", diagnostic.buffer)?;

        for annotation in diagnostic.annotations {
            self.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
            write!(self, "{} | ", " ".repeat(lineno_len))?;

            self.set_color(&annotation_color(&annotation.annotation_type))?;
            write!(self, "{}", " ".repeat(annotation.span.0))?;
            write!(
                self,
                "{}-- ",
                "^".repeat(annotation.span.1 - annotation.span.0)
            )?;
            writeln!(self, "{}", annotation.msg)?;
        }

        self.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;

        // Add the note (if any)
        if let Some(note) = diagnostic.note {
            write!(self, "{} =", " ".repeat(lineno_len))?;

            self.set_color(&annotation_color(&AnnotationType::Note))?;
            writeln!(self, "{}", &note)?;
        } else {
            writeln!(self, "{} |", " ".repeat(lineno_len))?;
        }
        self.reset()
    }
}
