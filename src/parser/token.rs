#[derive(Debug)]
/// The smallest syntactical units in the Leuchtkraft language
pub struct Token<'a> {
    /// The matched characters of the input string
    matched: &'a str,
    /// the type of token
    token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    /// An identifier that consists of alphanumeric characters and underscores,
    /// but the first character cannot be anumber
    Ident,

    /// An opening parenthesis (`(`)
    OpeningParen,

    /// A closing parenthesis (`)`)
    ClosingParen,

    /// An implication symbol (`=>`)
    Implication,

    /// A questionmark (`?`)
    Questionmark,

    /// A comma (`,`)
    Comma,

    /// Forall keyword
    Forall,

    /// True primitive
    True,

    /// False primitive
    False,

    /// Singleline comment (`//`)
    SinglelineComment,

    /// Multiline comment open (`/*`)
    MultilineCommentOpen,

    /// Multiline comment close (`*/`)
    MultilineCommentClose,

    /// Spaces (used for indentation and seperating tokens)
    Space,

    /// Tabs (used for indentation)
    Tab,
}

impl<'a> Token<'a> {
    pub fn new(matched: &'a str, token_type: TokenType) -> Self {
        Self {
            matched: matched,
            token_type: token_type,
        }
    }

    pub fn type(&self) -> TokenType {
        self.token_type
    }
}
