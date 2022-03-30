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

    /// A character that cannot be directly identified as a token
    Character,

    /// A token marking the end of the input stream
    End,
}
