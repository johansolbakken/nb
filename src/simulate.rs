use std::{collections::HashMap, panic::PanicInfo};

use tracing::info;

use crate::{
    cfg::{BasicBlock, Instruction, Opcode, Operand, CFG},
    symbol::{StringList, SymbolTable},
};

#[derive(Debug, PartialEq)]
enum BranchDirection {
    Left,
    Right,
}

fn simulate_instruction(
    instruction: &Instruction,
    symbol_table: &SymbolTable,
    string_list: &StringList,
    state: &mut HashMap<String, i64>,
    temporaries: &mut HashMap<usize, i64>,
) -> BranchDirection {
    match instruction.opcode {
        Opcode::Print => match &instruction.operands[0] {
            Operand::Immediate(value) => {
                println!("{}", value);
            }
            Operand::Temporary(temporary_id) => {
                let value = temporaries.get(temporary_id).unwrap();
                println!("{}", value);
            }
            Operand::Variable(symbol_ref) => {
                let symbol = symbol_table.get(symbol_ref.clone());
                let value = state.get(&symbol.name).unwrap();
                println!("{}", value);
            }
            Operand::String(string_id) => {
                let string = string_list.get(string_id.clone());
                println!("{}", string);
            }
            _ => {
                unreachable!();
            }
        },
        Opcode::Set => match &instruction.operands[0] {
            Operand::Variable(symbol_ref) => match &instruction.operands[1] {
                Operand::Immediate(value) => {
                    let symbol = symbol_table.get(symbol_ref.clone());
                    let value = value.clone();
                    state.insert(symbol.name.clone(), value);
                }
                Operand::Temporary(temporary_id) => {
                    let symbol = symbol_table.get(symbol_ref.clone());
                    let value = temporaries.get(temporary_id).unwrap();
                    state.insert(symbol.name.clone(), value.clone());
                }
                _ => {}
            },
            Operand::Temporary(temporary_id) => match &instruction.operands[1] {
                Operand::Immediate(value) => {
                    let value = value.clone();
                    temporaries.insert(temporary_id.clone(), value);
                }
                Operand::Temporary(temporary_id) => {
                    let value = temporaries.get(temporary_id).unwrap().clone();
                    temporaries.insert(temporary_id.clone(), value);
                }
                Operand::Variable(symbol_ref) => {
                    let symbol = symbol_table.get(symbol_ref.clone());
                    let value = state.get(&symbol.name).unwrap().clone();
                    temporaries.insert(temporary_id.clone(), value);
                }
                _ => {}
            },
            _ => {}
        },
        Opcode::Add => {
            let temporary_id = match &instruction.operands[0] {
                Operand::Temporary(temporary_id) => temporary_id.clone(),
                _ => unreachable!(),
            };
            let left = match &instruction.operands[1] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            let right = match &instruction.operands[2] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            temporaries.insert(temporary_id, left + right);
        }
        Opcode::Sub => {
            let temporary_id = match &instruction.operands[0] {
                Operand::Temporary(temporary_id) => temporary_id.clone(),
                _ => unreachable!(),
            };
            let left = match &instruction.operands[1] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            let right = match &instruction.operands[2] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            temporaries.insert(temporary_id, left - right);
        }
        Opcode::Mul => {
            let temporary_id = match &instruction.operands[0] {
                Operand::Temporary(temporary_id) => temporary_id.clone(),
                _ => unreachable!(),
            };
            let left = match &instruction.operands[1] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            let right = match &instruction.operands[2] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            temporaries.insert(temporary_id, left * right);
        }
        Opcode::Div => {
            let temporary_id = match &instruction.operands[0] {
                Operand::Temporary(temporary_id) => temporary_id.clone(),
                _ => unreachable!(),
            };
            let left = match &instruction.operands[1] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            let right = match &instruction.operands[2] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            temporaries.insert(temporary_id, left / right);
        }
        Opcode::If => {
            let target = match &instruction.operands[0] {
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            if target == 0 {
                return BranchDirection::Right;
            }
        }
        Opcode::CmpEq => {
            info!("{:?}", temporaries);
            let temporary_id = match &instruction.operands[0] {
                Operand::Temporary(temporary_id) => temporary_id.clone(),
                _ => unreachable!(),
            };
            let left = match &instruction.operands[1] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            let right = match &instruction.operands[2] {
                Operand::Immediate(value) => value.clone(),
                Operand::Temporary(temporary_id) => temporaries.get(temporary_id).unwrap().clone(),
                _ => unreachable!(),
            };
            if left == right {
                temporaries.insert(temporary_id, 1);
            } else {
                temporaries.insert(temporary_id, 0);
            }
        }
        _ => {
            println!("unimplemented instruction: {:?}", instruction);
        }
    }

    return BranchDirection::Left;
}

fn simulate_basic_block(
    basic_block: &BasicBlock,
    symbol_table: &SymbolTable,
    string_list: &StringList,
    state: &mut HashMap<String, i64>,
    temporaries: &mut HashMap<usize, i64>,
) -> BranchDirection {
    for instruction in basic_block.get_instructions() {
        if simulate_instruction(instruction, symbol_table, string_list, state, temporaries)
            == BranchDirection::Right
        {
            return BranchDirection::Right;
        }
    }
    return BranchDirection::Left;
}

pub fn simulate_cfg(cfg: &CFG, symbol_table: &SymbolTable, string_list: &StringList) {
    let mut state: HashMap<String, i64> = HashMap::new();
    let mut temporaries: HashMap<usize, i64> = HashMap::new();
    let mut id = cfg.entry_block();
    loop {
        let block = cfg.get_block(id);
        let branch_dir = simulate_basic_block(
            block,
            symbol_table,
            string_list,
            &mut state,
            &mut temporaries,
        );
        if branch_dir == BranchDirection::Right {
            id = cfg.get_successors(id)[1];
        } else {
            id = cfg.get_successors(id)[0];
        }
        if id == cfg.exit_block() {
            break;
        }
    }
}
