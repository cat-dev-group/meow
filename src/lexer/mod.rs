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
        | '\u{000A}' // \n
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
            position: 1,
            line: 1,
            column: 1,
        }
    }

    /// Move a single position and column forward in the lexer. Returns the
    /// next char in the source.
    fn advance(&mut self) -> Option<char> {
        self.position += 1;
        self.column += 1;
        self.source.next()
    }

    /// Move a single position and line forward in the lexer, and reset the
    /// rolumn. Returns the next char in the source.
    fn advance_line(&mut self) -> Option<char> {
        self.position += 1;
        self.line += 1;
        self.column = 1;
        self.source.next()
    }

    /// Advance tokens while being aware of newlines.
    fn newline_aware_advance(&mut self) -> Option<char> {
        if self.peek() == '\n' {
            self.advance_line()
        } else {
            self.advance()
        }
    }

    /// Return the next char in the source without consuming it, or return `\0`
    /// if it is `None`.
    fn peek(&mut self) -> char {
        *self.source.peek().unwrap_or(&'\0')
    }

    // Return true or false based on whether the lexer is at the end of the source code
    fn at_end(&mut self) -> bool {
        self.peek() == '\0'
    }

    /// Given a `TokenKind`, create an `Token` with the `line` and `column`
    /// data from the lexer.
    fn create_token(&mut self, kind: TokenKind, length: u32) -> Token {
        let position = self.position as u32 - length;
        Token::new(kind, self.line, position, self.position)
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
            let token = self.create_token(double, 2);
            self.advance();
            token
        } else {
            self.create_token(single, 1)
        }
    }

    /// Match the next token. If it's the expected character, generate a
    /// specified token. Otherwise, generate an Invalid token.
    fn with_double(&mut self, expected: char, kind: TokenKind) -> Token {
        let c = self.peek();
        if c == expected {
            let token = self.create_token(kind, 1);
            self.advance();
            token
        } else {
            self.create_token(
                Error(format!("Unknown character `{}` found in source", c)),
                1,
            )
        }
    }

    // Lexes a string
    fn lex_string(&mut self) -> Token {
        let mut value = String::new();

        while self.peek() != '"' {
            let char = self.newline_aware_advance();
            value.push(char.unwrap());
        }

        let length = value.len() as u32;

        if self.at_end() {
            return self.create_token(Error("Unterminated string literal, expected closing quote, EOF (End of File) encountered".to_string()), length + 1);
        }

        self.advance();
        self.create_token(Str(value), length + 2)
    }

    // Lexes either an integer or a float
    fn lex_number(&mut self, first_char: char) -> Token {
        let mut is_integer = true;

        let mut value = String::from(first_char);

        if self.at_end() {
            return self.create_token(TokenKind::Int(value), 1);
        }

        while self.peek().is_numeric() {
            let char = self.advance();
            value.push(char.unwrap());
        }

        if self.peek() == '.' {
            // Set is_integer to false, since dot indicates that value is a decimal
            is_integer = false;
            // Consume and add dot to value
            value.push(self.advance().unwrap());
            while self.peek().is_numeric() {
                value.push(self.advance().unwrap())
            }
        }

        let length = value.len() as u32;

        self.create_token(
            if is_integer {
                TokenKind::Int(value)
            } else {
                TokenKind::Float(value)
            },
            length,
        )
    }

    // Checks whether a given value matches the keyword
    fn get_keyword(
        &self,
        value: &str,
        keyword: &str,
        length: usize,
        token: TokenKind,
    ) -> TokenKind {
        if value[length..] == keyword[length..] {
            token
        } else {
            TokenKind::Ident(value.to_string())
        }
    }

    // Use a state machine to single out Meow keywords
    fn ident_type(&self, value: &str) -> TokenKind {
        match &value[..1] {
            "c" => self.get_keyword(value, "class", 1, TokenKind::Class),
            "e" => self.get_keyword(value, "else", 1, TokenKind::Else),
            "f" => {
                if value.len() < 2 {
                    TokenKind::Ident(value.to_string());
                }

                match &value[1..2] {
                    "a" => self.get_keyword(value, "false", 2, TokenKind::False),
                    "o" => self.get_keyword(value, "for", 2, TokenKind::For),
                    "u" => self.get_keyword(value, "fun", 2, TokenKind::Fun),
                    _ => TokenKind::Ident(value.to_string()),
                }
            }
            "i" => {
                if value.len() < 2 {
                    return TokenKind::Ident(value.to_string());
                }

                match &value[1..2] {
                    "f" => TokenKind::If,
                    "m" => {
                        if value.len() < 5 {
                            return TokenKind::Ident(value.to_string());
                        }

                        if &value[2..3] == "p" {
                            return match &value[3..4] {
                                "o" => self.get_keyword(value, "import", 4, TokenKind::Import),
                                "l" => self.get_keyword(value, "impls", 4, TokenKind::Impls),
                                _ => TokenKind::Ident(value.to_string()),
                            };
                        }

                        TokenKind::Ident(value.to_string())
                    }
                    _ => TokenKind::Ident(value.to_string()),
                }
            }
            "l" => self.get_keyword(value, "let", 1, TokenKind::Let),
            "m" => {
                if value.len() < 2 {
                    return TokenKind::Ident(value.to_string());
                }

                match &value[1..2] {
                    "a" => self.get_keyword(value, "match", 2, TokenKind::Match),
                    "u" => self.get_keyword(value, "mut", 2, TokenKind::Mut),
                    _ => TokenKind::Ident(value.to_string()),
                }
            }
            "r" => self.get_keyword(value, "return", 1, TokenKind::Return),
            "t" => {
                if value.len() < 3 {
                    return TokenKind::Ident(value.to_string());
                }

                if !(&value[1..2] == "r") {
                    return TokenKind::Ident(value.to_string());
                }

                match &value[2..3] {
                    "u" => self.get_keyword(value, "true", 3, TokenKind::True),
                    "a" => self.get_keyword(value, "trait", 3, TokenKind::Trait),
                    _ => TokenKind::Ident(value.to_string()),
                }
            }
            "w" => return self.get_keyword(value, "while", 1, TokenKind::While),
            _ => TokenKind::Ident(value.to_string()),
        }
    }

    // Lexes identifiers and keywords
    fn get_ident(&mut self, first_char: char) -> Token {
        let mut value = String::from(first_char);

        if self.at_end() {
            return self.create_token(TokenKind::Ident(value), 1);
        }

        // Add to the eventual value as long as the next character is a valid identifer
        while unicode_xid::UnicodeXID::is_xid_continue(self.peek()) {
            let char = self.advance();
            value.push(char.unwrap());
        }

        let token_type = self.ident_type(&value);
        self.create_token(token_type, value.len() as u32)
    }

    // Lexes a single char
    fn lex_char(&mut self) -> Token {
        // If at end, create an error token since there isn't a closing quote
        if self.at_end() {
            return self.create_token(
                Error("Unterminated char literal, expected closing single quote".to_string()),
                1,
            );
        }

        let value = self.advance().unwrap();

        // If the value is a closing quote, create an error token since empty char literals aren't allowed
        if value == '\'' {
            return self.create_token(
                Error("Unterminated char literal, expected closing single quote".to_string()),
                1,
            );
        }

        // If no closing quote is found, create an error token
        if self.peek() != '\'' {
            return self.create_token(
                Error("Unterminated char literal, expected closing single quote".to_string()),
                1,
            );
        }
        // Consume closing quote
        self.advance();

        self.create_token(Char(value), 1)
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
        let next = self.newline_aware_advance();

        if let Some(c) = next {
            return match c {
                // range characters
                '.' if self.peek() == '.' => {
                    self.advance();
                    let mut token = self.create_token(Range, 2);
                    if self.peek() == '=' {
                        self.advance();
                        token = self.create_token(RangeInclusive, 3);
                    }
                    token
                }

                // literals

                // identifiers and keywords

                // simple single character tokens
                '(' => self.create_token(OpenParen, 1),
                ')' => self.create_token(CloseParen, 1),
                '[' => self.create_token(OpenBracket, 1),
                ']' => self.create_token(CloseBracket, 1),
                '{' => self.create_token(OpenBrace, 1),
                '}' => self.create_token(CloseBrace, 1),
                ',' => self.create_token(Comma, 1),
                '.' => self.create_token(Dot, 1),
                ';' => self.create_token(Semicolon, 1),

                // simple double character tokens
                '&' => self.with_double('&', And),
                '|' => self.with_double('|', Or),

                // simple single or double character tokens
                '=' => self.with_single_or_double('=', Equal, EqualEqual),
                '!' => self.with_single_or_double('=', Bang, BangEqual),
                '>' => self.with_single_or_double('=', Greater, GreaterEqual),
                '<' => self.with_single_or_double('=', Less, LessEqual),
                '+' => self.with_single_or_double('=', Plus, PlusEqual),
                '-' => self.with_single_or_double('=', Minus, MinusEqual),
                '*' => self.with_single_or_double('=', Star, StarEqual),
                '/' => self.with_single_or_double('=', Slash, SlashEqual),

                // whitespace
                c if is_whitespace(c) => self.next_token(),

                // String literals
                '"' => self.lex_string(),

                // Chars (Characters)
                '\'' => self.lex_char(),

                // Integer literals
                '0'..='9' => self.lex_number(c),

                // Identifiers
                c if c == '_' || unicode_xid::UnicodeXID::is_xid_start(c) => self.get_ident(c),

                c => self.create_token(
                    Error(format!("Unknown character `{}` found in source", c)),
                    1,
                ),
            };
        }
        self.create_token(Eof, 0)
    }
}
