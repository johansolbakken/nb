use tracing::info;

mod lexer;

fn main() {
    tracing_subscriber::fmt::init();

    let mut lexer = lexer::Lexer::new(String::from("si \"hello, world!\""));
    let token = lexer.lex();
    info!("token: {:?}", token);
    let token = lexer.lex();
    info!("token: {:?}", token);
}
