use pest::Parser;

use crate::ast::ast_from_tree;
use crate::error::Error;
use crate::logic::clause::Clause;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LEParser;

pub fn parse_str(unparsed_file: &str) -> Result<Vec<Clause>, Error> {
    log::info!(target: "Parser", "Calling pest");
    let tree = LEParser::parse(Rule::Program, &unparsed_file)?
        .next()
        .unwrap();
    log::info!(target: "Parser", "Parsing AST");
    Ok(ast_from_tree(tree))
}
