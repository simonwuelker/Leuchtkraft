type Identifier = String;

pub struct Program {
    pub background_knowledge: Vec<HornClause>,
    /// positive and negative training examples
    pub training_data: Vec<Literal>,
    /// The predicate whose clauses must be derived from the background knowledge so they match the training data
    pub target_clause: Literal,
}

/// head <= all(body)
pub struct HornClause {
    pub head: Literal,
    pub body: Vec<Literal>,
}

/// Solve the program using FOIL
pub fn simulate(program: Program) {
    // Initialize the target predicate with an empty body
    let mut result = HornClause {
        head: program.target_clause,
        body: vec![],
    };

    // Continue to add literals to the target predicate body until all positive (and none of the negative) training samples are matched
}

pub fn compile(program: Program) {
    unimplemented!();
}

pub struct Literal {
    pub is_negated: bool,
    pub ident: Identifier,
    pub arguments: Vec<Identifier>
}

