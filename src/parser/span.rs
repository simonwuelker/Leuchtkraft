#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

pub struct Spanned<T> {
    content: T,
    span: Span,
}

impl Span {
    /// Get a span for a single character
    pub fn position(position: usize) -> Self {
        Self(position, position + 1)
    }
}

impl<T> Spanned<T> {
    pub fn new(content: T, span: Span) -> Self {
        Self {
            content: content,
            span: span,
        }
    }

    pub fn as_inner(&self) -> &T {
        &self.content
    }

    pub fn into_inner(self) -> T {
        self.content
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn map<C>(&self, new_content: C) -> Spanned<C> {
        Spanned::new(new_content, self.span)
    }
}

impl Span {
    pub fn as_range(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl From<usize> for Span {
    fn from(from: usize) -> Self {
        Self(from, from + 1)
    }
}
