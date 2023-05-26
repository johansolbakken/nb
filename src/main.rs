use tracing::info;

use crate::simulate::simulate_cfg;

mod ast;
mod cfg;
mod lexer;
mod parser;
mod simulate;
mod symbol;
mod utils;

fn main() {
    tracing_subscriber::fmt::init();

    // take file from arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run -- <source>");
        return;
    }

    let file = &args[1];

    info!("Reading source: {}", file);
    let source = utils::read_file(file).expect("Failed to read file");

    info!("Lexical and syntactic analysis");
    let lexer = lexer::Lexer::new(String::from(source));
    let mut parser = parser::Parser::new(lexer);
    let mut tree = parser.parse();

    info!("Semantic analysis");
    ast::simplify_tree(&mut tree);

    info!("Building string list");
    let mut string_list = symbol::StringList::new();
    ast::fill_string_list(&mut tree, &mut string_list);

    info!("Building symbol table");
    let mut symbol_table = symbol::SymbolTable::new();
    ast::find_symbols(&mut tree, &mut symbol_table);

    info!("Building control flow graph");
    let mut cfg = cfg::CFG::new();
    cfg.build(&tree);

    info!("Writing files");
    tree.write_to_file("ast.png")
        .expect("Failed to write ast to file");
    symbol_table
        .write_to_file("symbol_table.txt")
        .expect("Failed to write symbol table to file");
    string_list
        .write_to_file("string_list.txt")
        .expect("Failed to write string list to file");
    cfg.write_to_graphwiz("cfg.png")
        .expect("Failed to write cfg to file");

    info!("Simulating");
    simulate_cfg(&cfg, &symbol_table, &string_list);

    info!("Done!");
}
