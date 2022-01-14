use meow::{
    lex,
    lexer::token::{
        Token,
        TokenKind::{self, *},
    },
};

use unindent::unindent;

fn test_tokens(input: &str, expected: &[TokenKind]) {
    let mut lexer = lex(input);

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let next = lexer.next_token();
        if next.kind == TokenKind::Eof {
            break;
        } else {
            tokens.push(next);
        }
    }

    for (token, kind) in tokens.iter().zip(expected) {
        assert_eq!(&token.kind, kind);
    }
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
fn strings() {
    // Single line string
    test_tokens("\"Hello, World\"", &vec![Str("Hello, World".to_string())]);

    // Multiline string
    test_tokens(
        &unindent(
            "\"
    Hello, World
    Foo, Bar
    \"",
        ),
        &vec![Str("\nHello, World\nFoo, Bar\n".to_string())],
    )
}

#[test]
fn chars() {
    test_tokens(
        "'a' 'b' 'c' 'd' 'e'",
        &vec![Char('a'), Char('b'), Char('c'), Char('d'), Char('e')],
    )
}

#[test]
fn numbers() {
    // Test integers
    test_tokens(
        "25 32 43",
        &vec![
            Int("25".to_string()),
            Int("32".to_string()),
            Int("43".to_string()),
        ],
    );

    // Test floats
    test_tokens(
        "3.14159 12.2",
        &vec![Float("3.14159".to_string()), Float("12.2".to_string())],
    );

    // Test too many dots
    test_tokens(
        "4.2.1",
        &vec![Float("4.2".to_string()), Dot, Int("1".to_string())],
    )
}

#[test]
fn identifiers() {
    test_tokens(
        "foo bar baz",
        &vec![
            Ident("foo".to_string()),
            Ident("bar".to_string()),
            Ident("baz".to_string()),
        ],
    )
}

#[test]
fn keywords() {
    test_tokens(
        "class else false for fun if impls import match mut return trait true let while",
        &vec![
            Class, Else, False, For, Fun, If, Impls, Import, Match, Mut, Return, Trait, True, Let,
            While,
        ],
    )
}
