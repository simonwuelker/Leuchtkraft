use super::span::Span;
use super::token::Token;
// use crate::debug::diagnostic::Diagnostic;
use crate::debug::error::ErrorVariant;

/// A token was expected, but not found
pub struct TokenNotFound {
    position: Span,
    expected: Token,
}

impl TokenNotFound {
    pub fn new(span: Span, expected: Token) -> Self {
        Self {
            position: span,
            expected: expected,
        }
    }

    pub fn position(&self) -> Span {
        self.position
    }
}

// impl DisplaySnippet for TokenNotFound {
//     fn title(&self) -> Annotation {
//         Annotation {
//             label: Some("Expected token was not found"),
//             id: None,
//             annotation_type: AnnotationType::Error,
//         }
//     }
//
//     fn footer(&self) -> Vec<Annotation> {
//         let example = match self.expected {
//             Token::Ident => "this_is_a_c0Ol_identifier",
//             Token::Indent => " ",
//             Token::OpeningParen => "(",
//             Token::ClosingParen => ")",
//             Token::Implication => "=>",
//             Token::Questionmark => "?",
//             Token::Forall => "forall",
//             Token::True => "true",
//             Token::False => "false",
//             Token::Comma => ",",
//             Token::Comment => "// this is a comment",
//             Token::Space => " ",
//             Token::End => unreachable!("dont recommend adding an END token"),
//         };
//
//         vec![
//             Annotation {
//                 id: None,
//                 label: Some(example),
//                 annotation_type: AnnotationType::Note,
//             },
//         ]
//     }
//
//     fn source_annotations(&self) -> Vec<SourceAnnotation> {
//         vec![
//             SourceAnnotation {
//                 range: self.position.as_range(),
//                 label: "FIXME", // format here
//                 annotation_type: AnnotationType::Info,
//             },
//         ]
//     }
// }
