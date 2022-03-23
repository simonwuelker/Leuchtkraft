use crate::logic::clause::Clause;

pub struct LogicEngine {
    known_clauses: Vec<Clause>,
}

impl Default for LogicEngine {
    fn default() -> Self {
        Self {
            known_clauses: vec![],
        }
    }
}

impl LogicEngine {
    pub fn add_clause(&mut self, clause: Clause) {
        self.known_clauses.push(clause);
    }

    pub fn resolve(&self, clause: Clause) {
        println!("resolving {:?}", clause);
    }
}
