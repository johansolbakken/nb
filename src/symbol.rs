use std::{collections::HashMap, error::Error, fs::File, io::Write};

use crate::ast::Node;

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
    pub node: Option<Box<Node>>,
}

pub struct SymbolTable {
    symbols: HashMap<String, Box<Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, kind: SymbolKind) -> Box<Symbol> {
        if self.symbols.contains_key(name) {
            return self.get(&name.to_string());
        }
        let symbol = Symbol {
            name: name.to_string(),
            kind,
            type_: Type::Unknown,
            node: None,
        };
        self.symbols.insert(name.to_string(), Box::new(symbol));
        self.symbols.get(name).unwrap().clone()
    }

    pub fn set_node(&mut self, name: &str, node: Box<Node>) {
        self.symbols.get_mut(name).unwrap().node = Some(node);
    }

    pub fn get(&self, identifier: &String) -> Box<Symbol> {
        self.symbols.get(identifier).unwrap().clone()
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
