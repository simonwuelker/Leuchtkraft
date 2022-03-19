use std::collections::HashMap;
use crate::ast::AstNode;
use crate::fuzzy_logic::FuzzyBoolean;

pub struct Interpreter {
    predicates: HashMap<String, AstNode>,
    /// Map of variables to their assigned predicates
    primitives: HashMap<String, Vec<String>>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            predicates: HashMap::new(),
            primitives: HashMap::new(),
        }
    }

    pub fn traverse(&mut self, root: AstNode) -> Option<FuzzyBoolean> {
        match root {
            AstNode::Program(children) => {
                for child in children {
                    self.traverse(child);
                }
                None
            },
            _ => unimplemented!("{:?}", root),

        }
    }
}
