use std::io::{stdin, stdout, Write};

use rust_calc::evaluator::Evaluator;
use rust_calc::numeric::BuiltinFn;

struct DefaultBuiltins;

impl BuiltinFn<f64> for DefaultBuiltins {
    fn call(&self, name: &str, arg: f64) -> Option<f64> {
        Some(match name {
            "sin" => arg.sin(),
            "sqrt" => arg.sqrt(),
            "abs" => arg.abs(),
            _ => return None,
        })
    }
}

fn main() {
    println!("RustCalc REPL (type 'exit' to quit)");

    let mut evaluator = Evaluator::new(DefaultBuiltins);
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
            Ok(None) => {}
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
