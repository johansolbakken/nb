use std::{error::Error, fs::File, io::Write};

use crate::{lexer::Token, utils};

#[derive(Debug, Clone)]
pub enum NodeType {
    Program,
    StatementList,
    Statement,
    PrintStatement,
    Expression,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub token: Option<Token>,
    pub children: Vec<Box<Node>>,
    pub node_type: NodeType,
}

impl Node {
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
