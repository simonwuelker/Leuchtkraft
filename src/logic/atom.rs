use crate::interpreter::Ident;

#[derive(Debug, Clone, PartialEq)]
/// The smallest (atomic) operand in a logical formula.
pub enum Atom {
    /// A boolean value, either `true` or `false`
    Boolean(bool),
    Predicate(Ident, Vec<Var>),
    Unknown(Ident),
}

#[derive(Debug, Clone, PartialEq)]
/// Objects that can be passed as arguments to predicates
pub enum Var {
    Fixed(Ident),
    Anonymous(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Clause(pub Vec<Vec<Atom>>);

impl Clause {
    pub fn new(and_chains: Vec<Vec<Atom>>) -> Self {
        Self(and_chains)
    }

    /// Pin all occurences of an anonymous var to a fixed var
    pub fn pin(&self, to_pin: Ident, pin_to: Ident) -> Self {
        let mut cloned = self.clone();
        for and_chain in &mut cloned.0 {
            for atom in and_chain {
                atom.pin_var(to_pin, pin_to);
            }
        }
        cloned
    }

    pub fn contains(&self, search_for: &Atom) -> bool {
        self.0
            .iter()
            .any(|and_chain| and_chain.iter().any(|atom| atom == search_for))
    }
}

impl Atom {
    fn pin_var(&mut self, to_pin: Ident, pin_to: Ident) {
        match self {
            Self::Predicate(_, args) => {
                for arg in args {
                    if let Var::Anonymous(ident) = arg {
                        if ident == &to_pin {
                            *arg = Var::Fixed(pin_to);
                        }
                    }
                }
            }
            _ => {} // only pin predicate args
        }
    }
}
