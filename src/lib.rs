use anyhow::{anyhow, Context, Result};
use std::collections::BTreeMap;

pub struct MathExpression {
    /// Supported non-number characters used for splitting the expression
    pub all_operators: Vec<char>,
    /// The operators are evaluated based on their priority. Evaluation goes left-to-right
    pub operators_by_priority: BTreeMap<i32, Vec<char>>,
    pub expression: String,
    pub parsed_expression: Option<Vec<String>>,
}

impl MathExpression {

    pub fn new(expression: &str) -> Result<Self> {
        let all_operators = vec!['+', '-', '*', 'x', '×', '/', '÷', '^', '(', ')'];

        let mut operators_by_priority = BTreeMap::new();

        operators_by_priority.insert(1, vec!['^']);
        operators_by_priority.insert(2, vec!['*', 'x', '×', '/', '÷']);
        operators_by_priority.insert(3, vec!['+', '-']);

        let expression = expression.to_string();

        Ok(MathExpression { all_operators, operators_by_priority, expression, parsed_expression: None })
    }

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
    /// use rust_calc::MathExpression;
    /// let mut expression = MathExpression::new("3 + 5 * 1")?;
    /// let result = expression.calculate()?;
    /// assert_eq!(result, 8.0);
    /// # anyhow::Ok(())
    /// ```
    pub fn calculate(&mut self) -> Result<f64> {
        self.parse()?;
        // let mut parsed = parse(&handle_parenthesis(expression)?)?;

        self.perform_operations()?;

        match &self.parsed_expression {
            Some(vec) => {
                if vec.len() == 1 {
                    return Ok(vec.first().unwrap().parse::<f64>()?);
                }
                Err(anyhow!("MathExpression operation performing error"))
            }
            None => Err(anyhow!("MathExpression operation performed an empty result"))
        }
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
    /// # use rust_calc::MathExpression;
    /// let mut expression = MathExpression::new("3 + 5,2 * 2 - 1")?;
    /// expression.parse()?;
    /// assert_eq!(expression.parsed_expression.unwrap(), vec!["3", "+", "5.2", "*", "2", "-", "1"]);
    /// # anyhow::Ok(())
    /// ```
    ///
    /// # Notes
    ///
    /// - it will automatically convert all `,` to `.` for compatibility
    pub fn parse(&mut self) -> Result<()> {
        let binding = self.expression.replace(",", ".");
        self.expression = binding.trim().to_string();

        let mut result = match self.parsed_expression {
            Some(_) => return Err(anyhow!("Expression already parsed")),
            None => Vec::new()
        };

        let mut last = 0;

        for (index, matched) in self.expression.match_indices(self.all_operators.as_slice()) {
            if last != index {
                result.push(self.expression[last..index].trim().to_string());
            }
            result.push(matched.trim().to_string());
            last = index + matched.len();
        }

        if last < self.expression.len() {
            result.push(self.expression[last..].trim().to_string());
        }

        self.parsed_expression = Some(result);
        self.handle_consecutive_operators()?;

        Ok(())
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
    /// # use rust_calc::MathExpression;
    /// let mut expression = MathExpression::new("3 + 5.2 * 2 - 8 / 2 + 9")?;
    /// expression.parse()?;
    /// expression.perform_operations()?;
    /// assert_eq!(expression.parsed_expression.unwrap(), vec!["18.4"]);
    /// # anyhow::Ok(())
    /// ```
    ///
    /// # Notes
    ///
    /// - in the example, the `vec` variable was silently converted to `Vec<&str>` for the `assert_eq!` macro
    pub fn perform_operations(&mut self) -> Result<()> {
        let vec = match self.parsed_expression.as_mut() {
            Some(vec) => vec,
            None => return Err(anyhow!("The expression isn't parsed"))
        };

        for operators in self.operators_by_priority.values() {
            let mut i = 0;
            while i < vec.len() {
                let s = &vec[i];

                if vec.get(i + 1).is_none() {
                    i += 1;
                    continue;
                }

                if s.len() == 1 && operators.contains(&s.chars().next().unwrap()) && i > 0 {
                    let a = vec[i - 1].parse::<f64>()
                        .with_context(|| format!(r#"\
                            Your input cannot contain multiple operators without a number between them.
                            This string could not be converted to a number: "{}"
                            "#, vec[i - 1]))?;
                    let b = vec[i + 1].parse::<f64>()
                        .with_context(|| format!(r#"\
                            Your input cannot contain multiple operators without a number between them.
                            This string could not be converted to a number: "{}"
                            "#, vec[i - 1]))?;

                    let partial_result = calculate_operation(s, a, b)?;

                    vec[i] = partial_result.to_string();
                    vec.remove(i + 1);
                    vec.remove(i - 1);
                } else {
                    i += 1;
                }
            }
        }

        vec.retain(|x| *x != "");

        Ok(())
    }

    /*fn handle_parenthesis(&mut self) -> Result<()> {
        if !self.expression.contains("(") || !self.expression.contains(")") {
            if self.expression.contains("(") || self.expression.contains(")") {
                return Err(anyhow!("Parenthesis not matching"));
            }
            return Ok(());
        }

        if self.expression.matches("(").count() != self.expression.matches(")").count() {
            return Err(anyhow!("Parenthesis not matching"));
        }

        let mut result = self.expression
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
    }*/

    fn handle_consecutive_operators(&mut self) -> Result<()> {
        let mut i = 0;

        let vec = match self.parsed_expression.as_mut() {
            Some(parsed_expression) => parsed_expression,
            None => return Err(anyhow!("The expression is not parsed")),
        };

        while i < vec.len() {
            let s = &vec[i];

            if !(s.len() == 1 && self.all_operators.contains(&s.chars().next().unwrap())) {
                i += 1;
                continue;
            }

            if !(vec.get(i + 1).is_some() && self.all_operators.contains(&vec[i + 1].chars().next().unwrap())) {
                i += 1;
                continue;
            }

            if vec.get(i+2).is_some() && self.all_operators.contains(&vec[i+2].chars().next().unwrap()) {
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
/// # anyhow::Ok(())
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
        let mut expression = MathExpression::new("1+1")?;
        expression.parse()?;
        assert_eq!(expression.parsed_expression.unwrap(), vec!["1", "+", "1"]);
        
        
        expression = MathExpression::new("1-1*5")?;
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1", "-", "1", "*", "5"]);

        expression = MathExpression::new("1.2x3")?;
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1.2", "x", "3"]);

        expression = MathExpression::new("1,2×5^2")?;
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1.2", "×", "5", "^", "2"]);

        expression = MathExpression::new("112/13÷10000000000000000000")?;
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["112", "/", "13", "÷", "10000000000000000000"]);


        Ok(())
    }

    #[test]
    fn test_calculate() -> Result<()> {
        // basic calculations
        let mut expression = MathExpression::new("1+1")?;
        assert_eq!(expression.calculate()?, 2f64);
        expression = MathExpression::new("1-1*5")?;
        assert_eq!(expression.calculate()?, -4f64);
        // assert_eq!(calculate("1.2x3")?, 3.6);
        expression = MathExpression::new("6^2/3")?;
        assert_eq!(expression.calculate()?, 12f64);

        // big numbers
        expression = MathExpression::new("112/16*1000000000000000000")?;
        assert_eq!(expression.calculate()?, 7000000000000000000f64);

        /*/ parenthesis
        expression = MathExpression::new("2+(1)")?;
        assert_eq!(expression.calculate()?, 3f64);
        expression = MathExpression::new("2*(-1)")?;
        assert_eq!(expression.calculate()?, -2f64);
        expression = MathExpression::new("2/(-1)")?;
        assert_eq!(expression.calculate()?, -2f64);
        expression = MathExpression::new("12,4*(3+2)+5")?;
        assert_eq!(expression.calculate()?, 67f64);
        expression = MathExpression::new("12,4*(3+2)+5/(6-1)+3")?;
        assert_eq!(expression.calculate()?, 66f64);
        expression = MathExpression::new("12,4*(2-3)+5")?;
        assert_eq!(expression.calculate()?, -7.4f64);
        // assert_eq!(calculate("12,4*(2/(3-1)-3)+5")?, -7.4f64); */

        // multiple operators
        expression = MathExpression::new("12,4-+0,4")?;
        assert_eq!(expression.calculate()?, 12f64);
        expression = MathExpression::new("12,4++0,4")?;
        assert_eq!(expression.calculate()?, 12.8f64);
        expression = MathExpression::new("12,4--0,4")?;
        assert_eq!(expression.calculate()?, 12.8f64);
        expression = MathExpression::new("12,4+-0,4")?;
        assert_eq!(expression.calculate()?, 12f64);
        expression = MathExpression::new("12,4*-0,5")?;
        assert_eq!(expression.calculate()?, -6.2f64);
        expression = MathExpression::new("12,4/-0,5")?;
        assert_eq!(expression.calculate()?, -24.8f64);

        Ok(())
    }
}
