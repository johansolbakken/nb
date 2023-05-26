// Control Flow Graph

use std::{error::Error, fs::File, io::Write};

use tracing::info;

use crate::{
    ast::{Node, NodeType},
    lexer::{Token, TokenType},
    utils,
};

#[derive(Debug, Clone, PartialEq, Copy)]
enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Cmp,
    Jmp,
    Jz,
    Jnz,
    Jl,
    Jle,
    Jg,
    Jge,
    Call,
    Ret,
    Push,
    Print,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Operand {
    Immediate(i64),
    Label(usize),
    String(usize),
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    id: usize,
    opcode: Opcode,
    operands: Vec<Operand>,
}

#[derive(Debug, Clone, PartialEq)]
struct BasicBlock {
    id: usize,
    instructions: Vec<Instruction>,
    predecessors: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CFG {
    blocks: Vec<BasicBlock>,
    entry: usize,
    exit: usize,
    next_id: usize,
}

impl CFG {
    pub fn new() -> Self {
        let cfg = Self {
            blocks: Vec::new(),
            entry: 0,
            exit: 0,
            next_id: 0,
        };
        cfg
    }

    pub fn build(&mut self, ast: &Box<Node>) {
        self.add_empty_entry_block();
        self.create_basic_blocks(ast, self.entry);
        self.add_empty_exit_block();
    }

    fn create_basic_blocks(&mut self, ast: &Box<Node>, parent_id: usize) -> usize {
        let mut seq_id = parent_id;
        match ast.node_type {
            NodeType::PrintStatement => {
                let expression = &ast.children[0];
                if let Some(token) = &expression.token {
                    match token.token_type() {
                        TokenType::StringListIndex(index) => {
                            let id = self.next_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Print,
                                operands: vec![Operand::String(*index)],
                            };
                            let block = BasicBlock {
                                id,
                                instructions: vec![instruction],
                                predecessors: vec![parent_id],
                            };
                            self.blocks.push(block);
                            seq_id = id;
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                for child in &ast.children {
                    seq_id = self.create_basic_blocks(child, seq_id);
                }
            }
        }
        return seq_id;
    }

    fn add_empty_entry_block(&mut self) {
        let id = self.next_id();
        self.entry = id;
        let block = BasicBlock {
            id,
            instructions: Vec::new(),
            predecessors: Vec::new(),
        };
        self.blocks.push(block);
    }

    fn add_empty_exit_block(&mut self) {
        let id = self.next_id();
        self.exit = id;
        let predecessor = self.blocks.len() - 1;
        let block = BasicBlock {
            id,
            instructions: Vec::new(),
            predecessors: vec![predecessor],
        };
        self.blocks.push(block);
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn write_to_graphwiz(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("cfg.dot")?;
        file.write_all(b"digraph {\n")?;
        file.write_all(b"  node [shape=rectangle];\n")?;

        // draw basic blocks with block id and instructions and draw edges between blocks
        for block in &self.blocks {
            file.write_all(format!("  {} [label=\"", block.id).as_bytes())?;
            if block.id == self.entry {
                file.write_all(b"entry\n")?;
            } else if block.id == self.exit {
                file.write_all(b"exit\n")?;
            } else {
                file.write_all(format!("block {}\n", block.id).as_bytes())?;
            }
            for instruction in &block.instructions {
                file.write_all(format!("{:?}\n", instruction).as_bytes())?;
            }
            file.write_all(b"\"];\n")?;
            for predecessor in &block.predecessors {
                file.write_all(format!("  {} -> {};\n", predecessor, block.id).as_bytes())?;
            }
        }

        file.write_all(b"}")?;
        utils::graphwiz_to_png("cfg.dot", filename).expect("Failed to convert dot file to png");
        std::fs::remove_file("cfg.dot").expect("Failed to delete cfg.dot");
        Ok(())
    }
}
