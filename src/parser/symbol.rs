//! Higher-level tokens

use crate::interpreter::Ident;

pub enum Line {
    Forall(Vec<Ident>),
    Rule(bool, Vec<Vec<Atom>>),
    Empty,
}

pub enum Atom {
    True,
    False,
    Predicate(Ident, Vec<Ident>),
    Unknown(Ident),
}
