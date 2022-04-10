use super::Graph;
use super::{Atom, Clause, Ident, Var};

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
        let mut implication_graph: Graph<Atom<Var>> = Graph::new();

        // Find all clauses "related" to the clause in question
        let mut related: Vec<Clause<Var>> = vec![clause.into_varclause()];
        let mut previously_found = 0;

        while previously_found != related.len() {
            // List of all related clauses, including once we've already seen
            let mut all_related = vec![];
            for clause in &related[previously_found..] {
                self.find_related_clauses(clause, &mut all_related);
            }
            previously_found = related.len();

            let deduped: Vec<Clause<Var>> = all_related
                .into_iter()
                .filter(|c| !related.contains(&c))
                .collect();
            related.extend(deduped);
            // break;
        }
        println!("found {} related clauses", related.len());
        for c in related {
            println!("{:?}", c);
        }
    }

    fn find_related_clauses(&self, from: &Clause<Var>, already_found: &mut Vec<Clause<Var>>) {
        for atom in from.0.iter().flatten() {
            if let Atom::Predicate(ident, args) = atom {
                for known_clause in &self.known_clauses {
                    known_clause.matches((&ident, args), already_found);
                }
            }
        }
    }
}
