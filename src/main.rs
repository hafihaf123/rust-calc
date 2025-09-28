use std::io::{Write, stdin, stdout};

use rust_calc::lexer::Lexer;

fn main() {
    let mut input = String::new();
    print!("lexer> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Wrong input");

    let lexer = Lexer::new(&input);
    for token in lexer.map(|res| res.unwrap()) {
        dbg!(token);
    }
}
