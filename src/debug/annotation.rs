use annotate_snippets::{
    display_list::FormatOptions,
    snippet::{Annotation, Slice, Snippet},
};

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
