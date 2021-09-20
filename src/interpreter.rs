pub enum Token {
    Comment,
    EOF,
}

enum State {
    Clear,
    Comment,
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

    pub fn tokenize(self) -> Vec<Token> {
        let mut state = State::Clear;

        for c in self.text.chars() {
            match &c {
                '#' => state = State::Comment,
                '\r' | '\n' => state = State::Clear,
                _ => {}
            }
            if let State::Comment = state {
            } else {
                println!("{:?}", c);
            }
        }
        vec![]
    }
}
