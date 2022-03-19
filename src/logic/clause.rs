#[derive(Debug, PartialEq)]
pub struct Clause {
    /// In a horn clause, all sub-clauses are concatenated using AND
    subclauses: Vec<OrClause>,
}

#[derive(Debug, PartialEq)]
pub struct OrClause {
    elems: Vec<Atom>,
}

#[derive(Debug, PartialEq)]
pub struct Atom {
    is_negated: bool,
    ident: String,
}

impl Clause {
    pub fn resolve(&self) -> Option<Vec<Atom>> {
        for subclause in &self.subclauses {
            if !subclause.is_resolvable() {
                None
            }

            // Find opposite elements in other subclauses
            for other in &self.subclauses {
            }
        }
    }
}

impl Atom {
    pub fn opposite_of(&self, other: Atom) -> bool {
        this.is_negated ^ other.is_negated && this.ident == other.ident
    }
}

impl OrClause {
    pub fn is_resolvable() {
        for elem in &self.elems {
            for other in &self.elems {
                if elem.opposite_of(other) {
                    false
                }
            }
        }
        true
    }
}
