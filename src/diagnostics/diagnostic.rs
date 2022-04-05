use super::annotation_type::AnnotationType;
use crate::parser::span::Span;

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
