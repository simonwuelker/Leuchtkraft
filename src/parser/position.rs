#[derive(Clone, Copy, Debug)]
/// Describes either a single position in the code or a span
pub enum Position {
    Pos(usize),
    Span(usize, usize),
}

pub struct Positioned<T> {
    content: T,
    pos: Position,
}

impl<T> Positioned<T> {
    pub fn new(content: T, pos: Position) -> Self {
        Self {
            content: content,
            pos: pos,
        }
    }

    pub fn as_inner(&self) -> &T {
        &self.content
    }

    pub fn into_inner(self) -> T {
        self.content
    }

    pub fn span(&self) -> Position {
        self.pos
    }

    pub fn map<C>(&self, new_content: C) -> Positioned<C> {
        Positioned::new(new_content, self.pos)
    }
}
