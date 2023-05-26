// Control Flow Graph

use std::{error::Error, fs::File, io::Write};

use crate::{
    ast::{Node, NodeType},
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
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Operand {
    Register(usize),
    Immediate(i64),
    Label(usize),
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
    successors: Vec<usize>,
    predecessors: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CFG {
    blocks: Vec<BasicBlock>,
    entry: usize,
    exit: usize,
}

impl CFG {
    pub fn new(ast: &Node) -> Self {
        Self {
            blocks: vec![
                BasicBlock {
                    id: 0,
                    instructions: vec![
                        Instruction {
                            id: 0,
                            opcode: Opcode::Push,
                            operands: vec![Operand::Immediate(0)],
                        },
                        Instruction {
                            id: 1,
                            opcode: Opcode::Jmp,
                            operands: vec![Operand::Label(1)],
                        },
                    ],
                    successors: vec![1],
                    predecessors: vec![],
                },
                BasicBlock {
                    id: 1,
                    instructions: vec![
                        Instruction {
                            id: 0,
                            opcode: Opcode::Push,
                            operands: vec![Operand::Immediate(0)],
                        },
                        Instruction {
                            id: 1,
                            opcode: Opcode::Jmp,
                            operands: vec![Operand::Label(1)],
                        },
                    ],
                    successors: vec![],
                    predecessors: vec![0],
                },
            ],
            entry: 0,
            exit: 1,
        }
    }
    pub fn write_to_graphwiz(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("cfg.dot")?;
        file.write_all(b"digraph {\n")?;
        file.write_all(b"  node [shape=rectangle];\n")?;
        for block in &self.blocks {
            file.write_all(format!("  {} [label=\"", block.id).as_bytes())?;
            for instruction in &block.instructions {
                file.write_all(format!("{:?}\\l", instruction).as_bytes())?;
            }
            file.write_all(b"\"];\n")?;
            for successor in &block.successors {
                file.write_all(format!("  {} -> {};\n", block.id, successor).as_bytes())?;
            }
        }
        file.write_all(b"}")?;
        utils::graphwiz_to_png("cfg.dot", filename).expect("Failed to convert dot file to png");
        std::fs::remove_file("cfg.dot").expect("Failed to delete cfg.dot");
        Ok(())
    }
}
