use super::span::Span;
use super::token::Token;
use crate::diagnostics::{Annotation, AnnotationType, Diagnostic};

/// A token was expected, but not found
pub struct TokenNotFound {
    position: usize,
    pub expected: Vec<Token>,
}

impl TokenNotFound {
    pub fn join_raw(&mut self, other: (usize, Token)) {
        if self.position < other.0 {
            self.position = other.0;
            self.expected.truncate(1);
            self.expected[0] = other.1;
        } else if self.position == other.0 {
            self.expected.push(other.1);
        }
    }

    pub fn join(&mut self, mut other: Self) {
        if self.position < other.position {
            self.position = other.position;
            self.expected = other.expected;
        } else if self.position == other.position {
            self.expected.append(&mut other.expected);
        }
    }
}

impl From<(usize, Token)> for TokenNotFound {
    fn from(from: (usize, Token)) -> Self {
        Self {
            position: from.0,
            expected: vec![from.1],
        }
    }
}

impl<'a> From<(TokenNotFound, &'a str)> for Diagnostic<'a> {
    fn from(from: (TokenNotFound, &'a str)) -> Self {
        let span = Span::from(from.0.position);

        let annotation = if from.0.expected.len() == 1 {
            Annotation {
                annotation_type: AnnotationType::Info,
                span: span,
                msg: format!("Expected {:?}", from.0.expected[0]),
            }
        } else {
            Annotation {
                annotation_type: AnnotationType::Info,
                span: span,
                msg: format!("Expected any of {:?}", from.0.expected),
            }
        };

        Self {
            code: None,
            buffer: from.1,
            annotation_type: AnnotationType::Error,
            annotations: vec![annotation],
            msg: "Expected token was not found".to_owned(),
            note: None,
        }
    }
}
