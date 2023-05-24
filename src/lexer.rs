use tracing::info;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Si,
    StringLiteral(String),
    Identifier(String),
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

        if self.peek() == '\"' {
            self.advance();
            let start = self.position;
            while self.peek() != '\"' {
                self.advance();
            }
            let end = self.position;
            self.advance();
            return Token::new(
                TokenType::StringLiteral(self.input[start..end].to_string()),
                self.line,
                self.column,
            );
        }

        if self.peek().is_alphabetic() {
            let start = self.position;
            while self.peek().is_alphabetic() {
                self.advance();
            }
            let end = self.position;
            let identifier = self.input[start..end].to_string();
            if identifier == "si" {
                return Token::new(TokenType::Si, self.line, self.column);
            }
            return Token::new(TokenType::Identifier(identifier), self.line, self.column);
        }

        panic!("Unexpected character: {}", self.peek());
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let current_char = self.peek();
        self.position += 1;
        self.column += 1;
        current_char
    }
}
