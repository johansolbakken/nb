use std::{collections::HashMap, error::Error, fs::File, io::Write};

enum SymbolKind {
    Var,
    Func,
}

enum Type {
    Int,
    Float,
    String,
    Bool,
    Void,
}

struct Symbol {
    name: String,
    kind: SymbolKind,
    type_: Type,
}

struct SymbolTable {
    symbols: HashMap<String, Symbol>,
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
