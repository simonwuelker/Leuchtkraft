use annotate_snippets::{
    display_list::FormatOptions,
    snippet::{Annotation, Slice, Snippet, SourceAnnotation},
};

/// Defines a interface for code annotations shown to the user
pub trait DisplaySnippet {
    fn title(&self) -> Annotation;
    fn footer(&self) -> Vec<Annotation>;
    fn source_annotations(&self) -> Vec<SourceAnnotation>;
    fn as_snippet<'a>(
        &'a self,
        src: &'a str,
        lineno: usize,
        origin: Option<&'a str>,
    ) -> Snippet<'a> {
        Snippet {
            title: Some(self.title()),
            footer: self.footer(),
            slices: vec![Slice {
                source: src,
                line_start: lineno,
                origin: origin,
                annotations: self.source_annotations(),
                fold: true,
            }],
            opt: FormatOptions {
                color: true,
                ..Default::default()
            },
        }
    }
}
