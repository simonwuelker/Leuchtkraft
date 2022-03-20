use pest::iterators::Pair;
use crate::parser::Rule;
use crate::logic::{atom::{Atom, Var}, clause::{Clause, Implication}};
use std::mem::discriminant;

pub fn ast_from_tree(pair: Pair<Rule>) -> Vec<Clause> {
    log::info!(target: "AST", "Parsing {:?} as {:?}", pair.as_str(), pair.as_rule());

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
        },
        Rule::Scopeblock => {
            let (idents_nodes, stmt_nodes): (Vec<Pair<Rule>>, Vec<Pair<Rule>>) = pair.into_inner()
                .partition(|e| discriminant(&Rule::Ident) == discriminant(&e.as_rule()));

            // Sanity checks
            assert!(stmt_nodes.iter().all(|val| discriminant(&val.as_rule()) == discriminant(&Rule::Rule)));

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
        },
        Rule::Rule => {
            let mut lhs_nodes: Vec<Pair<Rule>> = pair.into_inner()
                .collect();

            let split_ix = lhs_nodes.iter()
                .position(|child| discriminant(&child.as_rule()) == discriminant(&Rule::Implication))
                .unwrap();
            let rhs_nodes = lhs_nodes.split_off(split_ix + 1);
            let implication_node = lhs_nodes.pop().unwrap();

            // Sanity checks
            assert!(lhs_nodes.iter().all(|val| discriminant(&val.as_rule()) == discriminant(&Rule::Atom)));
            assert!(rhs_nodes.iter().all(|val| discriminant(&val.as_rule()) == discriminant(&Rule::Atom)));
            assert!(discriminant(&implication_node.as_rule()) == discriminant(&Rule::Implication));

            let lhs = lhs_nodes.iter().map(|node| Atom::from_pair(node.clone())).collect();
            let rhs = rhs_nodes.iter().map(|node| Atom::from_pair(node.clone())).collect();

            match implication_node.as_str() {
                "=>" => {
                    vec![
                        Clause {
                            lhs: lhs,
                            rhs: rhs,
                            implication: Implication::Unidirectional,
                        }
                    ]
                },
                "<=" => {
                    vec![
                        Clause {
                            lhs: rhs, // notice how lhs and rhs are swapped
                            rhs: lhs,
                            implication: Implication::Unidirectional,
                        }
                    ]
                },
                "<=>" => {
                    vec![
                        Clause {
                            lhs: rhs,
                            rhs: lhs,
                            implication: Implication::Bidirectional,
                        },
                    ]
                },
                _ => unreachable!(),
            }
        },
        _ => unimplemented!("\"{:?}\" pair", pair.as_rule()),
    }
}
