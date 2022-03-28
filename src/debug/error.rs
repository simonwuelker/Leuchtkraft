use annotate_snippets::snippet::{Annotation, AnnotationType, Slice};
use super::annotation::{DisplaySnippet, InputLocation};

#[derive(Debug)]
/// Information about an interruption of the execution that cannot
/// be recovered from.
pub struct Error {
    pub variant: ErrorVariant,
    pub location: InputLocation,
}

#[derive(Debug)]
pub enum ErrorVariant {
    /// Errors during the first steps in parsing
    UnexpectedCharacter {
        found: char,
        expected: Option<Vec<char>>,
    },
    UnexpectedEndOfInput,
    /// Syntactically correct statements that make no logical sense
    /// like true => false
    ParseError(String),
    /// Custom Errors
    Custom(String),
}

impl DisplaySnippet for Error {
    fn title(&self) -> Annotation {
        Annotation {
            label: Some(self.variant.title()),
            id: Some(self.variant.code()),
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

impl ErrorVariant {
    pub fn title(&self) -> &str {
        match self {
            Self::UnexpectedCharacter{ .. } => "Unexpected character",
            Self::UnexpectedEndOfInput => "Unexpected end of input",
            Self::ParseError{ .. } => "Parse Error",
            Self::Custom{ .. } => "Custom Error",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::UnexpectedCharacter{ .. } => "E001",
            Self::UnexpectedEndOfInput => "E002",
            Self::ParseError{ .. } => "E003",
            Self::Custom{ .. } => "E004",
        }
    }
}


impl Error {
    pub fn new(variant: ErrorVariant, location: InputLocation) -> Self {
        Self {
            variant: variant,
            location: location,
        }
    }
}
