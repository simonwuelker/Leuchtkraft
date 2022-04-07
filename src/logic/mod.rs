//! Leuchtkraft's underlying logic engine

mod atom;
mod clause;
mod logic_engine;

pub use atom::{Atom, Var};
pub use clause::Clause;
pub use logic_engine::LogicEngine;

/// Idents are hashed variable names
pub type Ident = u64;
