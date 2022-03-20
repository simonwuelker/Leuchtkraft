use crate::logic::atom::Atom;

#[derive(Debug)]
pub struct HornClause {
    pub head: Atom,
    pub body: Vec<Atom>,
}
