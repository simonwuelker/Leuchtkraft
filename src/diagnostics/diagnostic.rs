use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;

use super::annotation_type::AnnotationType;
use crate::parser::span::Span;
use crate::repl::Context;

pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub span: Span,
    pub msg: String,
}

pub struct Diagnostic<'a> {
    pub code: Option<usize>,
    pub buffer: &'a str,
    pub annotation_type: AnnotationType,
    pub annotations: Vec<Annotation>,
    pub msg: String,
    pub note: Option<&'a str>,
}

impl<'a> Diagnostic<'a> {
    /// Create a new error diagnostic
    pub fn error(context: Context, buffer: &'a str, msg: String) -> Self {
        Self {
            code: None,
            buffer: buffer,
            annotation_type: AnnotationType::Error,
            annotations: vec![],
            msg: msg,
            note: None,
        }
    }
}
