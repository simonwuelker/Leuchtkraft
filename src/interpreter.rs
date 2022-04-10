use crate::debug::error::Error;
use crate::debug::warning::Warning;
use crate::diagnostics::Diagnostic;
use crate::logic::{Atom, Clause, Ident, LogicEngine, Var};
use crate::parser::parser::Parser;
use crate::parser::span::{Span, Spanned};
use crate::parser::symbol::{Atom as AtomSymbol, Line};
use crate::util::calculate_hash;

pub struct Interpreter {
    inside_scopeblock: bool,
    free_vars: Vec<Ident>,
    logic_engine: LogicEngine,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            inside_scopeblock: false,
            free_vars: vec![],
            logic_engine: LogicEngine::default(),
        }
    }

    /// Resolve any free variables
    fn symbol_to_clause<T>(
        &self,
        and_chains: Vec<Vec<Spanned<AtomSymbol>>>,
    ) -> Result<Clause<T>, Error>
    where
        T: TryFrom<Var> + PartialEq,
    {
        let clause_raw: Result<Vec<Vec<Atom<T>>>, Error> = and_chains
            .into_iter()
            .map(|and_chain| {
                and_chain
                    .into_iter()
                    .map(|atom| {
                        match atom.as_inner() {
                            AtomSymbol::True => Ok(Atom::Boolean(true)),
                            AtomSymbol::False => Ok(Atom::Boolean(false)),
                            AtomSymbol::Predicate(name, args) => {
                                // Check if the args were freed using a forall statement
                                let checked_args: Result<Vec<T>, Error> = args
                                    .into_iter()
                                    .map(|ident_str| {
                                        ident_str.map(str_to_ident(ident_str.as_inner()))
                                    })
                                    .map(|arg| {
                                        let span = arg.span();
                                        let var = if self.free_vars.contains(arg.as_inner()) {
                                            Var::Free(arg.into_inner())
                                        } else {
                                            Var::Fixed(arg.into_inner())
                                        };
                                        T::try_from(var)
                                            .map_err(|_| Error::FreedVarInQuestion { span: span })
                                    })
                                    .collect();
                                Ok(Atom::Predicate(str_to_ident(name), checked_args?))
                            }
                            AtomSymbol::Unknown(ident) => Ok(Atom::Unknown(str_to_ident(ident))),
                        }
                    })
                    .collect()
            })
            .collect();

        clause_raw.map(|and_chain| Clause::new(and_chain))
    }

    pub fn execute<'a>(
        &mut self,
        line: &'a str,
        warnings: &mut Vec<Warning>,
    ) -> Result<Option<String>, Diagnostic<'a>> {
        // Parse the line
        let parser = Parser::new(line);
        let line_content = parser
            .line(warnings)
            .map_err(|err| Diagnostic::from((err, line)))?;
        if let Some(parsed_line) = line_content {
            let line_span = parsed_line.span();
            match parsed_line.into_inner() {
                Line::Forall(free_vars) => {
                    self.inside_scopeblock = true;
                    self.free_vars = free_vars
                        .into_iter()
                        .map(|spanned| spanned.into_inner())
                        .map(|ident_str| str_to_ident(ident_str))
                        .collect();
                }
                Line::Rule(is_indented, is_question, and_chains) => {
                    // Check indentation level
                    match (self.inside_scopeblock, is_indented) {
                        (true, false) => self.inside_scopeblock = false,
                        (false, true) => {
                            return Err(Diagnostic::from((Error::UnexpectedIndent, line)));
                        }
                        _ => {}
                    }

                    // Run some general checks on the clause
                    // (these checks only throw warnings, no errors)
                    sanity_check_clause(&and_chains, line_span, warnings);

                    if is_question {
                        let question: Clause<Ident> = self
                            .symbol_to_clause(and_chains)
                            .map_err(|err| Diagnostic::from((err, line)))?;
                        self.logic_engine.resolve(question);
                    } else {
                        // Can never fail but lets handle the error
                        // anyways, for clarity
                        let clause: Clause<Var> = self
                            .symbol_to_clause(and_chains)
                            .map_err(|err| Diagnostic::from((err, line)))?;
                        self.logic_engine.add(clause);
                    }
                }
            }
        }
        Ok(None)
    }
}

fn str_to_ident(ident_str: &str) -> Ident {
    Ident(calculate_hash(&ident_str))
}

fn sanity_check_clause(
    atoms: &Vec<Vec<Spanned<AtomSymbol>>>,
    clause_span: Span,
    warnings: &mut Vec<Warning>,
) {
    // Perform some sanity checks and generate warnings
    let mut contains_non_literal = false;
    atoms.iter().enumerate().for_each(|(block_ix, and_chain)| {
        // Check for redundant trues (x and true => y)
        if let Some(true_symbol) = and_chain
            .iter()
            .find(|atom| atom.as_inner() == &AtomSymbol::True)
        {
            if and_chain.len() != 1 {
                warnings.push(Warning::RedundantTrue {
                    span: true_symbol.span(),
                });
            }
        }

        // Check for nullifying falses
        if let Some(false_symbol) = and_chain
            .iter()
            .find(|atom| atom.as_inner() == &AtomSymbol::False)
        {
            if and_chain.len() != 1 {
                warnings.push(Warning::NullifyingFalse {
                    span: false_symbol.span(),
                });
            } else if and_chain.len() == 1 && block_ix != atoms.len() - 1 {
                warnings.push(Warning::RedundantFalse {
                    span: false_symbol.span(),
                });
            }
        }

        contains_non_literal |= and_chain
            .iter()
            .find(|atom| !atom.as_inner().is_literal())
            .is_some();
    });
    if !contains_non_literal {
        warnings.push(Warning::PurelyLiteralClause { span: clause_span });
    }
}
