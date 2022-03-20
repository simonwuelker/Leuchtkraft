use crate::parser::Rule;
use std::string::ToString;

#[derive(Debug)]
pub struct Error {
    pub variant: ErrorVariant,
    pub line_col: LineColLocation,
}

#[derive(Debug)]
pub enum LineColLocation {
    Pos((usize, usize)),
    Span((usize, usize), (usize, usize)),
}

#[derive(Debug)]
pub enum ErrorVariant {
    /// Like pest::error::ErrorVariant::ParsingError
    SyntaxError {
        positives: Vec<Rule>,
        negatives: Vec<Rule>,
    },
    ParseError(String),
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

impl From<pest::Span<'_>> for LineColLocation {
    fn from(span: pest::Span) -> Self {
        let start = span.start_pos();
        let end = span.end_pos();
        if start == end {
            Self::Pos(start.line_col())
        } else {
            Self::Span(start.line_col(), end.line_col())
        }
    }
}

impl Error {
    pub fn from_pair(span: pest::Span, details: String) -> Self {
        Self {
            variant: ErrorVariant::ParseError(details),
            line_col: LineColLocation::from(span),
        }
    }

    pub fn name(&self) -> &str {
        match self.variant {
            ErrorVariant::SyntaxError{ .. } => "Syntax Error",
            ErrorVariant::ParseError(_) => "Parse Error",
            ErrorVariant::Custom(_) => "Custom Error",
        }
    }

    pub fn details(&self) -> String {
        match &self.variant {
            ErrorVariant::SyntaxError{ positives, negatives: _ } => {
                if positives.len() == 1 {
                    format!("Expected {:?}", positives[0]).clone()
                } else {
                    format!("Expected any of {:?}", positives).clone()
                }
            },
            ErrorVariant::ParseError(msg) => msg.to_string(),
            ErrorVariant::Custom(msg) => msg.to_string(),
        }
    }
}

