use tracing::info;

mod lexer;
mod parser;
mod utils;

fn main() {
    tracing_subscriber::fmt::init();

    let source = utils::read_file("examples/si.nb").expect("Failed to read file");
    let mut lexer = lexer::Lexer::new(String::from(source));
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse();
    ast.print();
}
