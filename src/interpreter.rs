use crate::debug::error::{Error, ErrorVariant};
use crate::debug::warning::Warning;
use crate::logic::logic_engine::LogicEngine;
use crate::logic::{atom::Atom, clause::Clause};
use crate::parser::lexer::Lexer;

type Ident = String;

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

    pub fn execute(&mut self, line: &str) -> Result<Response, Error> {
        println!("Executing {}", line);

        // Parse the line
        let mut lexer = Lexer::new(line);
        // let tokens = lexer.tokenize()?;

        // match tokens.get().type() {
        //     TokenType::Indent
        // }

        // last child is always EOI
        // for child in pair.clone().into_inner() {
        //     match child.as_rule() {
        //         Rule::Rule => {
        //             self.state.inside_scopeblock = false;

        //             let mut blocks = vec![vec![]];

        //             child
        //                 .into_inner()
        //                 .enumerate()
        //                 .for_each(|(ix, node)| match node.as_rule() {
        //                     Rule::Implication => blocks.push(vec![]),
        //                     Rule::Atom => {
        //                         blocks
        //                             .last_mut()
        //                             .unwrap()
        //                             .push(Atom::from_pair(node.clone()));
        //                     }
        //                     _ => unreachable!(),
        //                 });
        //             let clause = Clause::new(blocks);
        //             if clause.is_question() {
        //                 self.logic.resolve(clause);
        //             } else {
        //                 self.logic.add_clause(clause);
        //             }
        //         }
        //         Rule::Scopeblock => {
        //             self.state.inside_scopeblock = true;

        //             for anon_var in child.into_inner() {
        //                 let ident = anon_var.as_str().to_string();
        //                 if self.state.anon_vars.contains(&ident) {
        //                     return Err(Error::new(
        //                         ErrorVariant::ParseError("Duplicate scoped variable".to_string()),
        //                         anon_var.as_span().into(),
        //                     ));
        //                 } else {
        //                     self.state.anon_vars.push(anon_var.as_str().to_string());
        //                 }
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        Ok(Response::empty())
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
