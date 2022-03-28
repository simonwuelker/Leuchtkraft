use crate::debug::error::{Error, ErrorVariant};

pub fn syntax_error(expected: Vec<char>, unexpected: char, pos: ) -> Error {
    let variant = ErrorVariant::SyntaxError {
        expected: expected,
        unexpected: unexpected,
    };
    Error::new(variant)
}
