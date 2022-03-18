use std::collections::HashMap;
use crate::ast::{AstNode, UnaryOperator, BinaryOperator};

pub struct Interpreter {
    state: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn traverse(&mut self, root: AstNode) -> Option<i32> {
        match root {
            AstNode::Program(children) => {
                for child in children {
                    self.traverse(child);
                }
                None
            },
            AstNode::Integer(num) => Some(num),
            AstNode::Variable(name) => Some(*self.state.get(&name).expect("Undefined")),
            AstNode::UnaryExpression{op, expr} => {
                let e = self.traverse(*expr).unwrap();
                match op {
                    UnaryOperator::Plus => Some(e),
                    UnaryOperator::Minus => Some(-1 * e),
                }
            },
            AstNode::BinaryExpression{lhs, op, rhs} => {
                let lh = self.traverse(*lhs).unwrap();
                let rh = self.traverse(*rhs).unwrap();
                match op {
                    BinaryOperator::Plus => Some(lh + rh),
                    BinaryOperator::Minus => Some(lh - rh),
                    BinaryOperator::Multiply => Some(lh * rh),
                    BinaryOperator::Divide => Some(lh / rh),
                }
            }
            AstNode::Assignment{lhs, rhs} => {
                if let AstNode::Variable(name) = *lhs {
                    let res = self.traverse(*rhs).unwrap();
                    self.state.insert(name, res);
                    None
                } else {
                    panic!("Expected value on lhs of assigment");
                }
            },
            AstNode::Print(expr) => {
                println!("{}", self.traverse(*expr).unwrap());
                None
            }

        }
    }
}
