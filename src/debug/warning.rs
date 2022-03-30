use super::annotation::DisplaySnippet;
use crate::parser::span::{Span, Spanned};
use annotate_snippets::snippet::{Annotation, AnnotationType, SourceAnnotation};

pub enum Warning {
    DuplicateScopedVariable {
        ident: String,
        first_declaration: Span,
        second_declaration: Span,
    },
    RedundantTrue {
        span: Span,
    },
}

impl DisplaySnippet for &Warning {
    fn title(&self) -> Annotation {
        let label = match self {
            Warning::DuplicateScopedVariable { .. } => {
                "Scoped variable was declared multiple times"
            }
            Warning::RedundantTrue { .. } => "'true' in this position does nothing",
        };

        let code = match self {
            Warning::DuplicateScopedVariable { .. } => "W001",
            Warning::RedundantTrue { .. } => "W002",
        };

        Annotation {
            label: Some(label),
            id: Some(code),
            annotation_type: AnnotationType::Warning,
        }
    }

    fn footer(&self) -> Vec<Annotation> {
        match self {
            Warning::DuplicateScopedVariable { .. } => {
                vec![
                    Annotation {
                        id: None,
                        label: Some("freeing a variable twice does nothing"),
                        annotation_type: AnnotationType::Note,
                    },
                    Annotation {
                        id: None,
                        label: Some("Remove the second free statement"),
                        annotation_type: AnnotationType::Help,
                    },
                ]
            }
            Warning::RedundantTrue { .. } => {
                vec![
                    Annotation {
                        id: None,
                        label: Some("the expression 'True and <X>' always evaluates to '<X>'"),
                        annotation_type: AnnotationType::Note,
                    },
                    Annotation {
                        id: None,
                        label: Some("Remove the redundant expression"),
                        annotation_type: AnnotationType::Help,
                    },
                ]
            }
        }
    }

    fn source_annotations(&self) -> Vec<SourceAnnotation> {
        match self {
            Warning::DuplicateScopedVariable {
                ident: _,
                first_declaration,
                second_declaration,
            } => {
                vec![
                    SourceAnnotation {
                        range: first_declaration.as_range(),
                        label: "first declared here...",
                        annotation_type: AnnotationType::Info,
                    },
                    SourceAnnotation {
                        range: second_declaration.as_range(),
                        label: "...then declared a second time here",
                        annotation_type: AnnotationType::Info,
                    },
                ]
            }
            Warning::RedundantTrue { span } => {
                vec![SourceAnnotation {
                    range: span.as_range(),
                    label: "this 'true'",
                    annotation_type: AnnotationType::Info,
                }]
            }
        }
    }
}
