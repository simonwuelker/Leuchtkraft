use crate::debug::warning::Warning;
use crate::diagnostics::Diagnostic;
use crate::logic::logic_engine::LogicEngine;
use crate::logic::{atom::Atom, clause::Clause};
use crate::parser::error::TokenNotFound;
use crate::parser::parser::Parser;

/// Idents are hashed variable names
pub type Ident = u64;

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

    pub fn execute<'a>(
        &mut self,
        line: &'a str,
    ) -> (Vec<Warning>, Result<Option<String>, Diagnostic<'a>>) {
        let expected_indentation = Some(0);
        let mut warnings = vec![];

        // Parse the line
        let mut parser = Parser::new(line);
        match parser.line(&mut warnings) {
            Ok(line) => (warnings, Ok(None)),
            Err(error) => (warnings, Err(Diagnostic::from((error, line)))),
        }
    }
}
