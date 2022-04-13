use super::{Atom, Clause, Ident, ImplicationGraph, Var};

#[derive(Debug)]
pub enum UnknownValue {
    True,
    False,
    Either,
    Neither,
}

#[derive(Debug)]
/// Possible results after pruning the implication graph once
pub enum Resolution {
    /// The resolution process is finished and possible values
    /// for the unknown have been determined
    Done(UnknownValue),
    /// A logical contradiction was encountered during resolution.
    /// Resolution cannot continue.
    Contradiction,
    /// Progress has been made but the unknown value cannot be determined yet
    Progressed,
    /// No progress has been made
    Halted,
}

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

    pub fn resolve(&self, clause: Clause<Ident>) -> Vec<UnknownValue> {
        let mut implication_graph = ImplicationGraph::new();
        let unknowns = clause.unknowns();
        implication_graph.add_clause(clause.into_varclause());

        let mut resolved_atoms = 0;
        while resolved_atoms != implication_graph.atoms.len() {
            let mut to_add = vec![];
            match &implication_graph.atoms[resolved_atoms] {
                Atom::Boolean(_) | Atom::Unknown(_) => {} // don't match
                Atom::Predicate(ident, args) => {
                    // Find all clauses related to that specific predicate
                    for known_clause in &self.known_clauses {
                        to_add.extend(known_clause.matches((ident, args)));
                    }
                }
            }
            for clause in to_add {
                // println!("matched {:?} with {:?}", implication_graph.atoms[resolved_atoms], clause);
                implication_graph.add_clause(clause);
            }
            resolved_atoms += 1;
        }

        let mut resolvents = vec![];
        for unknown in unknowns {
            let unknown_index = implication_graph.find_or_insert_atom(Atom::Unknown(unknown));
            let mut resolved_to = None;
            while let None = resolved_to {
                resolved_to = match implication_graph.resolution_step(unknown_index) {
                    Resolution::Done(val) => Some(val),
                    Resolution::Contradiction => Some(UnknownValue::Neither),
                    Resolution::Halted => Some(UnknownValue::Either),
                    Resolution::Progressed => None,
                };
            }
            resolvents.push(resolved_to.unwrap())
        }
        resolvents
    }
}
