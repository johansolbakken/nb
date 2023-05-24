use std::collections::HashMap;

use tracing::info;

use crate::symbol::Symbol;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Si,
    La,
    Være,
    Er,
    Lik,
    Mindre,
    Større,
    Enn,
    Eller,
    Ellers,
    Og,
    Dersom,
    Gjør,
    Følgende,
    Colon,
    SemiColon,
    Comma,
    Mekanisme,
    StringLiteral(String),
    StringListIndex(usize),
    SymbolRef(Box<Symbol>),
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    Dot,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn lex(&mut self) -> Token {
        self.skip_whitespace();

        if self.peek() == '\0' {
            return Token::new(TokenType::EOF, self.line, self.column);
        }

        if self.peek() == '.' {
            self.advance();
            return Token::new(TokenType::Dot, self.line, self.column);
        }

        if self.peek() == ':' {
            self.advance();
            return Token::new(TokenType::Colon, self.line, self.column);
        }

        if self.peek() == ';' {
            self.advance();
            return Token::new(TokenType::SemiColon, self.line, self.column);
        }

        if self.peek() == ',' {
            self.advance();
            return Token::new(TokenType::Comma, self.line, self.column);
        }

        if self.peek() == '\"' {
            self.advance();
            let start = self.position;
            while self.peek() != '\"' {
                self.advance();
            }
            let end = self.position;
            self.advance();
            return Token::new(
                TokenType::StringLiteral(self.get(start, end)),
                self.line,
                self.column,
            );
        }

        if self.peek().is_numeric() {
            let start = self.position;
            while self.peek().is_numeric() {
                self.advance();
            }
            if self.peek() == '.' && self.peek_next().is_numeric() {
                self.advance();
                while self.peek().is_numeric() {
                    self.advance();
                }
                let end = self.position;
                return Token::new(
                    TokenType::FloatLiteral(self.input[start..end].parse().unwrap()),
                    self.line,
                    self.column,
                );
            }
            let end = self.position;
            return Token::new(
                TokenType::IntLiteral(self.get(start, end).parse().unwrap()),
                self.line,
                self.column,
            );
        }

        // map string to type
        let keywords: HashMap<&str, TokenType> = [
            ("si", TokenType::Si),
            ("la", TokenType::La),
            ("være", TokenType::Være),
            ("er", TokenType::Er),
            ("lik", TokenType::Lik),
            ("mindre", TokenType::Mindre),
            ("større", TokenType::Større),
            ("enn", TokenType::Enn),
            ("ellers", TokenType::Ellers),
            ("og", TokenType::Og),
            ("eller", TokenType::Eller),
            ("dersom", TokenType::Dersom),
            ("gjør", TokenType::Gjør),
            ("følgende", TokenType::Følgende),
            ("mekanisme", TokenType::Mekanisme),
        ]
        .iter()
        .cloned()
        .collect();

        if self.peek().is_alphabetic() {
            let start = self.position;
            while self.peek().is_alphabetic() {
                self.advance();
            }
            let end = self.position;
            let identifier = self
                .input
                .chars()
                .skip(start)
                .take(end - start)
                .collect::<String>();
            if let Some(token_type) = keywords.get(&identifier[..]) {
                return Token::new(token_type.clone(), self.line, self.column);
            }
            return Token::new(TokenType::Identifier(identifier), self.line, self.column);
        }

        panic!("Unexpected character: {}", self.peek());
    }

    fn skip_whitespace(&mut self) {
        while self.peek() == ' ' || self.peek() == '\n' || self.peek() == '\t' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.input.chars().nth(self.position + 1).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let current_char = self.peek();
        self.position += 1;
        self.column += 1;
        current_char
    }

    fn get(&self, start: usize, end: usize) -> String {
        self.input.chars().skip(start).take(end - start).collect()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex();
        if token.token_type() == &TokenType::EOF {
            None
        } else {
            Some(token)
        }
    }
}
