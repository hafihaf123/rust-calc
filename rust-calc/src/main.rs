use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Display};

use num_bigfloat::BigFloat;
use rust_calc_lib::evaluator::Evaluator;
use rust_calc_lib::numeric::{BuiltinFn, NumericValue};
use rustyline::DefaultEditor;

struct DefaultBuiltins;

impl BuiltinFn<BigFloat> for DefaultBuiltins {
    fn call(&self, name: &str, arg: BigFloat) -> Option<BigFloat> {
        match name {
            "sqrt" => Some(arg.sqrt()),
            "sin" => Some(arg.sin()),
            "cos" => Some(arg.cos()),
            "tan" => Some(arg.tan()),
            "exp" => Some(arg.exp()),
            "ln" => Some(arg.ln()),
            "deg2rad" => {
                let pi = num_bigfloat::PI;
                Some(arg * pi / BigFloat::from_f64(180.0))
            }
            _ => None,
        }
    }

    fn constants(&self) -> HashMap<String, BigFloat> {
        let mut constants = HashMap::new();

        constants.insert(String::from("pi"), num_bigfloat::PI);
        constants.insert(String::from("e"), num_bigfloat::E);

        constants
    }
}

fn repl<N: NumericValue + Debug + Display, F: BuiltinFn<N>>(evaluator: &mut Evaluator<N, F>) {
    println!("RustCalc REPL (type 'exit' to quit)");
    let mut rl = match DefaultEditor::new() {
        Ok(res) => res,
        Err(_) => {
            eprintln!("Error creating a read-line default editor");
            return;
        }
    };

    loop {
        match rl.readline(">>>") {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }
                if input.trim() == "exit" {
                    break;
                }
                match rl.add_history_entry(&input) {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Error adding the history entry");
                        break;
                    }
                };

                match evaluator.parse(&input) {
                    Ok(Some(result)) => println!("{}", result),
                    Ok(None) => {}
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
            Err(_) => eprintln!("Error: failure reading a line from the repl"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut evaluator = Evaluator::new(DefaultBuiltins);
    if args.is_empty() {
        repl(&mut evaluator);
    } else {
        let input = args.join(" ");
        match evaluator.parse(&input) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {}
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}
