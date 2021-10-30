use lazy_static::lazy_static;
use regex::Regex;
use anyhow::{Result, bail};

macro_rules! search_for_token {
    ($ptr: ident, $firsttoken: ident, $($token: ident), *) => {
        if let Some(m) = $firsttoken.find($ptr) {
            $ptr = &$ptr[m.end()..];
            Token::$firsttoken
        }
        $(
        else if let Some(m) = $token.find($ptr) {
            $ptr = &$ptr[m.end()..];
            Token::$token
        }
        )*
        else {
            bail!("Syntax Error")
        }

    }
}

macro_rules! create_regex {
    ($name: expr) => {
        Regex::new(format!(r"^\s*{}", $expr)).unwrap()
    }
}

pub enum Token {
    IDENT,
    QUESTION,
    COLON,
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

    pub fn read_tokens(&mut self) -> Result<Vec<Token>> {
        let COMMENT: Regex = create_regex!(r"#.*((\r\n)|\n|\r)");
        lazy_static! {
        //    static ref COMMENT: Regex = create_regex!(r"#.*((\r\n)|\n|\r)");
            static ref IDENT: Regex = Regex::new(r"^[\w]+").unwrap();
            static ref QUESTION: Regex = Regex::new(r"^\?").unwrap();
            static ref COLON: Regex = Regex::new(r"^:").unwrap();
        }
        let mut tokens = vec![];
        let mut ptr = &self.text[..];
        while ptr.len() != 0 {
            if let Some(m) = COMMENT.find(ptr) {
                println!("found a comment: {:?}", m.as_str());
                ptr = &self.text[m.end()..];
            } else {
                tokens.push(search_for_token!(ptr, IDENT, QUESTION, COLON));
            }
            println!("remaining source code: {:?}", ptr);
        }
        Ok(tokens)
    }
}
