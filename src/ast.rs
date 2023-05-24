use std::{error::Error, fs::File, io::Write};

use crate::{
    lexer::Token,
    symbol::{StringList, SymbolTable},
    utils,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Program,
    StatementList,
    Statement,
    PrintStatement,
    AssignmentStatement,
    Expression,
    IfStatement,
    Condition,
    FunctionDefinition,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub token: Option<Token>,
    pub children: Vec<Box<Node>>,
    pub node_type: NodeType,
    pub data: String,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            token: None,
            children: Vec::new(),
            node_type,
            data: String::new(),
        }
    }

    pub fn print(&self) {
        self.print_aux(0);
    }

    fn print_aux(&self, indent: usize) {
        let indent_str = " ".repeat(indent);
        println!("{}{:?}", indent_str, self.node_type);
        if let Some(token) = &self.token {
            println!("{}{:?}", indent_str, token);
        }
        for child in &self.children {
            child.print_aux(indent + 2);
        }
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        ast_to_graphwiz("ast.dot", self)?;
        utils::graphwiz_to_png("ast.dot", filename).expect("Failed to convert dot file to png");
        std::fs::remove_file("ast.dot").expect("Failed to delete ast.dot");
        Ok(())
    }
}

pub fn simplify_tree(ast: &mut Box<Node>) {
    simplify_tree_aux(ast);
}

fn simplify_tree_aux(ast: &mut Box<Node>) {
    let mut new_children = Vec::new();
    for child in &mut ast.children {
        simplify_tree_aux(child);
        new_children.push(child.clone());
    }
    ast.children = new_children;

    match ast.node_type {
        NodeType::Program => {
            if ast.children.len() == 1 {
                *ast = ast.children.pop().unwrap();
            }
        }
        NodeType::StatementList => {
            if ast.children.len() == 1 {
                *ast = ast.children.pop().unwrap();
            }
            if ast.children.len() == 2 {
                if ast.children[1].node_type == NodeType::StatementList {
                    let mut new_children = Vec::new();
                    new_children.push(ast.children[0].clone());
                    for child in &mut ast.children[1].children {
                        new_children.push(child.clone());
                    }
                    ast.children = new_children;
                }
            }
        }
        NodeType::Statement => {
            if ast.children.len() == 1 {
                *ast = ast.children.pop().unwrap();
            }
        }
        NodeType::PrintStatement => {}
        NodeType::Expression => {
            if ast.children.len() == 1 {
                *ast = ast.children.pop().unwrap();
            }
        }
        NodeType::AssignmentStatement => {}
        NodeType::IfStatement => {}
        NodeType::Condition => {}
        NodeType::FunctionDefinition => {}
    }
}

pub fn fill_string_list(ast: &mut Box<Node>, string_list: &mut StringList) {
    if let NodeType::Expression = ast.node_type {
        if let Some(token) = &ast.token {
            match token.token_type() {
                crate::lexer::TokenType::StringLiteral(value) => {
                    let index = string_list.add(value);
                    ast.token = Some(Token::new(
                        crate::lexer::TokenType::StringListIndex(index),
                        token.line(),
                        token.column(),
                    ));
                }
                _ => {}
            }
        }
    }

    for child in &mut ast.children {
        fill_string_list(child, string_list);
    }
}

pub fn find_symbols(ast: &mut Box<Node>, symbol_table: &mut SymbolTable) {
    if let NodeType::AssignmentStatement = ast.node_type {
        if let Some(token) = &ast.children[0].token {
            match token.token_type() {
                crate::lexer::TokenType::Identifier(name) => {
                    let symbol = symbol_table.add(name, crate::symbol::SymbolKind::Var);
                    ast.children[0].token = Some(Token::new(
                        crate::lexer::TokenType::SymbolRef(symbol),
                        token.line(),
                        token.column(),
                    ));
                    return;
                }
                _ => {}
            }
        }
    }
    if let NodeType::FunctionDefinition = ast.node_type {
        if let Some(token) = &ast.children[0].token {
            match token.token_type() {
                crate::lexer::TokenType::Identifier(name) => {
                    let symbol = symbol_table.add(name, crate::symbol::SymbolKind::Func);
                    ast.children[0].token = Some(Token::new(
                        crate::lexer::TokenType::SymbolRef(symbol),
                        token.line(),
                        token.column(),
                    ));
                    return;
                }
                _ => {}
            }
        }
    }

    match ast.token.clone() {
        Some(token) => match token.token_type() {
            crate::lexer::TokenType::Identifier(name) => {
                let symbol = symbol_table.get(name);
                ast.token = Some(Token::new(
                    crate::lexer::TokenType::SymbolRef(symbol),
                    token.line(),
                    token.column(),
                ));
                return;
            }
            _ => {}
        },
        None => {}
    }

    for child in &mut ast.children {
        find_symbols(child, symbol_table);
    }
}

struct AstWriterState {
    file: File,
    id: usize,
}

pub fn ast_to_graphwiz(filename: &str, ast: &Node) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut state = AstWriterState { file, id: 0 };
    state.file.write_all(b"digraph {\n")?;
    ast_to_graphwiz_aux(&mut state, ast)?;
    state.file.write_all(b"}")?;
    Ok(())
}

fn ast_to_graphwiz_aux(state: &mut AstWriterState, ast: &Node) -> Result<(), Box<dyn Error>> {
    let id = state.id;
    state.id += 1;
    let label = format!("{:?}", ast.node_type);
    state
        .file
        .write_all(format!("    {} [label=\"{}\"];\n", id, label).as_bytes())?;
    if ast.data.len() > 0 {
        let data_id = state.id;
        state.id += 1;
        let mut data_label = ast.data.clone();
        data_label = data_label.replace("\"", "\\\"");
        state
            .file
            .write_all(format!("    {} [label=\"{}\"];\n", data_id, data_label).as_bytes())?;
        // arrow with data label
        state
            .file
            .write_all(format!("    {} -> {} [label=\"data\"];\n", id, data_id).as_bytes())?;
    }
    if let Some(token) = &ast.token {
        let token_id = state.id;
        state.id += 1;
        let mut token_label = format!("{:?}", token.token_type());
        token_label = token_label.replace("\"", "\\\"");
        state
            .file
            .write_all(format!("    {} [label=\"{}\"];\n", token_id, token_label).as_bytes())?;
        state
            .file
            .write_all(format!("    {} -> {};\n", id, token_id).as_bytes())?;
    }
    for child in &ast.children {
        let child_id = state.id;
        state
            .file
            .write_all(format!("    {} -> {};\n", id, child_id).as_bytes())?;
        ast_to_graphwiz_aux(state, child)?;
    }

    Ok(())
}
