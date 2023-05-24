use tracing::info;

mod ast;
mod lexer;
mod parser;
mod simulate;
mod symbol;
mod utils;

fn main() {
    // tracing_subscriber::fmt::init();

    let file = "examples/si.nb";
    info!("Reading source: {}", file);
    let source = utils::read_file(file).expect("Failed to read file");

    info!("Lexical and syntactic analysis");
    let lexer = lexer::Lexer::new(String::from(source));
    let mut parser = parser::Parser::new(lexer);
    let mut tree = parser.parse();

    info!("Semantic analysis");
    ast::simplify_tree(&mut tree);

    info!("Building symbol table");
    let mut string_list = symbol::StringList::new();
    ast::fill_string_list(&mut tree, &mut string_list);
    let mut symbol_table = symbol::SymbolTable::new();
    ast::find_symbols(&mut tree, &mut symbol_table);

    tree.write_to_file("ast.png")
        .expect("Failed to write ast to file");
    string_list
        .write_to_file("string_list.txt")
        .expect("Failed to write string list to file");
    symbol_table
        .write_to_file("symbol_table.txt")
        .expect("Failed to write symbol table to file");

    info!("Simulating");
    simulate::simulate(&tree, &symbol_table, &string_list);

    info!("Done!");
}
