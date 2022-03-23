use crate::error::Error;
use crate::logic::{
    atom::{Atom, Predicate},
    clause::Clause,
};
use std::collections::HashMap;

type Interpretation = HashMap<Atom, bool>;

pub struct Interpreter {
    known_clauses: Vec<Clause>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            known_clauses: vec![],
        }
    }

    pub fn traverse(&mut self, clauses: Vec<Clause>) -> Result<(), Error> {
        for clause in clauses {
            if clause.is_question() {
                println!("==> {}", self.resolve_clause(&clause));
            } else {
                self.known_clauses.push(clause);
            }
        }
        Ok(())
    }

    fn resolve_clause(&self, clause: &Clause) -> bool {
        log::trace!(target: "Interpreter", "Resolving {:?}", clause);
        let mut i = Interpretation::new();
        let mut cloned = clause.clone();

        // If A => B => C
        // and we try to resolve C, only A matters

        // If A => B
        // and
        // C => B => D
        // and we try to resolve D, both A and B matter
        // for mut atom in &cloned.lhs {
        //     if let Atom::Predicate(predicate) = atom {
        //         atom = self.resolve_predicate(&predicate);
        //     }
        // }
        todo!();
    }

    fn resolve_predicate(&self, predicate: &Predicate) -> Atom {
        let next = self.try_find_match(predicate);
        println!("{:?}", next);
        todo!();
    }

    /// Resolve an atom of any type into either Atom::Boolean or Atom::Unknown
    fn resolve(&self, atom: &Atom) -> Atom {
        match atom {
            Atom::Boolean(_) => atom.clone(),
            Atom::Predicate(predicate) => self.resolve_predicate(predicate),
            Atom::Unknown(_) => atom.clone(),
        }
    }

    fn try_find_match(&self, predicate: &Predicate) -> Option<Clause> {
        for clause in &self.known_clauses {
            let matched = clause.try_match(predicate);
            if matched.is_some() {
                return matched;
            }
        }
        None
    }
}
