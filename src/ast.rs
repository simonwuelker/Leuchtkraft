// built-in data types
pub enum Primitive {
    String(String),
    Integer(i64),
    Boolean(bool),
}

// A Object
pub enum Object {
    Atom(Identifier),
    Primitive(Primitive),
    Variable(Identifier),
    Structure(Vec<Object>),
    Anonymous,
}

// built-in comparison operators
pub enum ComparisonOperator {
    Equal,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    NotEqual,
}

// built-in arithmetic operators
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulus,
    IntegerDivide,
    Increment,
    Decrement,
}

// built-in boolean operators
pub enum BooleanOperator {
    Not,
    And,
    Or,
    Xor,
}

// Operations
pub enum Operation {
    DefineAs,
    And
}
