use crate::{
    ast::{Node, NodeType},
    lexer::{Lexer, Token},
};

// program -> statement_list
// statement_list -> statement statement_list | ε
// statement -> print_statement .
// print_statement -> si expression
// expression -> string_literal

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
        let mut node = Box::new(Node {
            token: None,
            children: Vec::new(),
            node_type: NodeType::Program,
        });
        node.children.push(self.statement_list());
        node
    }

    fn statement_list(&mut self) -> Box<Node> {
        let mut node = Box::new(Node {
            token: None,
            children: Vec::new(),
            node_type: NodeType::StatementList,
        });
        node.children.push(self.statement());
        if !self.is_at_end() {
            node.children.push(self.statement_list());
        }
        node
    }

    fn statement(&mut self) -> Box<Node> {
        let mut node = Box::new(Node {
            token: None,
            children: Vec::new(),
            node_type: NodeType::Statement,
        });
        node.children.push(self.print_statement());
        node
    }

    fn print_statement(&mut self) -> Box<Node> {
        let mut node = Box::new(Node {
            token: None,
            children: Vec::new(),
            node_type: NodeType::PrintStatement,
        });
        self.expect(crate::lexer::TokenType::Si);
        node.children.push(self.expression());
        self.expect(crate::lexer::TokenType::Dot);
        node
    }

    fn expression(&mut self) -> Box<Node> {
        let mut node = Box::new(Node {
            token: None,
            children: Vec::new(),
            node_type: NodeType::Expression,
        });
        node.children.push(self.string_literal());
        node
    }

    fn string_literal(&mut self) -> Box<Node> {
        let mut node = Box::new(Node {
            token: None,
            children: Vec::new(),
            node_type: NodeType::Expression,
        });
        node.token = Some(self.token.clone());
        self.advance();
        node
    }
}
