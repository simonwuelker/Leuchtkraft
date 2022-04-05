//! Leuchtkraft's underlying logic engine

mod atom;
mod logic_engine;

pub use atom::{Atom, Clause, Var};
pub use logic_engine::LogicEngine;
