use crate::parser::Rule;
use std::string::ToString;

pub struct Error {
    pub variant: ErrorVariant,
    pub line_col: LineColLocation,
}

pub enum LineColLocation {
    Pos((usize, usize)),
    Span((usize, usize), (usize, usize)),
}

pub enum ErrorVariant {
    /// Like pest::error::ErrorVariant::ParsingError
    SyntaxError {
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
    },
    ParseError,
    Custom(String),
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(pest_error: pest::error::Error<Rule>) -> Self {
        Self {
            variant: pest_error.variant.into(),
            line_col: pest_error.line_col.into(),
        }
    }
}

impl From<pest::error::ErrorVariant<Rule>> for ErrorVariant {
    fn from(pest_variant: pest::error::ErrorVariant<Rule>) -> Self {
        match pest_variant {
            pest::error::ErrorVariant::ParsingError { positives, negatives } => {
                Self::SyntaxError {
                    positives: positives,
                    negatives: negatives,
                }
            },
            pest::error::ErrorVariant::CustomError { message } => {
                Self::Custom(message)
            },
        }
    }
}

impl From<pest::error::LineColLocation> for LineColLocation {
    fn from(pest_location: pest::error::LineColLocation) -> Self {
        match pest_location {
            pest::error::LineColLocation::Pos(pos) => Self::Pos(pos),
            pest::error::LineColLocation::Span(start, end) => Self::Span(start, end),
        }
    }
}

impl Error {
    pub fn name(&self) -> &str {
        match self.variant {
            ErrorVariant::SyntaxError{ .. } => "Syntax Error",
            ErrorVariant::ParseError => "Parse Errr",
            ErrorVariant::Custom(_) => "Custom Error",
        }
    }

    pub fn details(&self) -> String {
        match &self.variant {
            ErrorVariant::SyntaxError{ positives, negatives } => {
                if positives.len() == 1 {
                    format!("Expected {:?}", positives[0]).clone()
                } else {
                    format!("Expected any of {:?}", positives).clone()
                }
            },
            ErrorVariant::ParseError => "parse error here!".to_string(),
            ErrorVariant::Custom(msg) => msg.to_string(),
        }
    }
}

