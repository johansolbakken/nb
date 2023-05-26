use std::collections::HashMap;

use crate::{
    cfg::{BasicBlock, Instruction, Opcode, Operand, CFG},
    symbol::{StringList, SymbolTable},
};

fn simulate_instruction(
    instruction: &Instruction,
    symbol_table: &SymbolTable,
    string_list: &StringList,
    state: &mut HashMap<String, i64>,
) {
    match instruction.opcode {
        Opcode::Print => match &instruction.operands[0] {
            Operand::String(string_index) => {
                let value = string_list.get(string_index.clone());
                println!("{}", value);
            }
            Operand::Variable(symbol_ref) => {
                let symbol = symbol_table.get(symbol_ref.clone());
                let value = state.get(&symbol.name).unwrap();
                println!("{}", value);
            }
            _ => {}
        },
        Opcode::Set => match &instruction.operands[0] {
            Operand::Variable(symbol_ref) => match &instruction.operands[1] {
                Operand::Immediate(value) => {
                    let symbol = symbol_table.get(symbol_ref.clone());
                    let value = value.clone();
                    state.insert(symbol.name.clone(), value);
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

fn simulate_basic_block(
    basic_block: &BasicBlock,
    symbol_table: &SymbolTable,
    string_list: &StringList,
    state: &mut HashMap<String, i64>,
) {
    for instruction in basic_block.get_instructions() {
        simulate_instruction(instruction, symbol_table, string_list, state);
    }
}

pub fn simulate_cfg(cfg: &CFG, symbol_table: &SymbolTable, string_list: &StringList) {
    let mut state: HashMap<String, i64> = HashMap::new();
    let mut id = cfg.entry_block();
    loop {
        let block = cfg.get_block(id);
        simulate_basic_block(block, symbol_table, string_list, &mut state);
        id = cfg.get_successor(id);
        if id == cfg.exit_block() {
            break;
        }
    }
}
