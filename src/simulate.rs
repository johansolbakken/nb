use std::collections::HashMap;

use crate::{
    ast::{Node, NodeType},
    symbol::{StringList, SymbolTable},
};

fn evaluate(ast: &Box<Node>, state: &HashMap<String, i64>) -> i64 {
    match ast.node_type {
        NodeType::Expression => {
            let token = &ast.token.as_ref().unwrap();
            match token.token_type() {
                crate::lexer::TokenType::IntLiteral(value) => *value,
                crate::lexer::TokenType::SymbolRef(symbol) => *state.get(&symbol.name).unwrap(),
                _ => 0,
            }
        }
        _ => 0,
    }
}

pub fn simulate(ast: &Box<Node>, symbol_table: &SymbolTable, string_list: &StringList) {
    let mut state: HashMap<String, i64> = HashMap::new();
    for statement in &ast.children {
        match statement.node_type {
            NodeType::PrintStatement => {
                let expression = &statement.children[0];
                match expression.node_type {
                    NodeType::Expression => {
                        let token = &expression.token.as_ref().unwrap();
                        match token.token_type() {
                            crate::lexer::TokenType::StringListIndex(index) => {
                                let string = string_list.get(*index);
                                println!("{}", string);
                            }
                            crate::lexer::TokenType::IntLiteral(value) => {
                                println!("{}", value);
                            }
                            crate::lexer::TokenType::SymbolRef(symbol) => {
                                let value = state.get(&symbol.name).unwrap();
                                println!("{}", value);
                            }
                            crate::lexer::TokenType::Identifier(name) => {
                                let value = state.get(name).unwrap();
                                println!("{}", value);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            NodeType::AssignmentStatement => {
                let identifier = &statement.children[0];
                let expression = &statement.children[1];
                match identifier.node_type {
                    NodeType::Expression => {
                        let token = &identifier.token.as_ref().unwrap();
                        match token.token_type() {
                            crate::lexer::TokenType::SymbolRef(symbol) => {
                                let value = evaluate(expression, &state);
                                state.insert(symbol.name.clone(), value);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
