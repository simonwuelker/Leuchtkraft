use pest::iterators::Pair;
use crate::parser::Rule;
use crate::error::*;

#[derive(Debug)]
pub enum Atom {
    Boolean(bool),
    Predicate {
        name: String,
        args: Vec<String>,
    },
}

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Question {
        body: Atom,
    },
    HornClause {
        head: Atom,
        body: Vec<Atom>,
    },
}

impl AstNode {
    pub fn from_tree(pair: Pair<Rule>) -> Result<AstNode, Error> {
        let node = match pair.as_rule() {
            Rule::Question => {
                let child = pair.into_inner().next().unwrap();
                Self::Question {
                    body: Atom::from_pair(child),
                }
            },
            Rule::Program => {
                let mut childnodes = vec![];
                for child in pair.into_inner() {
                    if let Rule::EOI = child.as_rule() {
                        break;
                    }
                    childnodes.push(Self::from_tree(child)?);
                }
                Self::Program(childnodes)
            },
            Rule::Rule => {
                let mut children: Vec<Atom> = pair.into_inner()
                    .map(|p| Atom::from_pair(p))
                    .collect();
                let head = children.pop().unwrap();

                Self::HornClause {
                    head: head,
                    body: children,
                }
            },
            Rule::Predicate => {
                let mut idents = pair.into_inner().map(|p| p.to_string());
                Self::HornClause {
                    head: Atom::Predicate {
                        name: idents.next().unwrap(),
                        args: idents.collect(),
                    },
                    body: vec![Atom::Boolean(true)],
                }
            },
            _ => unimplemented!("\"{:?}\" pair", pair.as_rule()),
        };
        Ok(node)
    }

}

impl Atom {
    fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(
            std::mem::discriminant(&Rule::Atom), 
            std::mem::discriminant(&pair.as_rule())
        );

        let child = pair.into_inner().next().unwrap();
        match child.as_rule() {
            Rule::Predicate => {
                let mut idents = child.into_inner().map(|p| p.to_string());
                Self::Predicate {
                    name: idents.next().unwrap(),
                    args: idents.collect(),
                }
            },
            Rule::Boolean => {
                match child.as_str() {
                    "true" => Self::Boolean(true),
                    "false" => Self::Boolean(false),
                    _ => unreachable!(),
                }
            },
            _ => unreachable!("Converting non-atom value to atom"),
        }
    }
}
