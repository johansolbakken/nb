use tracing::info;

mod ast;
mod lexer;
mod parser;
mod utils;

fn main() {
    tracing_subscriber::fmt::init();

    let source = utils::read_file("examples/si.nb").expect("Failed to read file");
    let lexer = lexer::Lexer::new(String::from(source));
    let mut parser = parser::Parser::new(lexer);
    let mut tree = parser.parse();
    ast::simplify_tree(&mut tree);
    tree.write_to_file("ast.png")
        .expect("Failed to write ast to file");
}
