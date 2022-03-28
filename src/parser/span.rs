pub type Span = (usize, usize);

pub struct Spanned<T> {
    content: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(content: T, span: Span) -> Self {
        Self {
            content: content,
            span: span
        }
    }

    pub fn as_inner(&self) -> &T {
        &self.content
    }

    pub fn as_inner_mut(&self) -> &mut T {
        &mut self.content
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
