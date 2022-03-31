//! Defines behaviour shared by every Leuchtkraft error type

// use super::diagnostic::Diagnostic;

pub trait ErrorVariant {
    fn title(&self) -> &str;
    /// Every error is assigned a unique code.
    /// Parse errors occupy the 0-100 range
    fn code(&self) -> usize;
}

// impl DisplaySnippet for Error {
//     fn title(&self) -> Annotation {
//         Annotation {
//             label: Some(self.variant.title()),
//             id: Some(self.variant.code()),
//             annotation_type: AnnotationType::Error,
//         }
//     }
//     fn footer(&self) -> Vec<Annotation> {
//         vec![]
//     }
//     fn slice(&self) -> Vec<Slice> {
//         vec![]
//     }
// }

// impl ErrorVariant {
//     pub fn title(&self) -> &str {
//         match self {
//             Self::UnexpectedCharacter { .. } => "Unexpected character",
//             Self::UnexpectedEndOfInput => "Unexpected end of input",
//             Self::ParseError { .. } => "Parse Error",
//             Self::Custom { .. } => "Custom Error",
//         }
//     }
//
//     pub fn code(&self) -> &str {
//         match self {
//             Self::UnexpectedCharacter { .. } => "E001",
//             Self::UnexpectedEndOfInput => "E002",
//             Self::ParseError { .. } => "E003",
//             Self::Custom { .. } => "E004",
//         }
//     }
// }
//
// impl Error {
//     pub fn new(variant: ErrorVariant, location: Position) -> Self {
//         Self {
//             variant: variant,
//             location: location,
//         }
//     }
// }
