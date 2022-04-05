//! Higher-level tokens

use crate::interpreter::Ident;

pub enum Line {
    Forall(Vec<Ident>),
    Rule(bool, Vec<Vec<Atom>>),
}

#[derive(PartialEq)]
pub enum Atom {
    True,
    False,
    Predicate(Ident, Vec<Ident>),
    Unknown(Ident),
}

impl Atom {
    pub fn is_literal(&self) -> bool {
        match self {
            Atom::True | Atom::False => true,
            _ => false,
        }
    }
}
