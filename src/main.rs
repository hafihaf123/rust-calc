use std::io::{stdin, stdout, Write};
use clap::Parser;
use anyhow::Result;
use rust_calc::calculate;

#[derive(Parser)]
#[command(name = "calc")]
#[command(version, about, long_about = None)]
struct Cli {
    expression: Option<String>
}

fn main() -> Result<()>{
    let cli = Cli::parse();

    let expression = match cli.expression {
        None => {
            println!("Enter expression:");
            let mut input_text = String::new();
            print!("\t> ");
            stdout().flush()?;
            stdin().read_line(&mut input_text)?;
            input_text.trim().to_string()
        }
        Some(string) => string
    };

    println!("{}", expression);

    let result = calculate(&expression)?;
    println!("{}", result);

    Ok(())
}