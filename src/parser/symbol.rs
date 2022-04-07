//! Higher-level tokens

use super::span::Spanned;

pub enum Line<'a> {
    Forall(Vec<Spanned<&'a str>>),
    /// (is_indented, is_question, and_chains)
    Rule(bool, bool, Vec<Vec<Spanned<Atom<'a>>>>),
}

#[derive(PartialEq)]
pub enum Atom<'a> {
    True,
    False,
    Predicate(&'a str, Vec<&'a str>),
    Unknown(&'a str),
}

impl Atom<'_> {
    pub fn is_literal(&self) -> bool {
        match self {
            Atom::True | Atom::False => true,
            _ => false,
        }
    }
}
