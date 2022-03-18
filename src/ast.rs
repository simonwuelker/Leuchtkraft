use pest::iterators::Pair;
use crate::parser::Rule;

pub enum UnaryOperator {
    Plus,
    Minus,
}

pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub enum AstNode {
    Program(Vec<AstNode>),
    Print(Box<AstNode>),
    Integer(i32),
    Variable(String),
    UnaryExpression {
        op: UnaryOperator,
        expr: Box<AstNode>
    },
    BinaryExpression {
        lhs: Box<AstNode>,
        op: BinaryOperator,
        rhs: Box<AstNode>,
    },
    Assignment {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    }
}

impl AstNode {
    pub fn from_tree(pair: Pair<Rule>) -> AstNode {
        match pair.as_rule() {
            Rule::Assignment => {
                let mut children = pair.into_inner();
                Self::Assignment {
                    lhs: Box::new(Self::from_tree(children.next().unwrap())),
                    rhs: Box::new(Self::from_tree(children.next().unwrap())),
                }
            },
            Rule::Word => Self::Variable(pair.as_str().to_string()),
            Rule::Integer => Self::Integer(pair.as_str().parse().unwrap()),
            Rule::Print => {
                let expr = pair.into_inner().next().unwrap();
                Self::Print(Box::new(Self::from_tree(expr)))
            },
            Rule::Program => {
                let mut childnodes = vec![];
                for child in pair.into_inner() {
                    if let Rule::EOI = child.as_rule() {
                        break;
                    }
                    childnodes.push(Self::from_tree(child));
                }
                Self::Program(childnodes)
            },
            Rule::BinaryExpression => {
                let mut children = pair.into_inner();
                Self::BinaryExpression {
                    lhs: Box::new(Self::from_tree(children.next().unwrap())),
                    op: BinaryOperator::from_pair(children.next().unwrap()),
                    rhs: Box::new(Self::from_tree(children.next().unwrap())),
                }
            },
            _ => todo!("{:?} pair", pair.as_rule()),
        }
    }

}

impl UnaryOperator {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::Plus => Self::Plus,
            Rule::Minus => Self::Minus,
            _ => unreachable!("Expected unary operator, got {:?}", pair.as_rule()),
        }
    }
}

impl BinaryOperator {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::Plus => Self::Plus,
            Rule::Minus => Self::Minus,
            Rule::Multiply => Self::Multiply,
            Rule::Divide => Self::Divide,
            _ => unreachable!("Expected binary operator, got {:?}", pair.as_rule()),
        }
    }
}
