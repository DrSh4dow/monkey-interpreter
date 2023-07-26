#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT,
    INT,
    // 1343456
    // Operators
    ASSIGN,
    PLUS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type: match token_type {
                TokenType::IDENT => lookup_ident(literal),
                _ => token_type,
            },
            literal: literal.to_string(),
        }
    }
}

fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}
