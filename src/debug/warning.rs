use super::annotation::DisplaySnippet;
use crate::parser::position::Position;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice};

#[derive(Debug)]
pub struct Warning {
    pub variant: WarningVariant,
    pub location: Position,
}

#[derive(Debug)]
pub enum WarningVariant {
    DuplicateScopedVariable(String),
    RedundantTrue,
}

impl DisplaySnippet for &Warning {
    fn title(&self) -> Annotation {
        Annotation {
            label: Some(self.variant.title()),
            id: Some(self.variant.code()),
            annotation_type: AnnotationType::Warning,
        }
    }
    fn footer(&self) -> Vec<Annotation> {
        vec![]
    }
    fn slice(&self) -> Vec<Slice> {
        vec![]
    }
}

impl WarningVariant {
    pub fn title(&self) -> &str {
        match self {
            WarningVariant::DuplicateScopedVariable(_) => {
                "Scoped variable was declared multiple times"
            }
            RedundantTrue => "'true' in this position does nothing",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Self::DuplicateScopedVariable(_) => "W001",
            Self::RedundantTrue => "W002",
        }
    }
}
