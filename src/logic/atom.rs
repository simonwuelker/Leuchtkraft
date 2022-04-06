use crate::interpreter::Ident;

#[derive(Debug, Clone, PartialEq)]
/// The smallest (atomic) operand in a logical formula.
pub enum Atom<T: PartialEq> {
    /// A boolean value, either `true` or `false`
    Boolean(bool),
    Predicate(Ident, Vec<T>),
    Unknown(Ident),
}

#[derive(Debug, Clone, PartialEq)]
/// Objects that can be passed as arguments to predicates
pub enum Var {
    Fixed(Ident),
    Anonymous(Ident),
}

impl Atom<Var> {
    pub fn pin_var(&mut self, to_pin: Ident, pin_to: Ident) {
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

    /// Match an atom with a predicate.
    /// If the atom matched, a map of anonymous idents to the pinned idents will be returned
    pub fn match_predicate(&self, predicate: (&Ident, &Vec<Ident>)) -> Option<Vec<(Ident, Ident)>> {
        match self {
            // Only Predicates can be matched
            Atom::Predicate(ident, args) => {
                // Predicate name and number of args must be equal
                if ident == predicate.0 && args.len() == predicate.1.len() {
                    let mut anon_arg_map = vec![];
                    // For each argument, check if that argument is either
                    // the same literal value,
                    // or a free variable that can be pinned to the literal
                    for (arg, arg_2) in args.iter().zip(predicate.1) {
                        match arg {
                            Var::Fixed(fixed_ident) => {
                                if fixed_ident != arg_2 {
                                    return None;
                                }
                            }
                            Var::Anonymous(anon_ident) => {
                                // Check if that free ident has previously been pinned
                                // to another variable
                                if anon_arg_map
                                    .iter()
                                    .any(|(from, to)| from == anon_ident && to != arg_2)
                                {
                                    return None;
                                }

                                // Pin that free ident to the args value
                                anon_arg_map.push((*anon_ident, *arg_2));
                            }
                        }
                    }
                    Some(anon_arg_map)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
