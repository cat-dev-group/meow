//! This file includes the different precedence levels for Meow's prefix and infix operators.

use crate::lexer::token::TokenKind;

/// A struct representing two possible precedence values, a prefix and infix precedence level
pub struct Precedence {
    pub prefix: Option<u8>,
    pub infix: Option<u8>,
}

impl Precedence {
    /// Creates a new `Precedence`, which is used to represent two
    /// possible precedence values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use meow::parser::precedence::Precedence;
    ///
    /// let precedence = Precedence::new(None, Some(3));
    /// ```
    pub fn new(prefix: Option<u8>, infix: Option<u8>) -> Self {
        Self { prefix, infix }
    }
}

/// Returns the precedence level for a given infix operator
pub fn get_precedence(op: &TokenKind) -> Precedence {
    match op {
        // Assignment Operators = += -= *= /=
        TokenKind::Equal
        | TokenKind::PlusEqual
        | TokenKind::MinusEqual
        | TokenKind::StarEqual
        | TokenKind::SlashEqual => Precedence::new(None, Some(1)),

        // Or operator ||
        TokenKind::Or => Precedence::new(None, Some(2)),

        // And operator &&
        TokenKind::And => Precedence::new(None, Some(3)),

        // Equality operators == !=
        TokenKind::EqualEqual | TokenKind::BangEqual => Precedence::new(None, Some(4)),

        // Comparison operators > >= < <=
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
            Precedence::new(None, Some(5))
        }

        // Term operators + -
        TokenKind::Plus => Precedence::new(None, Some(6)),
        TokenKind::Minus => Precedence::new(Some(8), Some(6)),

        // Factor Operators * /
        TokenKind::Star | TokenKind::Slash => Precedence::new(None, Some(7)),

        // Unary bang operator !
        TokenKind::Bang => Precedence::new(Some(8), None),

        // Call operators . ()
        TokenKind::OpenParen | TokenKind::Dot => Precedence::new(None, Some(9)),
        _ => Precedence::new(None, Some(0)),
    }
}
