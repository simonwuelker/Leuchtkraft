use crate::diagnostics::{Annotation, AnnotationType, Diagnostic};
use crate::parser::span::{Span, Spanned};

pub enum Warning {
    DuplicateScopedVariable {
        ident: String,
        first_declaration: Span,
        second_declaration: Span,
    },
    RedundantTrue {
        span: Span,
    },
    NullifyingFalse {
        span: Span,
    },
    RedundantFalse {
        span: Span,
    },
    PurelyLiteralClause {
        span: Span,
    },
}

impl<'a> From<(&'a Warning, &'a str)> for Diagnostic<'a> {
    fn from(other: (&'a Warning, &'a str)) -> Self {
        let annotations = match other.0 {
            Warning::DuplicateScopedVariable {
                ident,
                first_declaration,
                second_declaration,
            } => vec![
                Annotation {
                    annotation_type: AnnotationType::Info,
                    span: *first_declaration,
                    msg: format!("{:?} is first declared here", ident),
                },
                Annotation {
                    annotation_type: AnnotationType::Info,
                    span: *second_declaration,
                    msg: "then declared a second time here".to_owned(),
                },
            ],
            Warning::RedundantTrue { span } => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: *span,
                msg: "'true' in this position does nothing".to_owned(),
            }],
            Warning::NullifyingFalse { span } => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: *span,
                msg: "this 'false' makes the entire and-chain false, preventing any conclusions about the other atoms".to_owned(),
            }],
            Warning::RedundantFalse { span } => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: *span,
                msg: "this 'false' can imply anything".to_owned(),
            }],
            Warning::PurelyLiteralClause { span } => vec![Annotation {
                annotation_type: AnnotationType::Info,
                span: *span,
                msg: "this clause does not contain any variables - and is therefore useless".to_owned(),
            }],
        };

        let msg = match other.0 {
            Warning::DuplicateScopedVariable { ident, .. } => {
                format!("Free variable {:?} was declared multiple times", ident)
            }
            Warning::RedundantTrue { .. } => "Redundant 'true'".to_owned(),
            Warning::NullifyingFalse { .. } => "Nullifying 'false'".to_owned(),
            Warning::RedundantFalse { .. } => "Meaningless implication".to_owned(),
            Warning::PurelyLiteralClause { .. } => "Purely literal clause".to_owned(),
        };

        let note = match other.0 {
            Warning::DuplicateScopedVariable { .. } => None,
            Warning::RedundantTrue { .. } => {
                Some("the expression 'true and x' always evaluates to 'x'")
            }
            Warning::NullifyingFalse { .. } => None,
            Warning::RedundantFalse { .. } => Some("'false' implies both 'true' and 'false'"),
            Warning::PurelyLiteralClause { .. } => None,
        };

        Self {
            code: None,
            buffer: other.1,
            annotation_type: AnnotationType::Warning,
            annotations: annotations,
            msg: msg,
            note: note,
        }
    }
}
