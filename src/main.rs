mod lexer;

fn main() {
    let tokens = lexer::Lexer::from_str("2 + 1").extract_tokens();

    println!("{:?}", tokens);
}
