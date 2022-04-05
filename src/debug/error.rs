use crate::diagnostics::{Annotation, AnnotationType, Diagnostic};
use crate::parser::span::Span;

pub enum Error {
    UnexpectedIndent,
}

impl<'a> From<(&'a Error, &'a str)> for Diagnostic<'a> {
    fn from(other: (&'a Error, &'a str)) -> Self {
        let annotations = match other.0 {
            Error::UnexpectedIndent => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: Span::from(0), // indents are always at the beginning
                msg: "expected no indentation".to_owned(),
            }],
        };

        let msg = match other.0 {
            Error::UnexpectedIndent => "Unexpected indentation level".to_owned(),
        };

        let note = match other.0 {
            Error::UnexpectedIndent { .. } => {
                Some("Any number of spaces/tabs at the beginning of a line count as indentation")
            }
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
