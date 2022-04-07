use crate::diagnostics::{Annotation, AnnotationType, Diagnostic};
use crate::parser::span::Span;

pub enum Error {
    UnexpectedIndent,
    FreedVarInQuestion { span: Span },
}

impl<'a> From<(Error, &'a str)> for Diagnostic<'a> {
    fn from(other: (Error, &'a str)) -> Self {
        let annotations = match other.0 {
            Error::UnexpectedIndent => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: Span::from(0), // indents are always at the beginning
                msg: "expected no indentation".to_owned(),
            }],
            Error::FreedVarInQuestion { span } => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: span,
                msg: format!("{} was previously freed", &other.1[span.0..span.1]),
            }],
        };

        let msg = match other.0 {
            Error::UnexpectedIndent => "Unexpected indentation level".to_owned(),
            Error::FreedVarInQuestion { span } => format!(
                "{} is a free variable and may not be used in a question",
                &other.1[span.0..span.1]
            ),
        };

        let note = match other.0 {
            Error::UnexpectedIndent { .. } => {
                Some("any number of spaces/tabs at the beginning of a line count as indentation")
            }
            Error::FreedVarInQuestion { .. } => Some(
                "free variables in questions are intended to be implemented in a future release",
            ),
        };

        Self {
            code: None,
            buffer: other.1,
            annotation_type: AnnotationType::Error,
            annotations: annotations,
            msg: msg,
            note: note,
        }
    }
}
