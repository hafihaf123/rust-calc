use std::io::{Write, stdin, stdout};

use rust_calc::parser::Parser;

fn main() {
    let mut input = String::new();
    print!("parser> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Wrong input");

    let mut parser = Parser::<f64>::new(&input);
    match parser.parse_program() {
        Err(e) => {
            dbg!(e);
        }
        Ok(statements) => statements.iter().for_each(|statement| {
            dbg!(statement);
        }),
    }
}
