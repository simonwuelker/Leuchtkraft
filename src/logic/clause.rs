use super::{Atom, Ident, Var};
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Clause<T>(pub Vec<Vec<Atom<T>>>);

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

impl Clause<Ident> {
    pub fn into_varclause(self) -> Clause<Var> {
        let and_chains = self
            .0
            .into_iter()
            .map(|block| block.into_iter().map(|atom| atom.into()).collect())
            .collect();
        Clause(and_chains)
    }
}

impl Clause<Var> {
    /// Pin all occurences of an anonymous var to a fixed var
    pub fn pin(&self, to_pin: Ident, pin_to: Ident) -> Self {
        let mut cloned = self.clone();
        for atom in &mut cloned.0.iter_mut().flatten() {
            atom.pin_var(to_pin, pin_to);
        }
        cloned
    }

    /// Return a list of clauses representing the different possibilities how
    /// the given predicate can be formatted into the clause
    ///
    /// # Example
    /// `foo(a, A) => bar(A, B)` (`A` and `B` are free vars) formatted with `foo(a, x)`
    /// produces `foo(a, x) => bar(x, B)`
    pub fn matches(&self, predicate: (&Ident, &Vec<Var>), pin_variants: &mut Vec<Self>) {
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

impl<T: fmt::Debug> fmt::Debug for Clause<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, and_chain) in self.0.iter().enumerate() {
            for (atom_index, atom) in and_chain.iter().enumerate() {
                write!(f, "{:?}", atom)?;
                if atom_index != and_chain.len() - 1 {
                    write!(f, " and ")?;
                }
            }

            if index != self.0.len() - 1 {
                write!(f, " => ")?;
            }
        }
        Ok(())
    }
}
