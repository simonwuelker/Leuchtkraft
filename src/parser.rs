use pest::Parser;

use crate::error::Error;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LEParser;

/// Returns None if parsing was successful, but the line was empty
/// and Err if parsing was not successful
pub fn parse_line(line: &str) -> Result<Pair<Rule>, Error> {
    let stmt = LEParser::parse(Rule::Line, &line)?.next().unwrap(); // can never fail

    Ok(stmt)
}
