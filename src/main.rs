use tracing::info;

mod lexer;
mod utils;

fn main() {
    tracing_subscriber::fmt::init();

    let source = utils::read_file("examples/si.nb").expect("Failed to read file");
    let mut lexer = lexer::Lexer::new(String::from(source));
    let mut token = lexer.lex();
    info!(?token);
    while *token.token_type() != lexer::TokenType::EOF {
        token = lexer.lex();
        info!(?token);
    }
}
