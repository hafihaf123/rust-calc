use std::io::{stdin, stdout, Write};
use clap::Parser;
use anyhow::Result;
use rust_calc::MathExpression;
use big_rational_str::BigRationalExt;

#[derive(Parser)]
#[command(name = "calc")]
#[command(version, about, long_about = None)]
struct Cli {
    expression: Option<String>
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
            MathExpression::new(input_text.trim())?
        }
        Some(string) => MathExpression::new(&string)?
    };

    let result = expression.calculate()?;
    println!("{}", result.to_dec_string());

    Ok(())
}
