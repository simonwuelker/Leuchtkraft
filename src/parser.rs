use pest::Parser;

use crate::ast::AstNode;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LEParser;

pub fn parse_str(unparsed_file: &str) -> Result<AstNode, pest::error::Error<Rule>> {
    let tree = LEParser::parse(Rule::Program, &unparsed_file)?.next().unwrap();
    Ok(AstNode::from_tree(tree))
}
