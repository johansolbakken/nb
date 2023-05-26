use std::{collections::HashMap, error::Error, fs::File, io::Write};

pub type SymbolRef = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unknown,
    Int,
    Float,
    String,
    Bool,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub type_: Type,
}

pub struct SymbolTable {
    symbols: HashMap<SymbolRef, Box<Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, kind: SymbolKind) -> SymbolRef {
        if self.contains_name(name) {
            for (symbol_ref, symbol) in self.symbols.iter() {
                if symbol.name == name {
                    return *symbol_ref;
                }
            }
        }
        let symbol = Symbol {
            name: name.to_string(),
            kind,
            type_: Type::Unknown,
        };
        let symbol_ref = self.symbols.len();
        self.symbols.insert(symbol_ref, Box::new(symbol));
        symbol_ref
    }

    pub fn get(&self, symbol_ref: SymbolRef) -> &Box<Symbol> {
        self.symbols.get(&symbol_ref).unwrap()
    }

    pub fn get_symbol_ref(&self, name: &String) -> Option<SymbolRef> {
        for (symbol_ref, symbol) in self.symbols.iter() {
            if symbol.name == *name {
                return Some(*symbol_ref);
            }
        }
        None
    }

    pub fn contains_name(&self, name: &str) -> bool {
        for symbol in self.symbols.values() {
            if symbol.name == name {
                return true;
            }
        }
        false
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        writeln!(file, "Symbol_table\n------------")?;
        for symbol in self.symbols.values() {
            writeln!(
                file,
                "name = {}, kind = {:?}, type = {:?}",
                symbol.name, symbol.kind, symbol.type_
            )?;
        }
        Ok(())
    }
}

pub struct StringList {
    strings: Vec<String>,
}

impl StringList {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }

    pub fn add(&mut self, s: &str) -> usize {
        self.strings.push(s.to_string());
        self.strings.len() - 1
    }

    pub fn get(&self, index: usize) -> &str {
        &self.strings[index]
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(filename)?;
        writeln!(file, "String_list\n-----------")?;
        for (index, s) in self.strings.iter().enumerate() {
            writeln!(file, "str_{} = \"{}\"", index, s)?;
        }
        Ok(())
    }
}
