use std::fmt;

/// The `TokenKind` enum contains every possible Token that the Meow lexer
/// could return. This is not intended for use outside the lexer.
///
/// Some sections do contain additional data. This is not stored in the `Token`
/// struct because there is no reason to hold the content of simple tokens such
/// as `OpenParen`. That will always be `(`, and the language uses that
/// knowledge when needed.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    // single char tokens
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Semicolon,

    // two or more char tokens
    And,
    Or,
    Range,
    RangeInclusive,

    // single or double char tokens
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,

    // literals
    Str(String),
    Char(char),
    Int(String),
    Float(String),

    // identifiers
    Ident(String),

    // Keywords
    // `True` and `False` are considered boolean literals, but will be lexed as
    // as keywords for simplicity and ease of implementation
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Impls,
    Import,
    Match,
    Mut,
    Return,
    Trait,
    True,
    Let,
    While,

    Error(String),

    Eof,
}

/// The `Token` struct stores the type of a single lexeme, as well as the line
/// and column on which it starts. The end is not included, since that can be
/// computed on demand as need be.
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub column: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} at {}:{}", self.kind, self.line, self.column)
    }
}

impl Token {
    /// Create a new token. This is only used in the interpreter by way of the
    /// various methods on the `Lexer` struct, but is available for testing
    /// purposes.
    pub fn new(kind: TokenKind, line: u32, column: u32) -> Self {
        Self { kind, line, column }
    }
}
