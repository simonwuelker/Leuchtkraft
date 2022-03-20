use crate::logic::atom::{Atom, Var};

#[derive(Debug)]
/// A logical implication of the form "A and B imply C and D" 
pub struct Clause {
    /// The operands on the left hand side of the implication
    pub lhs: Vec<Atom>,
    /// The operands on the right hand side of the implication
    pub rhs: Vec<Atom>,
    pub implication: Implication,
}

#[derive(Debug)]
pub enum Implication {
    Unidirectional,
    Bidirectional,
}

impl Clause {
    pub fn replace(&mut self, to_replace: &Var, replace_with: &Var) {
        for atom in &mut self.lhs {
            atom.replace(to_replace, replace_with);
        }
        for atom in &mut self.rhs {
            atom.replace(to_replace, replace_with);
        }
    }

    pub fn contains(&self, atom: &Atom) -> bool {
        for operand in &self.lhs {
            if operand == atom {
                return true;
            }
        }
        for operand in &self.rhs {
            if operand == atom {
                return true;
            }
        }
        false
    }

    pub fn try_match(&self, atom: &Atom) -> Option<Self> {
        if self.contains(atom) {
            let mut cloned = self.clone();
        }
        None
    }
}
