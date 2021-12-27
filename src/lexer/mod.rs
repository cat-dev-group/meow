//! Lexing is the first step of Meow's execution. The primary goal is to create
//! a stream of [`Token`](crate::lexer::token::Token)s for the parser to
//! consume on demand. Consumption is done through the `.next_token()` method
//! on the `Lexer` struct.
//!
//! The lexer is fairly standard, iterating through a
//! [`Peekable`](std::iter::Peekable) sequence of [`Chars`](std::str::Chars),
//! matching known tokens to variants of the `TokenKind` enum and inserting the
//! `Invalid` variant for any unknown ones. Errors are not emitted here, but
//! rather passed to the parser for it to handle.

pub mod token;

use std::{iter::Peekable, str::Chars};
use token::{
    Token,
    TokenKind::{self, *},
};

/// Returns true if the `c` matches Unicode's Pattern_White_Space. This does
/// not include `\n`, however, because that is handled separately.
fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

/// The `Lexer` struct provides the first step of Meow's execution. It accepts
/// a UTF-8 encoded string, and converts it into a stream of `Token`s for the
/// parser to use to generate an AST.
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: usize,
    line: u32,
    column: u32,
}

impl<'a> Lexer<'a> {
    /// Create a new Lexer instance. This should typically be used through the
    /// top-level `lex()` function, but is initially defined here.
    ///
    /// # Examples
    ///
    /// ```
    /// use meow::lexer::Lexer;
    ///
    /// let program = r#"
    /// fun main() {
    ///     println("Hello, world!");
    /// }
    /// "#;
    /// let mut lexer = Lexer::new(program);
    /// ```
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            position: 0,
            line: 1,
            column: 0,
        }
    }

    /// Move a single position and column forward in the lexer. Returns the
    /// next char in the source.
    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.column += 1;
        self.source.next()
    }

    /// Return the next char in the source without consuming it, or return `\0`
    /// if it is `None`.
    fn peek(&mut self) -> char {
        *self.source.peek().unwrap_or(&'\0')
    }

    /// Given a `TokenKind`, create an `Token` with the `line` and `column`
    /// data from the lexer.
    fn create_token(&mut self, kind: TokenKind) -> Token {
        Token::new(kind, self.line, self.column)
    }

    /// Match the next token. If it's the expected character, generate a
    /// specified token. Otherwise, generate another specified token.
    fn with_single_or_double(
        &mut self,
        expected_double: char,
        single: TokenKind,
        double: TokenKind,
    ) -> Token {
        if self.peek() == expected_double {
            let token = self.create_token(double);
            self.advance();
            token
        } else {
            self.create_token(single)
        }
    }

    /// Match the next token. If it's the expected character, generate a
    /// specified token. Otherwise, generate an Invalid token.
    fn with_double(&mut self, expected: char, kind: TokenKind) -> Token {
        if self.peek() == expected {
            let token = self.create_token(kind);
            self.advance();
            token
        } else {
            self.create_token(Invalid)
        }
    }

    /// Return the next `Token` for use in the parser. This is the method that
    /// completes the final token matching and pulls together all other parts
    /// of the Lexer.
    ///
    /// # Examples
    ///
    /// ```
    /// use meow::lexer::{Lexer, token::TokenKind};
    ///
    /// let program = r#"
    /// fun main() {
    ///     println("Hello, world!");
    /// }
    /// "#;
    /// let mut lexer = Lexer::new(program);
    ///
    /// loop {
    ///     let next = lexer.next_token();
    ///     if next.kind == TokenKind::Eof {
    ///         break;
    ///     } else {
    ///         println!("{}", next);
    ///     }
    /// }
    /// ```
    pub fn next_token(&mut self) -> Token {
        let next = self.advance();

        if let Some(c) = next {
            return match c {
                '.' if self.peek() == '.' => {
                    self.advance();
                    if self.peek() == '=' {
                        let token = self.create_token(RangeInclusive);
                        self.advance();
                        token
                    } else {
                        self.create_token(Range)
                    }
                }

                '(' => self.create_token(OpenParen),
                ')' => self.create_token(CloseParen),
                '[' => self.create_token(OpenBracket),
                ']' => self.create_token(CloseBracket),
                '{' => self.create_token(OpenBrace),
                '}' => self.create_token(CloseBrace),
                ',' => self.create_token(Comma),
                '.' => self.create_token(Dot),
                ';' => self.create_token(Semicolon),

                '&' => self.with_double('&', And),
                '|' => self.with_double('|', Or),

                '=' => self.with_single_or_double('=', Equal, EqualEqual),
                '!' => self.with_single_or_double('=', Bang, BangEqual),
                '>' => self.with_single_or_double('=', Greater, GreaterEqual),
                '<' => self.with_single_or_double('=', Less, LessEqual),
                '+' => self.with_single_or_double('=', Plus, PlusEqual),
                '-' => self.with_single_or_double('=', Minus, MinusEqual),
                '*' => self.with_single_or_double('=', Star, StarEqual),
                '/' => self.with_single_or_double('=', Slash, SlashEqual),

                c if is_whitespace(c) => self.next_token(),

                _ => self.create_token(Invalid),
            };
        }

        self.create_token(Eof)
    }
}
