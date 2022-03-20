use pest::iterators::Pair;
use crate::parser::Rule;
use crate::interpreter::Interpreter;
use crate::logic::Ident;


#[derive(Debug, Clone, PartialEq)]
/// The smallest (atomic) operand in a logical formula.
pub enum Atom {
    /// A boolean value, either `true` or `false`
    Boolean(bool),
    Predicate {
        name: Ident,
        args: Vec<Var>,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
/// Objects that can be passed as arguments to predicates
pub enum Var {
    Fixed(Ident),
    Anonymous(Ident),
}

impl Atom {
    pub fn is_unknown(&self) -> bool {
        match self {
            Atom::Unknown => true,
            _ => false,
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
                    args: idents.map(|s| Var::Fixed(s)).collect(),
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

    pub fn replace(&mut self, to_replace: &Var, replace_with: &Var) {
        match self {
            Atom::Predicate{name, args} => {
                for arg in args {
                    if arg == to_replace {
                        *arg = replace_with.clone();
                    }
                }
            }
            _ => {},
        }
    }

    /// other is assumed to not be anonymous
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Atom::Boolean(x), Atom::Boolean(y)) => x == y,
            (Atom::Predicate{name, args}, Atom::Predicate{name: othername, args: otherargs}) => {
                // Predicate names must be the same
                if name != othername {
                    return false;
                }
                return args.iter().zip(otherargs).all(|(a1, a2)| {
                    if let Var::Anonymous(_) = a1 {
                        return true;
                    } else {
                        return a1 == a2;
                    }
                });

            }
            Unknown => false,
        }
    }
}

