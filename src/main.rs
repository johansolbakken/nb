use tracing::info;

mod ast;
mod lexer;
mod parser;
mod symbol;
mod utils;
mod simulate;

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

    tree.write_to_file("ast.png")
        .expect("Failed to write ast to file");
    string_list.write_to_file("string_list.txt");

    info!("Simulating");
    simulate::simulate(&tree, &string_list);

    info!("Done!");
}
