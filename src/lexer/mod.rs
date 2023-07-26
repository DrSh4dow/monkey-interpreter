use crate::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: usize, // current position in input (points to current char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: Option<char>, // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lex = Lexer {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some(ch) => match ch {
                '=' => Token {
                    token_type: TokenType::ASSIGN,
                    literal: String::from("="),
                },
                ';' => Token {
                    token_type: TokenType::SEMICOLON,
                    literal: String::from(";"),
                },
                '(' => Token {
                    token_type: TokenType::LPAREN,
                    literal: String::from("("),
                },
                ')' => Token {
                    token_type: TokenType::RPAREN,
                    literal: String::from(")"),
                },
                ',' => Token {
                    token_type: TokenType::COMMA,
                    literal: String::from(","),
                },
                '+' => Token {
                    token_type: TokenType::PLUS,
                    literal: String::from("+"),
                },
                '{' => Token {
                    token_type: TokenType::LBRACE,
                    literal: String::from("{"),
                },
                '}' => Token {
                    token_type: TokenType::RBRACE,
                    literal: String::from("}"),
                },
                _ => Token {
                    token_type: TokenType::EOF,
                    literal: String::from(""),
                },
            },
            None => Token {
                token_type: TokenType::EOF,
                literal: String::from(""),
            },
        };

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}
