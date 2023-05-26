use std::{error::Error, fs::File, io::Write};

use tracing::info;

use crate::{ast, lexer, symbol, utils};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    CmpEq,
    Call,
    Ret,
    Push,
    Print,
    Set,
    If,
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
            ast::NodeType::PrintStatement => match &ast.children[0].token {
                Some(token) => match token.token_type() {
                    lexer::TokenType::StringListIndex(string_id) => {
                        let id = self.next_id();
                        let instruction = Instruction {
                            id,
                            opcode: Opcode::Print,
                            operands: vec![Operand::String(*string_id)],
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
                    _ => {
                        panic!("Invalid token type for print statement");
                    }
                },
                None => {
                    let expression = &ast.children[0];
                    seq_id = self.create_basic_blocks(expression, seq_id);
                    let temp_id = self.get_last_temp_id();
                    let id = self.next_id();
                    let instruction = Instruction {
                        id,
                        opcode: Opcode::Print,
                        operands: vec![Operand::Temporary(temp_id)],
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
            },
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
                        "-" => {
                            let id = self.next_id();
                            let temp_id = self.next_temp_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Sub,
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
                        "*" => {
                            let id = self.next_id();
                            let temp_id = self.next_temp_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Mul,
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
                        "/" => {
                            let id = self.next_id();
                            let temp_id = self.next_temp_id();
                            let instruction = Instruction {
                                id,
                                opcode: Opcode::Div,
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
                            lexer::TokenType::Symbol(symbol) => {
                                let id = self.next_id();
                                let temp_id = self.next_temp_id();
                                let instruction = Instruction {
                                    id,
                                    opcode: Opcode::Set,
                                    operands: vec![
                                        Operand::Temporary(temp_id),
                                        Operand::Variable(symbol.clone()),
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
                                panic!("No token found for expression {:?}", token);
                            }
                        }
                    } else {
                        panic!("No token found for expression");
                    }
                }
            }
            ast::NodeType::IfStatement => {
                // empty start block
                let empty_start_id = self.next_id();
                let block = BasicBlock {
                    id: empty_start_id,
                    instructions: Vec::new(),
                    predecessors: vec![seq_id],
                    successors: vec![],
                };
                self.blocks.push(block);

                // generate condition
                let condition = &ast.children[0];
                seq_id = self.create_basic_blocks(condition, empty_start_id);
                let condition_temp = self.get_last_temp_id();
                let condition_id = seq_id;

                // Generate if
                let if_instruction = Instruction {
                    id: self.next_id(),
                    opcode: Opcode::If,
                    operands: vec![Operand::Temporary(condition_temp)],
                };
                let if_id = self.next_id();
                let if_block = BasicBlock {
                    id: if_id,
                    instructions: vec![if_instruction],
                    predecessors: vec![condition_id],
                    successors: vec![],
                };
                self.blocks.push(if_block);

                // generate if block
                let body = &ast.children[1];
                seq_id = self.create_basic_blocks(body, if_id);
                let body_id = seq_id;

                let mut else_block_id = if_id;
                if ast.children.len() == 3 {
                    // generate else block
                    let else_block = &ast.children[2];
                    seq_id = self.create_basic_blocks(else_block, if_id);
                    else_block_id = seq_id;
                }

                let end_id = self.next_id();
                let end_block = BasicBlock {
                    id: end_id,
                    instructions: Vec::new(),
                    predecessors: vec![body_id, else_block_id],
                    successors: vec![],
                };
                self.blocks.push(end_block);
                seq_id = end_id;
            }
            ast::NodeType::Condition => {
                let expr1 = &ast.children[0];
                seq_id = self.create_basic_blocks(expr1, seq_id);
                let expr1_temp = self.get_last_temp_id();

                let expr2 = &ast.children[1];
                seq_id = self.create_basic_blocks(expr2, seq_id);
                let expr2_temp = self.get_last_temp_id();

                match ast.data.as_str() {
                    "==" => {
                        let id = self.next_id();
                        let temp_id = self.next_temp_id();
                        let instruction = Instruction {
                            id,
                            opcode: Opcode::CmpEq,
                            operands: vec![
                                Operand::Temporary(temp_id),
                                Operand::Temporary(expr1_temp),
                                Operand::Temporary(expr2_temp),
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
        let predecessor = self.blocks[self.blocks.len() - 1].id;
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

    pub fn get_successors(&self, id: usize) -> Vec<usize> {
        let mut successors = Vec::new();
        for block in &self.blocks {
            if block.predecessors.contains(&id) {
                successors.push(block.id);
            }
        }
        successors
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
