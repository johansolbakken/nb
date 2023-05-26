// Control Flow Graph

use std::{error::Error, fs::File, io::Write};

use tracing::info;

use crate::{ast, lexer, symbol, utils};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Opcode {
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
    Set,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Immediate(i64),
    Label(usize),
    String(usize),
    Variable(symbol::SymbolRef),
    Temporary(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub id: usize,
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    id: usize,
    instructions: Vec<Instruction>,
    predecessors: Vec<usize>,
    successors: Vec<usize>,
}

impl BasicBlock {
    pub fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CFG {
    blocks: Vec<BasicBlock>,
    entry: usize,
    exit: usize,
    next_id: usize,
    next_temporary_variable_id: usize,
}

impl CFG {
    pub fn new() -> Self {
        let cfg = Self {
            blocks: Vec::new(),
            entry: 0,
            exit: 0,
            next_id: 0,
            next_temporary_variable_id: 0,
        };
        cfg
    }

    pub fn build(&mut self, ast: &Box<ast::Node>) {
        self.add_empty_entry_block();
        self.create_basic_blocks(ast, self.entry);
        self.add_empty_exit_block();
    }

    fn create_basic_blocks(&mut self, ast: &Box<ast::Node>, parent_id: usize) -> usize {
        let mut seq_id = parent_id;
        match ast.node_type {
            ast::NodeType::PrintStatement => {
                let expression = &ast.children[0];
                if let Some(token) = &expression.token {
                    match token.token_type() {
                        lexer::TokenType::StringListIndex(index) => {
                            let id = self.next_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Print,
                                operands: vec![Operand::String(*index)],
                            };
                            let block = BasicBlock {
                                id,
                                instructions: vec![instruction],
                                predecessors: vec![seq_id],
                                successors: vec![],
                            };
                            self.blocks.push(block);
                            seq_id = id;
                        }
                        lexer::TokenType::IntLiteral(value) => {
                            let id = self.next_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Print,
                                operands: vec![Operand::Immediate(*value)],
                            };
                            let block = BasicBlock {
                                id,
                                instructions: vec![instruction],
                                predecessors: vec![seq_id],
                                successors: vec![],
                            };
                            self.blocks.push(block);
                            seq_id = id;
                        }
                        lexer::TokenType::Symbol(symbol) => {
                            let id = self.next_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Print,
                                operands: vec![Operand::Variable(symbol.clone())],
                            };
                            let block = BasicBlock {
                                id,
                                instructions: vec![instruction],
                                predecessors: vec![seq_id],
                                successors: vec![],
                            };
                            self.blocks.push(block);
                            seq_id = id;
                        }
                        _ => {}
                    }
                }
            }
            ast::NodeType::AssignmentStatement => {
                let expression = &ast.children[1];
                seq_id = self.create_basic_blocks(expression, seq_id);

                let temp_id = self.get_last_temp_id();
                let identifier = &ast.children[0];
                if let Some(token) = &identifier.token {
                    match token.token_type() {
                        lexer::TokenType::Symbol(symbol) => {
                            let id = self.next_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Set,
                                operands: vec![
                                    Operand::Variable(symbol.clone()),
                                    Operand::Temporary(temp_id),
                                ],
                            };
                            let block = BasicBlock {
                                id,
                                instructions: vec![instruction],
                                predecessors: vec![seq_id],
                                successors: vec![],
                            };
                            self.blocks.push(block);
                            seq_id = id;
                        }
                        _ => {}
                    }
                }
            }
            ast::NodeType::Expression => {
                if ast.children.len() == 2 {
                    let left = &ast.children[0];
                    seq_id = self.create_basic_blocks(left, seq_id);
                    let left_temp = self.get_last_temp_id();
                    // generate right part of expression
                    let right = &ast.children[1];
                    seq_id = self.create_basic_blocks(right, seq_id);
                    let right_temp = self.get_last_temp_id();

                    // generate instruction
                    match ast.data.as_str() {
                        "+" => {
                            let id = self.next_id();
                            let temp_id = self.next_temp_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Add,
                                operands: vec![
                                    Operand::Temporary(temp_id),
                                    Operand::Temporary(left_temp),
                                    Operand::Temporary(right_temp),
                                ],
                            };
                            let block = BasicBlock {
                                id,
                                instructions: vec![instruction],
                                predecessors: vec![seq_id],
                                successors: vec![],
                            };
                            self.blocks.push(block);
                            seq_id = id;
                        }
                        _ => {
                            panic!("Unknown operator");
                        }
                    }
                } else if ast.children.len() == 0 {
                    if let Some(token) = &ast.token {
                        match token.token_type() {
                            lexer::TokenType::IntLiteral(value) => {
                                let id = self.next_id();
                                let temp_id = self.next_temp_id();
                                let instruction = Instruction {
                                    id,
                                    opcode: Opcode::Set,
                                    operands: vec![
                                        Operand::Temporary(temp_id),
                                        Operand::Immediate(*value),
                                    ],
                                };
                                let block = BasicBlock {
                                    id,
                                    instructions: vec![instruction],
                                    predecessors: vec![seq_id],
                                    successors: vec![],
                                };
                                self.blocks.push(block);
                                seq_id = id;
                            }
                            _ => {
                                panic!("No token found for expression");
                            }
                        }
                    } else {
                        panic!("No token found for expression");
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
            successors: vec![],
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
            successors: Vec::new(),
        };
        self.blocks.push(block);
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn next_temp_id(&mut self) -> usize {
        let id = self.next_temporary_variable_id;
        self.next_temporary_variable_id += 1;
        id
    }

    fn get_last_temp_id(&self) -> usize {
        self.next_temporary_variable_id - 1
    }

    pub fn get_successor(&self, id: usize) -> usize {
        for block in &self.blocks {
            if block.predecessors.contains(&id) {
                return block.id;
            }
        }
        panic!("No successor found for block {}", id);
    }

    pub fn entry_block(&self) -> usize {
        self.entry
    }

    pub fn exit_block(&self) -> usize {
        self.exit
    }

    pub fn get_block(&self, id: usize) -> &BasicBlock {
        &self.blocks[id]
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
                let mut instruction_str = format!("{:?}", instruction);
                instruction_str = instruction_str.replace("\"", "\\\"");
                file.write_all(format!("{}\n", instruction_str).as_bytes())?;
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
