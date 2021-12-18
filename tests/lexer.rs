use meow::{
    lex,
    lexer::token::{Token, TokenKind::{self, *},}
};

fn test_tokens(input: &str, expected: &[TokenKind]) {
    let mut lexer = lex(input);

    let mut tokens: Vec<TokenKind> = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(token.kind);
    }

    assert_eq!(tokens, expected);
}

#[test]
fn operators() {
    test_tokens(
        r"( ) [ ] { } , . ; && || .. ..= = == ! != > >= < <= + += - -= * *= / /=",
        &vec![
            OpenParen,
            CloseParen,
            OpenBracket,
            CloseBracket,
            OpenBrace,
            CloseBrace,
            Comma,
            Dot,
            Semicolon,
            And,
            Or,
            Range,
            RangeInclusive,
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
        ],
    )
}

#[test]
fn numbers() {}

#[test]
fn strings() {}

#[test]
fn identifiers() {}

#[test]
fn keywords() {}

#[test]
fn whitespace() {}

#[test]
fn positions() {
    let input = "";
    let expected: Vec<Token> = vec![];

    let mut lexer = lex(input);

    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next() {
        tokens.push(token);
    }

    assert_eq!(tokens, expected);
}