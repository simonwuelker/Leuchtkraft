use super::atom::{Atom, Clause, Var};

// /// A clause with RefCells for variables,
// /// so changes in any clause affect the entire knowledge base
// type InternalClause = Vec<Vec<RefCell

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
    pub fn add(&mut self, clause: Clause) {
        self.known_clauses.push(clause);
    }

    pub fn resolve(&self, clause: Clause) {
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

    fn find_related_clauses(&self, from: Clause, already_found: &mut Vec<Clause>) {
        // for and_chain in from.0 {
        //     for atom in and_chain {
        //         let newly_found: Vec<Clause> = self
        //             .known_clauses
        //             .iter()
        //             .filter(|clause| !already_found.contains(clause))
        //             .filter(|clause| self.match_clause(&atom, clause))
        //             .map(|ref_clause| ref_clause.to_owned())
        //             .collect();
        //         already_found.extend(newly_found);
        //     }
        // }
    }

    /// Match an atom and a clause. The result will be true either if
    /// the atom is directly part of the clause or the atom is matched
    /// by a freed variable in the clause
    fn match_clause(&self, to_match: &Atom, clause: &Clause) -> bool {
        clause.0.iter().any(|and_chain| {
            // match directly
            if and_chain.contains(&to_match) {
                return true;
            }

            // match free args (only possible if the target is a predicate
            if let Atom::Predicate(ident, args) = to_match {
                and_chain.iter().any(|atom| {
                    if let Atom::Predicate(ident_2, args_2) = &atom {
                        if ident == ident_2 && args.len() == args_2.len() {
                            // The two predicates match if all arguments are either
                            // the same or match freed args at the same position
                            !args.iter().zip(args_2).any(|(arg, arg_2)| {
                                if arg == arg_2 {
                                    return true;
                                }
                                if let Var::Anonymous(_) = arg_2 {
                                    return true;
                                }
                                return false;
                            })
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        })
    }
}
