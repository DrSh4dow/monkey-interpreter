use monkey_interpreter::{lexer::*, token::*};

#[test]
fn test_lexer_special_characters() {
    let input = "=+(){},;";

    let tests = vec![
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::PLUS, "+"),
        Token::new(TokenType::LPAREN, "("),
        Token::new(TokenType::RPAREN, ")"),
        Token::new(TokenType::LBRACE, "{"),
        Token::new(TokenType::RBRACE, "}"),
        Token::new(TokenType::COMMA, ","),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::EOF, ""),
    ];

    let mut lex = Lexer::new(input);

    for t in tests {
        let tok = lex.next_token();

        assert_eq!(tok.token_type, t.token_type);
    }
}

#[test]
fn test_lexer_simple_function() {
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
";

    let tests = vec![
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::INT,
            literal: String::from("10"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: String::from("{"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("x"),
        },
        Token {
            token_type: TokenType::PLUS,
            literal: String::from("+"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("y"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: String::from("}"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("result"),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("add"),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: String::from("("),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("five"),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: String::from(","),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: String::from("ten"),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: String::from(")"),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        Token {
            token_type: TokenType::EOF,
            literal: String::from(""),
        },
    ];

    let mut lex = Lexer::new(input);

    for t in tests {
        let tok = lex.next_token();

        assert_eq!(tok.token_type, t.token_type);
    }
}
