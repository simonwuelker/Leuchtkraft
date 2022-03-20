use std::collections::HashMap;
use crate::logic::{atom::Atom, clause::Clause, Ident};
use crate::error::Error;

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
            // If the clause contains unknown values, its a question
            if clause.contains(&Atom::Unknown) {
                let mut i = Interpretation::new();

            } else {
                self.known_clauses.push(clause);
            }
        }
        Ok(())
    }

    fn resolve(&self, atom: &Atom) -> Option<bool> {
        match atom {
            Atom::Boolean(val) => Some(*val),
            Atom::Predicate{ ... } => {
                let next = self.try_find_match(atom);
                println!("{:?}", next);
                None
            }
            Unknown => {
                None
            }
        }
    }

    fn resolve_all(&self, atoms: &[Atom]) -> Option<bool> {
        for atom in atoms {
            match self.resolve(atom) {
                Some(false) => return Some(false),
                None => return None,
                _ => {}
            };
        }
        Some(true)
    }

    fn try_find_match(&self, atom: &Atom) -> Option<Clause> {
        for clause in &self.known_clauses {
            let matched = clause.try_match(atom);
            if matched.is_some() {
                return matched;
            }
        }
        None
    }
}
