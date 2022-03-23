use crate::logic::atom::{Atom, Predicate, Var};
use std::cell::Cell;

#[derive(Debug)]
/// A logical chain of implications and "and"s
pub struct Clause {
    /// The operands within the clause
    pub operands: Vec<Atom>,
    /// The indices at which implications are placed
    /// If no implication is placed between two operands, the 
    /// connective is assumed to be "and"
    pub implications_at: Vec<usize>,
}

impl Clause {
    pub fn replace(&mut self, to_replace: &Var, replace_with: &Var) {
        for atom in &mut self.operands {
            if let Atom::Predicate(predicate) = atom {
                predicate.replace(to_replace, replace_with);
            }
        }
    }

    pub fn find(&self, atom: &Atom) -> Option<usize> {
        let mut ix = 0;
        for operand in &self.operands {
            if operand == atom {
                return Some(ix);
            }
            ix += 1;
        }
        None
    }

    pub fn is_question(&self) -> bool {
        self.operands.iter().any(|op| {
            match op {
                Atom::Unknown(_) => true,
                _ => false,
            }
        })
    }

    pub fn try_match(&self, predicate: &Predicate) -> Option<Self> {
        if self.find(&Atom::Predicate(predicate.clone())).is_some() {
            let mut cloned = self.clone();
        }
        None
    }
}
