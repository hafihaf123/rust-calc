use anyhow::{anyhow, Context, Result};

/// supported non-number characters used for splitting the expression
const OPERATORS: [char; 10] = ['+', '-', '*', 'x', '×', '/', '÷', '^', '(', ')'];

/// operators with the highest priority - they should be evaluated **before** all others
///
/// evaluation goes left-right
const FIRST_PRIORITY_OPERATORS: [char; 1] = ['^'];

/// operators with the second-highest priority
///
/// evaluation goes left-right
const SECOND_PRIORITY_OPERATORS: [char; 5] = ['*', 'x', '×', '/', '÷'];

/// operators with the lowest priority - they should be evaluated **after** all others
///
/// evaluation goes left-right
const LOW_PRIORITY_OPERATORS: [char; 2] = ['+', '-'];

/// evaluates a string as a mathematical expression, returning the result as a 64-bit float
///
/// # Arguments
///
/// - `expression` - a string slice (`&str`) representing a mathematical expression that should be calculated
///
/// # Returns
///
/// this function returns an `anyhow::Result<f64>`
/// - `Ok(f64)`
///     - when the expression is valid and can be evaluated
///     - wrapped inside is a 64-bit floating integer representing the result of the expression
///
/// # Errors
///
/// the function may return an error in the following scenarios:
/// - parsing the expression fails
/// - the operations are invalid or could not be preformed
///
/// # Examples
///
/// ```
/// # use rust_calc::calculate;
/// let expression = "3 + 5 * (2 - 1)";
/// let result = calculate(expression)?;
/// assert_eq!(result, 8.0);
/// ```
pub fn calculate(expression: &str) -> Result<f64> {
    let mut parsed = parse(&handle_parenthesis(expression)?)?;

    perform_operations(&mut parsed, &FIRST_PRIORITY_OPERATORS)?;
    perform_operations(&mut parsed, &SECOND_PRIORITY_OPERATORS)?;
    perform_operations(&mut parsed, &LOW_PRIORITY_OPERATORS)?;

    let result = if parsed.len() == 1 {
        parsed[0].parse::<f64>()?
    } else { 0.0 };

    Ok(result)
}

/// parses a string representing a mathematical expression into a `Vec<String>`, splitting the expression on [OPERATORS], keeping them in the vector
///
/// # Arguments
///
/// - `expression` - a string slice (`&str`) representing a mathematical expression
///
/// # Returns
///
/// the function returns an `anyhow::Result<Vec<String>>`
/// - `Ok(Vec<String>)`
///     - when the expression has a valid format for parsing
///     - wrapped inside is a vector containing the individual parts of the expression (*terms* and *operators*) in the correct order
///
/// # Errors
///
/// the function may return an error in the following scenarios:
/// - the expression does not have a valid format because of:
///     - [consecutive operators](handle_consecutive_operators)
///
/// # Example
///
/// ```
/// # use rust_calc::parse;
/// let expression = "3 + 5,2 * (2 - 1)";
/// let mut result = parse(expression)?;
/// # result = result.iter().map(|s| &s).collect();
/// assert_eq!(result, vec!["3", "+", "5.2", "*", "(", "2", "-", "1", "("]);
/// ```
///
/// # Notes
///
/// - it will automatically convert all `,` to `.` for compatibility
/// - in the example, the `result` variable was silently converted to `Vec<&str>` for the `assert_eq!` macro
pub fn parse(expression: &str) -> Result<Vec<String>> {
    let binding = expression.replace(",", ".");
    let expression_new = binding.trim();

    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in expression_new
        .match_indices(OPERATORS) {
        if last != index {
            result.push(expression_new[last..index].trim().to_string());
        }
        result.push(matched.trim().to_string());
        last = index + matched.len();
    }
    if last < expression_new.len() {
        result.push(expression_new[last..].trim().to_string());
    }

    handle_consecutive_operators(&mut result)?;

    Ok(result)
}

/// performs the desired operations on a [parsed](parse) expression, changing the corresponding vector
///
/// # Arguments
///
/// - `vec` - a mutable pointer to a vector of strings (`&mut Vec<String>`) representing a [parsed](parse) expression
/// - `operators` - a pointer to a character array (`&[char]`) containing the operators, by which the operations are performed
///
/// # Returns
///
/// the function returns an `anyhow::Result<()>`
/// - `Ok(())`
///     - when performing the operations was successful
///     - it does not contain a value, but it means the `vec` argument has been successfully modified
///
/// # Errors
///
/// the function may return an error in the following scenarios:
/// - there is not a valid `f64` around an operator from `operators`
/// - the [calculate_operation] function failed
///
/// # Example
///
/// ```
/// # use rust_calc::{perform_operations, parse};
/// let expression = "3 + 5.2 * 2 - 8 / 2 + 9";
/// let mut vec = parse(expression)?;
/// let operators = ['*', '/', 'x'];
/// perform_operations(&mut vec, &operators)?;
/// # vec = vec.iter().map(|s| &s).collect();
/// assert_eq!(vec, vec!["3", "+", "10.4", "-", "4", "+", "9"]);
/// ```
///
/// # Notes
///
/// - in the example, the `vec` variable was silently converted to `Vec<&str>` for the `assert_eq!` macro
pub fn perform_operations(vec: &mut Vec<String>, operators: &[char]) -> Result<()> {
    let mut i = 0;
    while i < vec.len() {
        let s = &vec[i];

        if vec.get(i+1).is_none() {
            i += 1;
            continue;
        }

        if s.len() == 1 && operators.contains(&s.chars().next().unwrap()) && i > 0 {
            let a = vec[i - 1].parse::<f64>()
                .with_context(|| format!("Your input cannot contain multiple operators without a number between them.\nThis string could not be converted to a number: \"{}\"", vec[i-1]))?;
            let b = vec[i + 1].parse::<f64>()
                .with_context(|| format!("Your input cannot contain multiple operators without a number between them.\nThis string could not be converted to a number: \"{}\"", vec[i+1]))?;

            let partial_result = calculate_operation(s, a, b)?;

            vec[i] = partial_result.to_string();
            vec.remove(i + 1);
            vec.remove(i - 1);
        } else {
        i += 1;
        }
    }

    vec.retain(|x| *x != "");

    Ok(())
}

/// calculates an operation based on an operator and two 64-bit floats, returning a 64-bit float representing the result of the operation
///
/// # Arguments
///
/// - `operator` - a string slice (`&str`) representing an operator for the desired operation
/// - `a` - a 64-bit float (`f64`) representing the left side of the operation
/// - `b` - a 64-bit float (`f64`) representing the right side of the operation
///
/// # Returns
///
/// the function returns an `anyhow::Result<f64>`
/// - `Ok(f64)`
///     - when the operation was successful
///     - wrapped inside is a 64-bit float (`f64`) representing the result of the operation `<a> <operator> <b>`
///
/// # Errors
///
/// the function may return an error in the following scenarios:
/// - the operator is unknown or not defined
/// - a mathematical error occurs, such as division by zero
///
/// # Example
///
/// ```
/// # use rust_calc::calculate_operation;
/// let (a, b) = (4.2f64, 2f64);
/// assert_eq!(calculate_operation("+", a, b)?, 6.2f64);
/// assert_eq!(calculate_operation("-", a, b)?, 2.2f64);
/// assert_eq!(calculate_operation("*", a, b)?, 8.4f64);
/// assert_eq!(calculate_operation("/", a, b)?, 2.1f64);
/// assert_eq!(calculate_operation("^", a, b)?, 17.64f64);
/// ```
pub fn calculate_operation(operator: &str, a: f64, b: f64) -> Result<f64> {
    match operator {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" | "x" | "×" => Ok(a * b),
        "/" | "÷" => {
            if b == 0f64 {
                return Err(anyhow!("Cannot divide by zero"));
            }
            Ok(a / b)
        },
        "^" => Ok(a.powf(b)),
        _ => Err(anyhow!("Unknown operator: '{}'", operator)),
    }
}

fn handle_parenthesis(expression: &str) -> Result<String> {
    if !expression.contains("(") || !expression.contains(")") {
        if expression.contains("(") || expression.contains(")") {
            return Err(anyhow!("Parenthesis not matching"));
        }
        return Ok(expression.to_string());
    }

    if expression.matches("(").count() != expression.matches(")").count() {
        return Err(anyhow!("Parenthesis not matching"));
    }

    let mut result = expression
        .split(|x| x == '(' || x == ')')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut i = 1;
    while i < result.len() {
        let mut parsed_substring = parse(&result[i])?;

        perform_operations(&mut parsed_substring, &FIRST_PRIORITY_OPERATORS)?;
        perform_operations(&mut parsed_substring, &SECOND_PRIORITY_OPERATORS)?;
        perform_operations(&mut parsed_substring, &LOW_PRIORITY_OPERATORS)?;

        if parsed_substring.len() == 1 {
            result[i] = String::from(&parsed_substring[0]);
        }

        i += 2;
    }

    Ok(result.into_iter().collect::<String>())
}

fn handle_consecutive_operators(vec: &mut Vec<String>) -> Result<()> {
    let mut i = 0;
    while i < vec.len() {
        let s = &vec[i];

        if !(s.len() == 1 && OPERATORS.contains(&s.chars().next().unwrap())) {
            i += 1;
            continue;
        }

        if !(vec.get(i+1).is_some() && OPERATORS.contains(&vec[i+1].chars().next().unwrap())) {
            i += 1;
            continue;
        }

        if vec.get(i+2).is_some() && OPERATORS.contains(&vec[i+2].chars().next().unwrap()) {
            return Err(anyhow!("3 or more consecutive operators are not allowed"));
        }

        match (&**s, &*vec[i + 1]) {
            ("-", "+") => {
                vec[i+2] = format!("-{}", &vec[i+2]);
                vec.remove(i);
            },
            ("+", "+") => {
                vec.remove(i);
            },
            ("-", "-") => {
                vec.remove(i);
                vec[i] = "+".to_string();
            },
            ("*", "*") => {
                vec.remove(i);
                vec[i] = "^".to_string();
            },
            (_, "-") => {
                vec[i+1] = format!("-{}", &vec[i+2]);
                vec.remove(i+2);
            },
            (a, b) => {
                return Err(anyhow!("These two consecutive operators are not allowed: '{}' and '{}'", a, b));
            }
        }
        i += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;

    #[test]
    fn test_calculate_operation() -> Result<()> {
        let mut rng = rand::thread_rng();

        let a = rng.random::<f64>();
        let b = rng.random::<f64>();

        assert_eq!(calculate_operation("+", a, b)?, a + b);
        assert_eq!(calculate_operation("-", a, b)?, a - b);
        assert_eq!(calculate_operation("*", a, b)?, a * b);
        assert_eq!(calculate_operation("x", a, b)?, a * b);
        assert_eq!(calculate_operation("×", a, b)?, a * b);
        assert_eq!(calculate_operation("/", a, b)?, a / b);
        assert_eq!(calculate_operation("÷", a, b)?, a / b);
        assert_eq!(calculate_operation("^", a, b)?, a.powf(b));

        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(parse("1+1")?, vec!["1", "+", "1"]);
        assert_eq!(parse("1-1*5")?, vec!["1", "-", "1", "*", "5"]);
        assert_eq!(parse("1.2x3")?, vec!["1.2", "x", "3"]);
        assert_eq!(parse("1,2×5^2")?, vec!["1.2", "×", "5", "^", "2"]);
        assert_eq!(parse("112/13÷10000000000000000000")?, vec!["112", "/", "13", "÷", "10000000000000000000"]);

        Ok(())
    }

    #[test]
    fn test_calculate() -> Result<()> {
        // basic calculations
        assert_eq!(calculate("1+1")?, 2f64);
        assert_eq!(calculate("1-1*5")?, -4f64);
        // assert_eq!(calculate("1.2x3")?, 3.6);
        assert_eq!(calculate("6^2/3")?, 12f64);

        // big numbers
        assert_eq!(calculate("112/16*1000000000000000000")?, 7000000000000000000f64);

        // parenthesis
        assert_eq!(calculate("2+(1)")?, 3f64);
        assert_eq!(calculate("2*(-1)")?, -2f64);
        assert_eq!(calculate("2/(-1)")?, -2f64);
        assert_eq!(calculate("12,4*(3+2)+5")?, 67f64);
        assert_eq!(calculate("12,4*(3+2)+5/(6-1)+3")?, 66f64);
        assert_eq!(calculate("12,4*(2-3)+5")?, -7.4f64);
        // assert_eq!(calculate("12,4*(2/(3-1)-3)+5")?, -7.4f64);

        // multiple operators
        assert_eq!(calculate("12,4-+0,4")?, 12f64);
        assert_eq!(calculate("12,4++0,4")?, 12.8f64);
        assert_eq!(calculate("12,4--0,4")?, 12.8f64);
        assert_eq!(calculate("12,4+-0,4")?, 12f64);
        assert_eq!(calculate("12,4*-0,5")?, -6.2f64);
        assert_eq!(calculate("12,4/-0,5")?, -24.8f64);

        Ok(())
    }
}
