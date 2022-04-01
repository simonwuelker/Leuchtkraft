/// A module for generating pretty user-facing messages.
/// I have looked at both [annotate-snippets](https://github.com/rust-lang/annotate-snippets-rs) and
/// [codespan-reporting](https://github.com/brendanzab/codespan)
/// and determined that i hate them both to an about equal degree.
mod annotation_type;
mod diagnostic;
mod render;

pub use annotation_type::AnnotationType;
pub use diagnostic::{Annotation, Diagnostic};
pub use render::DisplayDiagnostic;
