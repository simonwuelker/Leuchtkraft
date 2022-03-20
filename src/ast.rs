use pest::iterators::Pair;
use crate::parser::Rule;
use crate::error::*;
use crate::logic::{atom::Atom, clause::HornClause, Ident};
use std::mem::discriminant;

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    ScopeNode(Vec<Ident>, Vec<AstNode>),
    HornClause(HornClause),
    Question(HornClause),
}

impl AstNode {
    pub fn from_tree(pair: Pair<Rule>) -> Result<AstNode, Error> {
        log::info!(target: "AST", "Parsing {:?} as {:?}", pair.as_str(), pair.as_rule());

        let node = match pair.as_rule() {
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
            Rule::Scopeblock => {
                let (idents, stmts): (Vec<Pair<Rule>>, Vec<Pair<Rule>>) = pair.into_inner()
                    .partition(|e| discriminant(&Rule::Ident) == discriminant(&e.as_rule()));

                Self::ScopeNode(
                    idents.iter().map(|i| i.as_str().to_string()).collect(),
                    stmts.iter().map(|s| AstNode::from_tree(s.clone()).unwrap()).collect(),
                )
            },
            Rule::Rule => {
                let mut children: Vec<Pair<Rule>> = pair.into_inner()
                    .collect();
                // If the rule contains unknowns, its a question
                let num_unknowns = children
                    .iter()
                    .filter(|p| discriminant(p) == discriminant(&Rule::Unknown))
                    .count();

                match num_unknowns {
                    0 => {
                        let atoms = children.iter().map(|p| Atom::from_pair(p)).collect();
                        let head = atoms.pop().unwrap();

                        Self::HornClause(HornClause {
                            head: head,
                            body: atoms,
                        })
                    },
                    1 => {
                        todo!()
                    },
                    _ => {
                        let err = "Clause contains more than two unknown variables".to_string();
                        log::error!("{}", err);
                        return Err(Error::from_pair(pair.as_span(), err));
                    },
                }

            },
            _ => unimplemented!("\"{:?}\" pair", pair.as_rule()),
        };
        Ok(node)
    }

}
