use pest::iterators::Pair;
use crate::parser::Rule;
use crate::interpreter::Interpreter;
use crate::logic::{Ident, EvalResult};


#[derive(Debug, PartialEq)]
pub enum Atom {
    Boolean(bool),
    Predicate {
        name: Ident,
        args: Vec<Ident>,
    },
    Unknown,
}

impl Atom {
    pub fn is_unknown(&self) -> bool {
        match self {
            Atom::Unknown => true,
            _ => false,
        }
    }

    pub fn eval(&self, interpreter: &Interpreter) -> EvalResult {
        match self {
            Atom::Boolean(b) => {
                match b {
                    true => EvalResult::True,
                    false => EvalResult::False,
                }
            },
            Atom::Predicate{ name, args } => {
                // TODO
                EvalResult::Indeterminate
            },
            Atom::Unknown => {
                log::warn!("{}", "Called 'Atom::eval' on Unknown");
                EvalResult::Indeterminate
            },
        }
    }
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(
            std::mem::discriminant(&Rule::Atom), 
            std::mem::discriminant(&pair.as_rule())
        );

        let child = pair.into_inner().next().unwrap();
        match child.as_rule() {
            Rule::Predicate => {
                let mut idents = child.into_inner().map(|p| p.to_string());
                Self::Predicate {
                    name: idents.next().unwrap(),
                    args: idents.collect(),
                }
            },
            Rule::Boolean => {
                match child.as_str() {
                    "true" => Self::Boolean(true),
                    "false" => Self::Boolean(false),
                    _ => unreachable!(),
                }
            },
            Rule::Unknown => Self::Unknown,
            _ => unreachable!("Converting non-atom value to atom"),
        }
    }
}

