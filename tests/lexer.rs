use meow::{
    lex,
    lexer::token::{Token, TokenKind::*},
};

fn test_tokens(input: &str, expected: &[Token]) {
    let mut lexer = lex(input);

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let next = lexer.next_token();
        if next.kind == Eof {
            break;
        } else {
            tokens.push(next);
        }
    }

    assert_eq!(tokens, expected);
}

#[test]
fn operators() {
    test_tokens(
        r"( ) [ ] { } , . ; && || .. ..= = == ! != > >= < <= + += - -= * *= / /=",
        &vec![
            Token::new(OpenParen, 1, 1),
            Token::new(CloseParen, 1, 3),
            Token::new(OpenBracket, 1, 5),
            Token::new(CloseBracket, 1, 7),
            Token::new(OpenBrace, 1, 9),
            Token::new(CloseBrace, 1, 11),
            Token::new(Comma, 1, 13),
            Token::new(Dot, 1, 15),
            Token::new(Semicolon, 1, 17),
            Token::new(And, 1, 19),
            Token::new(Or, 1, 22),
            Token::new(Range, 1, 25),
            Token::new(RangeInclusive, 1, 28),
            Token::new(Equal, 1, 32),
            Token::new(EqualEqual, 1, 34),
            Token::new(Bang, 1, 37),
            Token::new(BangEqual, 1, 39),
            Token::new(Greater, 1, 42),
            Token::new(GreaterEqual, 1, 44),
            Token::new(Less, 1, 47),
            Token::new(LessEqual, 1, 49),
            Token::new(Plus, 1, 52),
            Token::new(PlusEqual, 1, 54),
            Token::new(Minus, 1, 57),
            Token::new(MinusEqual, 1, 59),
            Token::new(Star, 1, 62),
            Token::new(StarEqual, 1, 64),
            Token::new(Slash, 1, 67),
            Token::new(SlashEqual, 1, 69),
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
