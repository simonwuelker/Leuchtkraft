use std::fmt;

#[derive(PartialEq, Clone, Copy)]
/// Idents are hashed variable names
pub struct Ident(pub u64);

#[derive(Clone, PartialEq)]
/// The smallest (atomic) operand in a logical formula.
pub enum Atom<T> {
    /// A boolean value, either `true` or `false`
    Boolean(bool),
    Predicate(Ident, Vec<T>),
    Unknown(Ident),
}

#[derive(Clone, PartialEq)]
/// Objects that can be passed as arguments to predicates
pub enum Var {
    Fixed(Ident),
    Free(Ident),
}

impl Atom<Var> {
    pub fn pin_var(&mut self, to_pin: Ident, pin_to: Ident) {
        match self {
            Self::Predicate(_, args) => {
                for arg in args {
                    if let Var::Free(ident) = arg {
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
    pub fn match_predicate(&self, predicate: (&Ident, &Vec<Var>)) -> Option<Vec<(Ident, Ident)>> {
        match self {
            // Only Predicates can be matched
            Atom::Predicate(ident, args) => {
                // Predicate name and number of args must be equal
                if ident == predicate.0 && args.len() == predicate.1.len() {
                    let mut free_arg_map = vec![];
                    // For each argument, check if that argument is either
                    // the same literal value,
                    // or a free variable that can be pinned to the literal
                    for (arg, arg_2) in args.iter().zip(predicate.1) {
                        match (arg, arg_2) {
                            (Var::Fixed(ident_1), Var::Fixed(ident_2)) => {
                                if ident_2 != ident_2 {
                                    return None;
                                }
                            }
                            (Var::Free(free_ident), Var::Fixed(ident_2)) => {
                                // Check if that free ident has previously been pinned
                                // to another variable
                                if free_arg_map
                                    .iter()
                                    .any(|(from, to)| from == free_ident && to != ident_2)
                                {
                                    return None;
                                }

                                // Pin that free ident to the args value
                                free_arg_map.push((*free_ident, *ident_2));
                            }
                            (Var::Free(_), Var::Free(_)) => {} // no action required
                            (Var::Fixed(ident_2), Var::Free(free_ident_2)) => {}
                        }
                    }
                    Some(free_arg_map)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl TryFrom<Var> for Ident {
    type Error = ();

    fn try_from(from: Var) -> Result<Ident, Self::Error> {
        match from {
            Var::Fixed(ident) => Ok(ident),
            Var::Free(_) => Err(()),
        }
    }
}

impl From<Ident> for Var {
    fn from(from: Ident) -> Self {
        Self::Fixed(from)
    }
}

impl From<Atom<Ident>> for Atom<Var> {
    fn from(from: Atom<Ident>) -> Self {
        match from {
            Atom::Boolean(b) => Self::Boolean(b),
            Atom::Unknown(i) => Self::Unknown(i),
            Atom::Predicate(i, args) => {
                let new_args = args.into_iter().map(|arg| arg.into()).collect();
                Self::Predicate(i, new_args)
            }
        }
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // display 5 digits to maintain readability
        // this is a fucking stupid way to do it but i can't think of a better one
        write!(f, "{:.5}", self.0.to_string())
    }
}

impl<T: fmt::Debug> fmt::Debug for Atom<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Boolean(b) => write!(f, "{:?}", b),
            Atom::Predicate(ident, args) => {
                write!(f, "{:?}", ident)?;
                write!(f, "(")?;
                for (index, arg) in args.iter().enumerate() {
                    write!(f, "{:?}", arg)?;
                    if index != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Atom::Unknown(i) => write!(f, "{:?}?", i),
        }
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed(i) => write!(f, "{:?}", i),
            Self::Free(i) => write!(f, "free({:?})", i),
        }
    }
}
