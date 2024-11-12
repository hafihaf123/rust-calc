use std::io::{stdin, stdout, Write};
use clap::Parser;
use anyhow::Result;
use rust_calc::MathExpression;
use big_rational_str::BigRationalExt;
use num::BigRational;

#[derive(Parser)]
#[command(name = "calc")]
#[command(version, about, long_about = None)]
struct Cli {
    expression: Option<String>,
    #[arg(default_value_t = 10)]
    precision: usize
}

fn main() -> Result<()>{
    let cli = Cli::parse();

    let mut expression = match cli.expression {
        None => {
            println!("Enter expression:");
            let mut input_text = String::new();
            print!("\t> ");
            stdout().flush()?;
            stdin().read_line(&mut input_text)?;
            MathExpression::new(input_text.trim())
        }
        Some(string) => MathExpression::new(&string)
    };

    let result = expression.calculate()?;
    let result_string = big_rational_to_string(&result, cli.precision);
    println!("{}", result_string);

    Ok(())
}

fn big_rational_to_string(number: &BigRational, precision: usize) -> String {
    let mut number_dec_str = number.to_dec_string();

    match number_dec_str.find('(') {
        Some(left_index) => {
            match number_dec_str.find(')') {
                Some(right_index) => {
                    if let Some(parenthesis_substring) = number_dec_str.get(left_index + 1..right_index).map(|s| s.to_string()) {
                        number_dec_str.remove(right_index);
                        number_dec_str.remove(left_index);
                        number_dec_str.push_str(&parenthesis_substring);
                    }
                }
                None => {}
            }
        }
        None => {}
    }

    let mut parts = number_dec_str.split('.');

    let integer_part = parts.next().unwrap_or("0");

    if let Some(decimal_part) = parts.next() {
        let truncated_decimal = &decimal_part[..decimal_part.len().min(precision)];
        format!("{}.{}", integer_part, truncated_decimal)
    } else {
        integer_part.to_string()
    }
}