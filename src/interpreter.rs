use crate::debug::warning::Warning;
use crate::logic::logic_engine::LogicEngine;
use crate::logic::{atom::Atom, clause::Clause};
use crate::parser::error::TokenNotFound;
use crate::parser::parser::Parser;

/// Idents are hashed variable names
pub type Ident = u64;

pub struct Response {
    text: Option<String>,
    warnings: Vec<Warning>,
}

struct State {
    inside_scopeblock: bool,
    anon_vars: Vec<Ident>,
}

pub struct Interpreter {
    known_clauses: Vec<Clause>,
    state: State,
    logic: LogicEngine,
}

impl Default for State {
    fn default() -> Self {
        Self {
            inside_scopeblock: false,
            anon_vars: vec![],
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            known_clauses: vec![],
            state: State::default(),
            logic: LogicEngine::default(),
        }
    }

    pub fn execute(&mut self, line: &str) -> Result<Response, TokenNotFound> {
        let expected_indentation = Some(0);
        let mut warnings = vec![];

        // Parse the line
        let mut parser = Parser::new(line);
        {
            parser.line(&mut warnings)?;
        }

        let mut response = Response::empty();
        response.set_warnings(warnings);
        Ok(response)
    }
}

impl Response {
    pub fn empty() -> Self {
        Self {
            text: None,
            warnings: vec![],
        }
    }

    pub fn warnings<'a>(&'a self) -> &'a [Warning] {
        &self.warnings
    }

    pub fn text<'a>(&'a self) -> Option<&'a String> {
        self.text.as_ref()
    }

    pub fn set_warnings(&mut self, warnings: Vec<Warning>) {
        self.warnings = warnings;
    }
}
