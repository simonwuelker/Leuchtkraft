use super::{Atom, Clause, Var};
use crate::interpreter::Ident;

pub struct LogicEngine {
    known_clauses: Vec<Clause<Var>>,
}

impl Default for LogicEngine {
    fn default() -> Self {
        Self {
            known_clauses: vec![],
        }
    }
}

impl LogicEngine {
    pub fn add(&mut self, clause: Clause<Var>) {
        self.known_clauses.push(clause);
    }

    pub fn resolve(&self, clause: Clause<Ident>) {
        println!("resolving {:?}", clause);
        println!("know {} clauses", self.known_clauses.len());

        // Find all clauses "related" to the clause in question
        let mut related = vec![];
        self.find_related_clauses(clause, &mut related);
        println!("found {} related clauses", related.len());
        for c in related {
            println!("{:?}", c);
        }
    }

    fn find_related_clauses(&self, from: Clause<Ident>, already_found: &mut Vec<Clause<Var>>) {
        for atom in from.0.iter().flatten() {
            if let Atom::Predicate(ident, args) = atom {
                for known_clause in self.known_clauses {
                    known_clause.matches((&ident, args), already_found);
                }
            }
        }
    }
}
