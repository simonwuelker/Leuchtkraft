use lazy_static::lazy_static;
use regex::RegexSet;

pub enum Token {
    Comment,
    Variable(String),
    Question,
    Colon,
    EOF,
}

/// Converts the program into a stream of tokens
pub struct Lexer {
    text: String,
    pos: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            pos: 0,
        }
    }

    pub fn read_tokens(&mut self) {
        lazy_static! {
            static ref set: RegexSet = RegexSet::new(&[
                r"#.*\n", // Oneline Comments
                r"[\w]+", // Variable Names
                r"\?", // questions
                r":", // Colons
            ]).unwrap();
        }
        for m in set.matches(&self.text).iter() {
            println!("{:?}", m.start);
        }
    }
}
