use std::io::{Write, stdin, stdout};

use rust_calc::evaluator::Evaluator;

fn main() {
    println!("RustCalc REPL (type 'exit' to quit)");

    let mut evaluator = Evaluator::<f64>::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        if stdin().read_line(&mut input).is_err() {
            break;
        }

        if input.trim() == "exit" {
            break;
        }

        match evaluator.parse(&input) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => println!(),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
