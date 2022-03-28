/// The smallest syntactical units in the Leuchtkraft language
pub enum Token<'a> {
    /// Indentation level (aka any number of spaces at the beginning of the line)
    /// Since Leuchtkraft scripts only ever have one level one indentation,
    /// parsing indent levels is really simple
    Indent,

    /// An identifier that consists of alphanumeric characters and underscores,
    /// but the first character cannot be anumber
    Ident(&'a str),

    /// An opening parenthesis (`(`)
    OpeningParen,

    /// A closing parenthesis (`)`)
    ClosingParen,

    /// An implication symbol (`=>`)
    Implication,

    /// A questionmark (`?`)
    QuestionMark,

    /// A comma (`,`)
    Comma,

    /// Forall keyword
    Forall,

    /// True primitive
    True,

    /// False primitive
    False,
}
