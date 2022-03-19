use pest::Parser;

use crate::error::Error;
use crate::ast::AstNode;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LEParser;

pub fn parse_str(unparsed_file: &str) -> Result<AstNode, Error> {
    let tree = LEParser::parse(Rule::Program, &unparsed_file)?.next().unwrap();
    AstNode::from_tree(tree)
}
