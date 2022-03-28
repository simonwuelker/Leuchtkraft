use annotate_snippets::{
    display_list::FormatOptions,
    snippet::{Annotation, Slice, Snippet},
};

#[derive(Debug)]
/// Describes either a single position in the code or a span
pub enum InputLocation {
    Pos(usize),
    Span((usize, usize)),
}

/// Defines a interface for code annotations shown to the user
pub trait DisplaySnippet {
    fn title(&self) -> Annotation;
    fn footer(&self) -> Vec<Annotation>;
    fn slice(&self) -> Vec<Slice>;
    fn as_snippet(&self) -> Snippet {
        Snippet {
            title: Some(self.title()),
            footer: self.footer(),
            slices: self.slice(),
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        }
    }
}
