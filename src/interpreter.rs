pub enum Token {
    Comment,
    EOF,
}

pub struct Interpreter {
    /// The program code to be executed
    text: String,
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
        }
    }

    pub fn next_token(&mut self) -> Token {
        Token::EOF
    }
}
