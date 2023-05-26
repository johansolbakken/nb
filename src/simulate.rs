use std::collections::HashMap;

use tracing::info;

use crate::{
    ast::{Node, NodeType},
    symbol::{StringList, SymbolTable},
};

fn evaluate(ast: &Box<Node>, state: &HashMap<String, i64>) -> i64 {
    match ast.node_type {
        NodeType::Expression => {
            if let Some(token) = &ast.token {
                match token.token_type() {
                    crate::lexer::TokenType::IntLiteral(value) => return *value,
                    crate::lexer::TokenType::SymbolRef(symbol) => {
                        return state.get(&symbol.name).unwrap().clone();
                    }
                    _ => {}
                }
            }

            if ast.children.len() == 2 {
                let left = evaluate(&ast.children[0], state);
                let right = evaluate(&ast.children[1], state);
                return match ast.data.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => 0,
                };
            }
        }
        _ => return 0,
    }
    return 0;
}

fn simulate_statement(
    statement: &Box<Node>,
    symbol_table: &SymbolTable,
    string_list: &StringList,
    state: &mut HashMap<String, i64>,
) {
    match statement.node_type {
        NodeType::Statement => {
            for child in &statement.children {
                simulate_statement(child, symbol_table, string_list, state);
            }
        }
        NodeType::FunctionCall => {
            let identifier = &statement.children[0];
            match identifier.node_type {
                NodeType::Expression => {
                    let token = &identifier.token.as_ref().unwrap();
                    match token.token_type() {
                        crate::lexer::TokenType::SymbolRef(symbol) => {
                            let symbol = symbol_table.get(&symbol.name);
                            let node = symbol.node.as_ref().unwrap();
                            simulate_statement(node, symbol_table, string_list, state);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        NodeType::PrintStatement => {
            let expression = &statement.children[0];
            match expression.node_type {
                NodeType::Expression => {
                    let token = &expression.token.as_ref().unwrap();
                    match token.token_type() {
                        crate::lexer::TokenType::StringListIndex(index) => {
                            let value = string_list.get(*index);
                            println!("{}", value);
                        }
                        _ => {
                            let value = evaluate(expression, &state);
                            println!("{}", value);
                        }
                    }
                }
                _ => {}
            }
        }
        NodeType::AssignmentStatement => {
            let identifier = &statement.children[0];
            let expression = &statement.children[1];
            match identifier.token.as_ref().unwrap().token_type() {
                crate::lexer::TokenType::SymbolRef(symbol) => {
                    let value = evaluate(expression, &state);
                    state.insert(symbol.name.clone(), value);
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
                    } else {
                        if statement.children.len() == 3 {
                            let else_body = &statement.children[2];
                            simulate_statement(else_body, symbol_table, string_list, state);
                        }
                    }
                }
                "!=" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left != right {
                        simulate_statement(body, symbol_table, string_list, state);
                    } else {
                        if statement.children.len() == 3 {
                            let else_body = &statement.children[2];
                            simulate_statement(else_body, symbol_table, string_list, state);
                        }
                    }
                }
                "<" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left < right {
                        simulate_statement(body, symbol_table, string_list, state);
                    } else {
                        if statement.children.len() == 3 {
                            let else_body = &statement.children[2];
                            simulate_statement(else_body, symbol_table, string_list, state);
                        }
                    }
                }
                "<=" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left <= right {
                        simulate_statement(body, symbol_table, string_list, state);
                    } else {
                        if statement.children.len() == 3 {
                            let else_body = &statement.children[2];
                            simulate_statement(else_body, symbol_table, string_list, state);
                        }
                    }
                }
                ">" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left > right {
                        simulate_statement(body, symbol_table, string_list, state);
                    } else {
                        if statement.children.len() == 3 {
                            let else_body = &statement.children[2];
                            simulate_statement(else_body, symbol_table, string_list, state);
                        }
                    }
                }
                ">=" => {
                    let left = evaluate(&condition.children[0], &state);
                    let right = evaluate(&condition.children[1], &state);
                    if left >= right {
                        simulate_statement(body, symbol_table, string_list, state);
                    } else {
                        if statement.children.len() == 3 {
                            let else_body = &statement.children[2];
                            simulate_statement(else_body, symbol_table, string_list, state);
                        }
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
