use super::position::Position;
use super::token::Token;
use crate::debug::annotation::DisplaySnippet;
use crate::debug::error::ErrorVariant;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice};

/// Errors during parsing occupy error codes 001-100
pub struct ParseError {
    position: Position,
    variant: ParseErrorVariant,
}

pub enum ParseErrorVariant {
    /// Errors during the first steps in parsing
    UnexpectedCharacter {
        found: char,
        expected: Option<Vec<char>>,
    },
    UnexpectedEndOfInput,
    UnexpectedToken {
        expected: Token,
    },
}

impl ErrorVariant for ParseErrorVariant {
    fn title(&self) -> &str {
        match self {
            ParseErrorVariant::UnexpectedCharacter { .. } => "Unexpected Chararacter",
            ParseErrorVariant::UnexpectedEndOfInput => "Unexpected end of input",
            ParseErrorVariant::UnexpectedToken { .. } => "Unexpected Token",
        }
    }

    fn code(&self) -> usize {
        match self {
            ParseErrorVariant::UnexpectedCharacter { .. } => 1,
            ParseErrorVariant::UnexpectedEndOfInput => 2,
            ParseErrorVariant::UnexpectedToken { .. } => 3,
        }
    }
}

impl ParseError {
    pub fn new(position: Position, variant: ParseErrorVariant) -> Self {
        Self {
            position: position,
            variant: variant,
        }
    }
}

impl DisplaySnippet for ParseError {
    fn title(&self) -> Annotation {
        Annotation {
            label: Some(self.variant.title()),
            id: None,
            annotation_type: AnnotationType::Error,
        }
    }
    fn footer(&self) -> Vec<Annotation> {
        vec![]
    }
    fn slice(&self) -> Vec<Slice> {
        vec![]
    }
}
