use crate::debug::error::Error;
use crate::debug::warning::Warning;
use crate::diagnostics::Diagnostic;
use crate::logic::{Atom, Clause, LogicEngine, Var};
use crate::parser::parser::Parser;
use crate::parser::symbol::{Atom as AtomSymbol, Line};

/// Idents are hashed variable names
pub type Ident = u64;

pub struct Interpreter {
    inside_scopeblock: bool,
    anon_vars: Vec<Ident>,
    logic_engine: LogicEngine,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            inside_scopeblock: false,
            anon_vars: vec![],
            logic_engine: LogicEngine::default(),
        }
    }

    /// Resolve any free variables
    fn symbol_to_clause(&self, and_chains: Vec<Vec<AtomSymbol>>) -> (Clause, bool) {
        let mut is_question = false;
        let clause_raw = and_chains
            .into_iter()
            .map(|and_chain| {
                and_chain
                    .into_iter()
                    .map(|atom| {
                        match atom {
                            AtomSymbol::True => Atom::Boolean(true),
                            AtomSymbol::False => Atom::Boolean(false),
                            AtomSymbol::Predicate(name, args) => {
                                // Check if the args were freed using a forall statement
                                let checked_args = args
                                    .into_iter()
                                    .map(|arg| {
                                        if self.anon_vars.contains(&arg) {
                                            Var::Anonymous(arg)
                                        } else {
                                            Var::Fixed(arg)
                                        }
                                    })
                                    .collect();
                                Atom::Predicate(name, checked_args)
                            }
                            AtomSymbol::Unknown(ident) => {
                                is_question = true;
                                Atom::Unknown(ident)
                            }
                        }
                    })
                    .collect()
            })
            .collect();
        (Clause::new(clause_raw), is_question)
    }

    pub fn execute<'a>(
        &mut self,
        line: &'a str,
    ) -> (Vec<Warning>, Result<Option<String>, Diagnostic<'a>>) {
        let mut warnings = vec![];

        // Parse the line
        let parser = Parser::new(line);
        match parser.line(&mut warnings) {
            Ok(line_content) => {
                if let Some(parsed_line) = line_content {
                    match parsed_line.into_inner() {
                        Line::Forall(free_vars) => {
                            self.inside_scopeblock = true;
                            self.anon_vars = free_vars.to_vec();
                        }
                        Line::Rule(is_indented, and_chains) => {
                            match (self.inside_scopeblock, is_indented) {
                                (true, true) => {}
                                (true, false) => self.inside_scopeblock = false,
                                (false, true) => {
                                    return (
                                        warnings,
                                        Err(Diagnostic::from((&Error::UnexpectedIndent, line))),
                                    );
                                }
                                (false, false) => {}
                            }
                            let (clause, is_question) = self.symbol_to_clause(and_chains);

                            if is_question {
                                self.logic_engine.resolve(clause);
                            } else {
                                self.logic_engine.add(clause);
                            }
                        }
                    }
                }
                (warnings, Ok(None))
            }
            Err(error) => (warnings, Err(Diagnostic::from((error, line)))),
        }
    }
}
