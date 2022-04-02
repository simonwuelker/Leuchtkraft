#[derive(Debug, PartialEq, Clone, Copy)]
/// Atomic syntactical units
pub enum Token {
    /// An identifier that consists of alphanumeric characters and underscores,
    /// but the first character cannot be anumber
    Ident,

    /// Indentation token
    /// Since Leuchtkraft only supports one level of indentation,
    /// ANY number of spaces or tabs (or mixtures of both) at the beginning of a line
    /// will be interpreted as an indentation
    Indent,

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

    /// And keyword
    And,

    /// True primitive
    True,

    /// False primitive
    False,

    /// Singleline comment (`//`)
    Comment,

    /// Spaces (used for indentation and seperating tokens)
    Space,

    /// A token marking the end of the input stream
    End,
}
