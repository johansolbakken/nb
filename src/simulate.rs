use std::collections::HashMap;

use tracing::info;

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

fn simulate_statement(
    statement: &Box<Node>,
    symbol_table: &SymbolTable,
    string_list: &StringList,
    state: &mut HashMap<String, i64>,
) {
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
        NodeType::IfStatement => {
            let condition = &statement.children[0];
            let body = &statement.children[1];
            match condition.data.as_str() {
                "==" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left == right {
                        simulate_statement(body, symbol_table, string_list, state);
                    }
                }
                "!=" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left != right {
                        simulate_statement(body, symbol_table, string_list, state);
                    }
                }
                "<" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left < right {
                        simulate_statement(body, symbol_table, string_list, state);
                    }
                }
                "<=" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left <= right {
                        simulate_statement(body, symbol_table, string_list, state);
                    }
                }
                ">" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left > right {
                        simulate_statement(body, symbol_table, string_list, state);
                    }
                }
                ">=" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left >= right {
                        simulate_statement(body, symbol_table, string_list, state);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

pub fn simulate(ast: &Box<Node>, symbol_table: &SymbolTable, string_list: &StringList) {
    let mut state: HashMap<String, i64> = HashMap::new();
    for statement in &ast.children {
        simulate_statement(statement, symbol_table, string_list, &mut state);
    }
}
