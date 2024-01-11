pub mod lexer;

fn main() {
    println!("Welcome to AtomScript!");
    println!("Feel free to type in commands...");

    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = lexer::Lexer::new(input.clone());
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            if token == lexer::Token::Eof {
                break;
            }
        }
        input.clear();
    }
}
