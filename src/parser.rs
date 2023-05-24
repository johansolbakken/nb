use tracing::info;

use crate::{
    ast::{Node, NodeType},
    lexer::{Lexer, Token},
};

// program -> statement_list
// statement_list -> statement . statement_list | ε
// statement -> print_statement | assignment_statement | if_statement
// print_statement -> si expression
// if_statement -> dersom condition gjør følgende: statement
// assignment_statement -> la identifier være expression
// expression -> string_literal | identifier | int_literal | float_literal

pub struct Parser {
    lexer: Lexer,
    token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token = lexer.lex();
        Self { lexer, token }
    }

    pub fn parse(&mut self) -> Box<Node> {
        self.program()
    }

    fn is_at_end(&self) -> bool {
        *self.token.token_type() == crate::lexer::TokenType::EOF
    }

    fn advance(&mut self) {
        self.token = self.lexer.lex();
    }

    fn expect(&mut self, token_type: crate::lexer::TokenType) {
        if *self.token.token_type() == token_type {
            self.advance();
        } else {
            panic!(
                "Expected {:?} but found {:?}",
                token_type,
                self.token.token_type()
            );
        }
    }

    fn program(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Program));
        node.children.push(self.statement_list());
        node
    }

    fn statement_list(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::StatementList));
        node.children.push(self.statement());
        self.expect(crate::lexer::TokenType::Dot);
        if !self.is_at_end() {
            node.children.push(self.statement_list());
        }
        node
    }

    fn statement(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Statement));
        match self.token.token_type() {
            crate::lexer::TokenType::Si => {
                node.children.push(self.print_statement());
            }
            crate::lexer::TokenType::La => {
                node.children.push(self.assignment_statement());
            }
            crate::lexer::TokenType::Dersom => {
                node.children.push(self.if_statement());
            }
            _ => panic!("Expected statement but found {:?}", self.token.token_type()),
        }
        node
    }

    fn print_statement(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::PrintStatement));
        self.expect(crate::lexer::TokenType::Si);
        node.children.push(self.expression());
        node
    }

    fn assignment_statement(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::AssignmentStatement));
        self.expect(crate::lexer::TokenType::La);
        node.children.push(self.identifier());
        self.expect(crate::lexer::TokenType::Være);
        node.children.push(self.expression());
        node
    }

    fn if_statement(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::IfStatement));
        self.expect(crate::lexer::TokenType::Dersom);
        node.children.push(self.condition());
        self.expect(crate::lexer::TokenType::Gjør);
        self.expect(crate::lexer::TokenType::Følgende);
        self.expect(crate::lexer::TokenType::Colon);
        node.children.push(self.statement());
        self.expect(crate::lexer::TokenType::Dot);
        if *self.token.token_type() == crate::lexer::TokenType::Ellers {
            self.advance();
            self.expect(crate::lexer::TokenType::Gjør);
            self.expect(crate::lexer::TokenType::Følgende);
            self.expect(crate::lexer::TokenType::Colon);
            node.children.push(self.statement());
        }
        node
    }

    fn condition(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Condition));
        node.children.push(self.expression());
        self.expect(crate::lexer::TokenType::Er);
        match self.token.token_type() {
            crate::lexer::TokenType::Lik => {
                self.advance();
                node.data = "==".to_string();
            }
            crate::lexer::TokenType::Større => {
                self.advance();
                self.expect(crate::lexer::TokenType::Enn);
                match self.token.token_type() {
                    crate::lexer::TokenType::Eller => {
                        self.advance();
                        self.expect(crate::lexer::TokenType::Lik);
                        node.data = ">=".to_string();
                    }
                    _ => node.data = ">".to_string(),
                }
            }
            crate::lexer::TokenType::Mindre => {
                self.advance();
                self.expect(crate::lexer::TokenType::Enn);
                match self.token.token_type() {
                    crate::lexer::TokenType::Eller => {
                        self.advance();
                        self.expect(crate::lexer::TokenType::Lik);
                        node.data = "<=".to_string();
                    }
                    _ => node.data = "<".to_string(),
                }
            }
            _ => panic!(
                "Expected equality, greater or lesser but found {:?}",
                self.token.token_type()
            ),
        }
        node.children.push(self.expression());
        node
    }

    fn expression(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Expression));
        match self.token.token_type() {
            crate::lexer::TokenType::StringListIndex(_) => {
                node.children.push(self.string_literal());
            }
            crate::lexer::TokenType::Identifier(_) => {
                node.children.push(self.identifier());
            }
            crate::lexer::TokenType::IntLiteral(_) => {
                node.children.push(self.int_literal());
            }
            crate::lexer::TokenType::FloatLiteral(_) => {
                node.children.push(self.float_literal());
            }
            crate::lexer::TokenType::StringLiteral(_) => {
                node.children.push(self.string_literal());
            }
            _ => panic!(
                "Expected expression but found {:?}",
                self.token.token_type()
            ),
        }
        node
    }

    fn string_literal(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Expression));
        node.token = Some(self.token.clone());
        self.advance();
        node
    }

    fn int_literal(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Expression));
        node.token = Some(self.token.clone());
        self.advance();
        node
    }

    fn float_literal(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Expression));
        node.token = Some(self.token.clone());
        self.advance();
        node
    }

    fn identifier(&mut self) -> Box<Node> {
        let mut node = Box::new(Node::new(NodeType::Expression));
        match self.token.token_type() {
            crate::lexer::TokenType::Identifier(_) => {}
            _ => panic!(
                "Expected identifier but found {:?}",
                self.token.token_type()
            ),
        }

        node.token = Some(self.token.clone());
        self.advance();
        node
    }
}
