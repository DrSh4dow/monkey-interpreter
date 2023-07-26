use crate::token::{Token, TokenType};

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}

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
        self.skip_whitespace();

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
                ch if ch.is_letter() => Token::new(TokenType::IDENT, &self.read_identifier()),
                _ => Token {
                    token_type: TokenType::ILLEGAL,
                    literal: String::from(""),
                },
            },
            None => Token {
                token_type: TokenType::EOF,
                literal: String::from(""),
            },
        };

        // the early return is because read_identifier advances the position until a non-letter is
        // detected and we don't want to advance it again
        if tok.token_type == TokenType::IDENT {
            return tok;
        }

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

    fn read_identifier(&mut self) -> String {
        let initial_position = self.position;

        while let Some(ch) = self.ch {
            if ch.is_letter() {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[initial_position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}
