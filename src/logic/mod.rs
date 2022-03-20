pub type Ident = String;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    True,
    False,
    Indeterminate,
}

pub mod atom;
pub mod clause;
