use crate::logic::{
    atom::{Atom, Var},
    clause::Clause,
};
use crate::parser::Rule;
use pest::iterators::Pair;
use std::mem::discriminant;

pub fn ast_from_tree(pair: Pair<Rule>) -> Vec<Clause> {
    log::trace!(target: "AST", "Parsing {:?} as {:?}", pair.as_str(), pair.as_rule());

    match pair.as_rule() {
        Rule::Program => {
            let mut stmts = vec![];
            for block in pair.into_inner() {
                if let Rule::EOI = block.as_rule() {
                    break;
                }
                stmts.extend(ast_from_tree(block));
            }
            stmts
        }
        Rule::Scopeblock => {
            let (idents_nodes, stmt_nodes): (Vec<Pair<Rule>>, Vec<Pair<Rule>>) = pair
                .into_inner()
                .partition(|e| discriminant(&Rule::Ident) == discriminant(&e.as_rule()));

            // Sanity checks
            assert!(stmt_nodes
                .iter()
                .all(|val| discriminant(&val.as_rule()) == discriminant(&Rule::Rule)));

            let mut stmts = vec![];
            for node in stmt_nodes {
                stmts.extend(ast_from_tree(node));
            }
            for ident_node in idents_nodes {
                let ident = ident_node.as_str().to_string();
                for stmt in &mut stmts {
                    stmt.replace(&Var::Fixed(ident.clone()), &Var::Anonymous(ident.clone()));
                }
            }
            stmts
        }
        Rule::Rule => {
            let mut children: Vec<Pair<Rule>> = pair.into_inner().collect();

            let mut operands = vec![];
            let mut implications_at = vec![];

            children.iter().enumerate().for_each(|(ix, node)| {
                match node.as_rule() {
                    Rule::Implication => implications_at.push(ix),
                    Rule::Atom => operands.push(Atom::from_pair(node.clone())),
                    _ => unreachable!(),
                }
            });

            vec![Clause {
                operands: operands,
                implications_at: implications_at,
            }]
        }
        _ => unimplemented!("\"{:?}\" pair", pair.as_rule()),
    }
}
