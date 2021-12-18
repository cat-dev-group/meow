//! Lexing is the first step of Meow's execution. The primary goal is to create
//! a stream of [`Token`](lexer::Token)s for the parser to consume on demand.
//! Consumption is done through the `.next()` instance method on the `Lexer`
//! struct.
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

/// The `Lexer` struct provides the first step of Meow's execution. It accepts
/// a UTF-8 encoded string, and converts it into a stream of `Token`s for the
/// parser to use to generate an AST.
pub struct Lexer<'a> {
    input: String,
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
    /// let program = "
    /// fun main() {
    ///     println(\"Hello, world!\");
    /// }
    /// ";
    /// let lexer = Lexer::new(program);
    /// ```
    pub fn new(source: &'a str) -> Self {
        Self {
            input: source.to_string(),
            source: source.chars().peekable(),
            position: 0,
            line: 1,
            column: 0,
        }
    }

    /// Move forward by a single position in the source, and update the `line`
    /// and `column` fields appropriately. This also returns a `char` to be
    /// used in the `.next()` method.
    fn advance(&mut self) -> Option<char> {
        let current = match self.source.next() {
            Some(c) => Some(c),
            None => None,
        };

        self.position += 1;
        if current == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        current
    }

    /// Given a `TokenKind`, create an `Option<Token>` with the `line` and
    /// `column` data from the lexer.
    fn create_token(&mut self, kind: TokenKind) -> Option<Token> {
        Some(Token {
            kind,
            line: self.line,
            column: self.column,
        })
    }

    fn with_single(&mut self, token: TokenKind) -> Option<Token> {
        return match self.source.next() {
            Some(_) => self.create_token(token),
            None => None,
        };
    }

    fn with_double(&mut self, token: TokenKind, second_char: char) -> Option<Token> {
        match self.source.next() {
            Some(second) => {
                if second == second_char {
                    self.create_token(token)
                } else {
                    self.create_token(Invalid)
                }
            }
            None => None,
        }
    }

    fn with_single_or_double(
        &mut self,
        expected_double: char,
        single: TokenKind,
        double: TokenKind,
    ) -> Option<Token> {
        return match self.source.next() {
            Some(next) => {
                if next == expected_double {
                    self.advance();
                    self.create_token(double)
                } else {
                    self.create_token(single)
                }
            }
            None => None,
        };
    }

    fn dot_and_ranges(&mut self) -> Option<Token> {
        if let Some(second) = self.source.next() {
            if second == '.' {
                if let Some(third) = self.source.next() {
                    if third == '=' {
                        self.create_token(RangeInclusive)
                    } else {
                        self.create_token(Range)
                    }
                } else {
                    None
                }
            } else {
                self.create_token(Dot)
            }
        } else {
            None
        }
    }

    /// Receive the next `Token` for use in the parser. This is the method that
    /// completes the final token matching and pulls together all other parts
    /// of the Lexer.
    ///
    /// # Examples
    ///
    /// ```
    /// use meow::lexer::Lexer;
    ///
    /// let program = "
    /// fun main() {
    ///     println(\"Hello, world!\");
    /// }
    /// ";
    /// let mut lexer = Lexer::new(program);
    /// while let Some(token) = lexer.next() {
    ///    println!("{}", token);
    /// }
    /// ```
    pub fn next(&mut self) -> Option<Token> {
        let c = self.advance().unwrap_or('\0');

        match c {
            '\0' => {
                if self.position > self.input.len() {
                    None
                } else {
                    self.create_token(Eof)
                }
            }

            '(' => self.with_single(OpenParen),
            ')' => self.with_single(CloseParen),
            '[' => self.with_single(OpenBracket),
            ']' => self.with_single(CloseBracket),
            '{' => self.with_single(OpenBrace),
            '}' => self.with_single(CloseBrace),
            ',' => self.with_single(Comma),
            ';' => self.with_single(Semicolon),

            '&' => self.with_double(And, '&'),
            '|' => self.with_double(Or, '|'),

            '.' => self.dot_and_ranges(),

            '=' => self.with_single_or_double('=', Equal, EqualEqual),
            '!' => self.with_single_or_double('=', Bang, BangEqual),
            '>' => self.with_single_or_double('=', Greater, GreaterEqual),
            '<' => self.with_single_or_double('=', Less, LessEqual),
            '+' => self.with_single_or_double('=', Plus, PlusEqual),
            '-' => self.with_single_or_double('=', Minus, MinusEqual),
            '*' => self.with_single_or_double('=', Star, StarEqual),
            '/' => self.with_single_or_double('=', Slash, SlashEqual),

            ' ' | '\n' | '\t' => self.next(),

            _ => self.create_token(Invalid),
        }
    }
}
