use anyhow::{Result, anyhow, Context};

pub fn calculate(expression: &str) -> Result<f64> {
    let operators = ['+', '-', '*', 'x', '×', '/', '÷'];
    let priority_operators = ['*', 'x', '×', '/', '÷'];
    let non_priority_operators = ['+', '-'];

    let mut parsed = parse(expression, &operators)?;

    perform_operations(&mut parsed, &priority_operators)?;
    perform_operations(&mut parsed, &non_priority_operators)?;

    let result = if parsed.len() == 1 {
        parsed[0].parse::<f64>()?
    } else { 0.0 };

    Ok(result)
}

fn parse(expression: &str, operators: &[char]) -> Result<Vec<String>> {
    let binding = expression.replace(",", ".");
    let expression_new = binding.trim();

    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in expression_new
        .match_indices(operators) {
        if last != index {
            result.push(expression_new[last..index].trim().to_string());
        }
        result.push(matched.trim().to_string());
        last = index + matched.len();
    }
    if last < expression_new.len() {
        result.push(expression_new[last..].trim().to_string());
    }

    Ok(result)
}

fn perform_operations(vec: &mut Vec<String>, operators: &[char]) -> Result<()> {
    let mut i = 0;
    while i < vec.len() {
        let s = &vec[i];

        if s.len() == 1 && operators.contains(&s.chars().next().unwrap()) && i > 0 {
            let a = vec[i - 1].parse::<f64>()
                .with_context(|| format!("Your input cannot contain multiple operators without a number between them.\nThis string could not be converted to a number: \"{}\"", vec[i-1]))?;
            let b = vec[i + 1].parse::<f64>()
                .with_context(|| format!("Your input cannot contain multiple operators without a number between them.\nThis string could not be converted to a number: \"{}\"", vec[i+1]))?;

            let partial_result = calculate_operation(s, a, b)?;

            vec[i] = partial_result.to_string();
            vec.remove(i + 1);
            vec.remove(i - 1);
        }

        i += 1;
    }

    vec.retain(|x| *x != "");

    Ok(())
}

fn calculate_operation(operation: &str, a: f64, b: f64) -> Result<f64> {
    match operation {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" | "x" | "×" => Ok(a * b),
        "/" | "÷" => Ok(a / b),
        _ => Err(anyhow!("Unknown operation {}", operation)),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn _test_1() {
    }
}
