use crate::logic::atom::Atom;

#[derive(Debug)]
/// A logical chain of implications and "and"s
pub struct Clause {
    /// The operands within the clause
    pub blocks: Vec<Vec<Atom>>,
}

impl Clause {
    pub fn new(blocks: Vec<Vec<Atom>>) -> Self {
        Self { blocks: blocks }
    }

    pub fn is_question(&self) -> bool {
        for block in &self.blocks {
            for atom in block {
                if let Atom::Unknown(_) = atom {
                    return true;
                }
            }
        }
        false
    }
}
