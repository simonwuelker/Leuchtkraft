use crate::ast::AstNode;
use crate::logic::{atom::Atom, clause::HornClause};
use crate::error::Error;

pub struct Interpreter {
    known_clauses: Vec<HornClause>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            known_clauses: vec![],
        }
    }

    pub fn traverse(&mut self, root: AstNode) -> Result<(), Error> {
        match root {
            AstNode::Program(children) => {
                for child in children {
                    self.traverse(child)?;
                }
            },
            AstNode::HornClause(clause) => self.known_clauses.push(clause),
            // AstNode::Question(atom) => {
            //     match self.resolve(&atom) {
            //         Some(answer) => println!("The answer is: {}", answer),
            //         None => println!("Answer cannot be determined"),
            //     }
            // },
            _ => unimplemented!("{:?}", root),
        }
        Ok(())
    }

    fn resolve(&self, atom: &Atom) -> Option<bool> {
        match atom {
            Atom::Boolean(val) => Some(*val),
            Atom::Predicate{ name, args } => {
                // Test if the statement is **true**
                for clause in &self.known_clauses {
                    if atom ==&clause.head {
                        if let Some(true) = self.resolve_all(&clause.body) {
                            return Some(true);
                        }
                    }
                }
                None
                // Test if the statement is **false**
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
}
