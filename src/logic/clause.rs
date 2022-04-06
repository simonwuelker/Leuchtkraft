use super::{Atom, Var};
use crate::interpreter::Ident;

#[derive(Debug, Clone, PartialEq)]
pub struct Clause<T: PartialEq>(pub Vec<Vec<Atom<T>>>);

impl<T: PartialEq> Clause<T> {
    pub fn new(and_chains: Vec<Vec<Atom<T>>>) -> Self {
        Self(and_chains)
    }

    pub fn contains(&self, search_for: &Atom<T>) -> bool {
        self.0
            .iter()
            .any(|and_chain| and_chain.iter().any(|atom| atom == search_for))
    }
}

impl Clause<Var> {
    /// Pin all occurences of an anonymous var to a fixed var
    pub fn pin(&self, to_pin: Ident, pin_to: Ident) -> Self {
        let mut cloned = self.clone();
        for and_chain in &mut cloned.0 {
            for atom in and_chain {
                atom.pin_var(to_pin, pin_to);
            }
        }
        cloned
    }

    /// Return a list of clauses representing the different possibilities how
    /// the given predicate can be formatted into the clause
    pub fn matches(&self, predicate: (&Ident, &Vec<Ident>), pin_variants: &mut Vec<Self>) {
        for atom in self.0.iter().flatten() {
            match atom.match_predicate(predicate) {
                Some(argmap) => {
                    let mut cloned = self.clone();
                    for (to_pin, pin_to) in argmap {
                        cloned.pin(to_pin, pin_to);
                    }
                    if !pin_variants.contains(&cloned) {
                        pin_variants.push(cloned);
                    }
                }
                None => {}
            }
        }
    }
}
