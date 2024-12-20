use anyhow::{anyhow, Result};
use big_rational_str::BigRationalExt;
use num::{BigInt, BigRational, FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::collections::BTreeMap;
use std::ops::Div;

pub struct MathExpression {
	/// Supported non-number characters used for splitting the expression
	pub all_operators: Vec<char>,
	/// The operators are evaluated based on their priority. Evaluation goes left-to-right
	pub operators_by_priority: BTreeMap<i32, Vec<char>>,
	pub expression: String,
	pub parsed_expression: Option<Vec<String>>,
}

impl MathExpression {
     /// A constructor for the [MathExpression] struct. It will automatically fill out the correct operators
     ///
     /// # Arguments
     ///
     /// * `expression`: the expression to save into the MathExpression
     ///
     /// # Returns
     ///
     /// `MathExpression`
     /// - a constructed struct
     /// - it has the operators fields pre-filled
     /// - the field parsed_expression is None
     ///
     /// # Examples
     ///
     /// ```
     /// use rust_calc::MathExpression;
     /// let expression_str = "4 + 5 * 8-2";
     /// let expression = MathExpression::new(expression_str);
     /// assert_eq!(expression.expression, expression_str);
     /// ```
	pub fn new(expression: &str) -> Self {
		let all_operators = vec!['+', '-', '*', 'x', '×', '/', '÷', '^', '(', ')'];

		let mut operators_by_priority = BTreeMap::new();

		operators_by_priority.insert(1, vec!['^']);
		operators_by_priority.insert(2, vec!['*', 'x', '×', '/', '÷']);
		operators_by_priority.insert(3, vec!['+', '-']);

		let expression = expression.to_string();

		MathExpression { all_operators, operators_by_priority, expression, parsed_expression: None }
	}

	/// evaluates the expression of a [MathExpression], returning the result as a BigRational
	///
	/// # Returns
	///
	/// `anyhow::Result<BigRational>`
	/// - `Ok(BigRational)`
	///     - when the expression is valid and can be evaluated
	///     - wrapped inside is a BigRational representing the result of the expression
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
	/// # use big_rational_str::BigRationalExt;
	/// # use num::BigRational;
	/// use rust_calc::MathExpression;
	/// let mut expression = MathExpression::new("3 + 5 * 1");
	/// let result = expression.calculate()?;
	/// assert_eq!(result, BigRational::from_dec_str("8.0")?);
	/// # anyhow::Ok(())
	/// ```
	pub fn calculate(&mut self) -> Result<BigRational> {
		self.parse()?;
		// let mut parsed = parse(&handle_parenthesis(expression)?)?;

		self.perform_operations()?;

		match &self.parsed_expression {
			Some(vec) => {
				if vec.len() != 1 {
					return Err(anyhow!("MathExpression operation performing error"));
				}
				Ok(BigRational::from_dec_str(vec.first().unwrap())?)
			}
			None => Err(anyhow!("MathExpression operation performed an empty result"))
		}
	}

	/// parses a string representing a mathematical expression into a `Vec<String>`, splitting the expression on its operators, keeping them in the vector
	///
	/// # Returns
	///
	/// `anyhow::Result<Vec<String>>`
	/// - `Ok(Vec<String>)`
	///     - when the expression has a valid format for parsing
	///     - wrapped inside is a vector containing the individual parts of the expression (*terms* and *operators*) in the correct order
	///
	/// # Errors
	///
	/// the function may return an error in the following scenarios:
	/// - the expression does not have a valid format because of:
	///     - handle_consecutive_operators
	///
	/// # Example
	///
	/// ```
	/// use rust_calc::MathExpression;
	/// let mut expression = MathExpression::new("3 + 5,2 * 2 - 1");
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

		let mut result = Vec::new();

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

	/// performs all defined operations on a [parsed](MathExpression::parse) expression, changing the parsed_expression field
	///
	/// # Returns
	///
	/// `anyhow::Result<()>`
	/// - `Ok(())`
	///     - when performing the operations was successful
	///     - it does not contain a value, but it means the [MathExpression] has been successfully modified
	///
	/// # Errors
	///
	/// the function may return an error in the following scenarios:
	/// - parsing the operations into [MathOperation] failed
	/// - the [MathOperation::calculate] function failed
	///
	/// # Example
	///
	/// ```
	/// use rust_calc::MathExpression;
	/// let mut expression = MathExpression::new("3 + 5.2 * 2 - 8 / 2 + 9");
	/// expression.parse()?;
	/// expression.perform_operations()?;
	/// assert_eq!(expression.parsed_expression.unwrap(), vec!["18.4"]);
	/// # anyhow::Ok(())
	/// ```
	pub fn perform_operations(&mut self) -> Result<()> {
		let vec = match self.parsed_expression.as_mut() {
			Some(vec) => vec,
			None => return Err(anyhow!("The expression isn't parsed"))
		};

		for operators in self.operators_by_priority.values() {
			let mut i = 0;
			while i < vec.len() {
				let s = &vec[i];

				if vec.get(i + 1).is_none() || s.len() != 1 || !operators.contains(&s.chars().next().unwrap()) || i <= 0 {
					i += 1;
					continue;
				}

				let operation = MathOperation::new(s, &vec[i - 1], &vec[i+1])?;

				let partial_result = operation.calculate()?;

				vec[i] = partial_result.to_dec_string();
				vec.remove(i + 1);
				vec.remove(i - 1);
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

pub struct MathOperation<'a> {
	operator: &'a str,
	operand1: BigRational,
	operand2: BigRational
}

impl MathOperation<'_> {
	 /// Creates a new instance of the [MathOperation] struct, taking in the operator and operands as string slices
	 ///
	 /// # Arguments
	 ///
	 /// * `operator`: a string slice representing the operator
	 /// * `operand1`: the first operand
	 /// * `operand2`: the second operand
	 ///
	 /// # Returns
	 ///
	 /// `anyhow::Result<MathOperation>`
	 /// - `Ok(MathOperation)`
	 /// 	- when the instance has been properly created
	 /// 	- inside is wrapped the created instance
	 ///
	 /// # Errors
	 ///
	 /// - the function may return an error in the following scenarios:
	 /// 	- parsing the operands from text to the `BigRational` type fails
	 ///
	 /// # Examples
	 ///
	 /// ```
	 /// # use big_rational_str::BigRationalExt;
	 /// # use num::BigRational;
	 /// use rust_calc::MathOperation;
	 /// let operation = MathOperation::new("+", "8.5", "6")?;
	 /// assert_eq!(operation.calculate()?, BigRational::from_dec_str("14.5")?);
	 /// # anyhow::Ok(())
	 /// ```
	pub fn new<'a>(operator: &'a str, operand1: &str, operand2: &str) -> Result<MathOperation<'a>> {
		let operand1_big_rational = BigRational::from_dec_str(operand1)?;
		let operand2_big_rational = BigRational::from_dec_str(operand2)?;
		Ok(MathOperation {operator, operand1: operand1_big_rational, operand2: operand2_big_rational})
	}

	/// calculates an operation, returning a `BigRational` representing the result of the operation
	///
	/// # Returns
	///
	/// `anyhow::Result<BigRational>`
	/// - `Ok(BigRational)`
	///     - when the operation was successful
	///     - wrapped inside is a BigRational representing the result of the operation
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
	/// # use big_rational_str::BigRationalExt;
	/// # use num::BigRational;
	/// use rust_calc::MathOperation;
	/// let (a, b) = ("4.2", "2");
	/// let operation = MathOperation::new("+", a, b)?;
	/// assert_eq!(operation.calculate()?, BigRational::from_dec_str("6.2")?);
	/// # anyhow::Ok(())
	/// ```
	pub fn calculate(&self) -> Result<BigRational> {
		Ok( match self.operator {
			"+" => &self.operand1 + &self.operand2,
			"-" => &self.operand1 - &self.operand2,
			"*" | "x" | "×" => &self.operand1 * &self.operand2,
			"/" | "÷" => {
				if self.operand2.is_zero() {
					return Err(anyhow!("Cannot divide by zero"));
				}
				&self.operand1 / &self.operand2
			},
			"^" => MathOperation::pow(&self.operand1, &self.operand2)?,
			_ => return Err(anyhow!("Unknown operator: '{}'", self.operator)),
		})
	}

	 /// a function implementing exponentiation for `BigRational` base and exponent
	 ///
	 /// # Arguments
	 ///
	 /// * `base`: a `BigRational` representing the base for the exponentiation
	 /// * `exponent`: a `BigRational` representing the exponent for the exponentiation
	 ///
	 /// # Returns
	 ///
	 /// `anyhow::Result<BigRational>`
	 /// - `Ok(BigRational)`
	 /// 	- when the exponentiation was successful
	 /// 	- inside is wrapped a `BigRational` representing the result of base to the power of exponent
	 ///
	 /// # Errors
	 ///
	 /// - the function may return an error in the following scenarios:
	 /// 	- the operation is invalid
	 /// 		- 0^0
	 /// 		- 0^(-x);
	 /// 	- parsing the exponent numer or denominator to an (unsigned) integer failed
	 ///
	 /// # Examples
	 ///
	 /// ```
	 /// # use num::{BigRational, FromPrimitive};
	 /// use rust_calc::MathOperation;
	 /// let operation = MathOperation::new("^", "2", "3")?;
	 /// assert_eq!(operation.calculate()?, BigRational::from_i32(8).unwrap());
	 /// # anyhow::Ok(())
	 /// ```
	fn pow(base: &BigRational, exponent: &BigRational) -> Result<BigRational> {
		if exponent.is_zero() {
			if base.is_zero() {
				return Err(anyhow!("Invalid operation: 0^0"));
			}
			return Ok(BigRational::one());
		}
		if base.is_zero() {
			if exponent.is_negative() {
				return Err(anyhow!("Invalid operation: cannot raise 0 to negative power"));
			}
			return Ok(BigRational::one());
		}

		let integer_power = match exponent.numer().to_i32() {
			Some(exponent_numer) => base.pow(exponent_numer),
			None => return Err(anyhow!("Failed to parse the exponent numer into an integer")),
		};

	 	let tolerance = BigRational::new(BigInt::one(), BigInt::from(10).pow(15u32));
		let res = MathOperation::nth_root(&integer_power, exponent.denom(), &tolerance)?;
		Ok(res)
	}

	fn nth_root(value: &BigRational, root: &BigInt, tolerance: &BigRational) -> Result<BigRational> {
		if root.is_zero() || root.is_negative() || value.is_zero() || value.is_negative() {
			return Err(anyhow!("Invalid operation: {}-th root of {} is undefined", root, value));
		}

		let root_int = match root.to_i32() {
			Some(root_int) => root_int,
			None => return Err(anyhow!("Failed to parse the root into an integer")),
		};

		let mut x = BigRational::from_f64(value.to_f64().unwrap().powf(1.0 / root.to_f64().unwrap())).unwrap();

		let root_minus_one = root_int - 1;

		loop {
			let x_next = BigRational::new(BigInt::one(), root.clone()) * (BigRational::from_integer(root - BigInt::one())*&x + value / x.pow(root_minus_one));
			if (&x_next - &x).abs() < *tolerance {
				break;
			}
			x = x_next;
		}

		let result_integer_approximation = BigRational::from(x.numer().div(x.denom()));

		if (&x - &result_integer_approximation).abs() < *tolerance {
			return Ok(result_integer_approximation);
		}

		Ok(x)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::random;
	use std::str::FromStr;

	#[test]
	fn test_calculate_operation() -> Result<()> {
		let (a, b) = (BigRational::from_float(random::<f64>()).unwrap(), BigRational::from_float(random::<f64>()).unwrap());

		let mut operation = MathOperation::new("+", &a.to_dec_string(), &b.to_dec_string())?;
		assert_eq!(operation.calculate()?, &a + &b);
		operation.operator = "-";
		assert_eq!(operation.calculate()?, &a - &b);
		operation.operator = "*";
		assert_eq!(operation.calculate()?, &a * &b);
		operation.operator = "x";
		assert_eq!(operation.calculate()?, &a * &b);
		operation.operator = "×";
		assert_eq!(operation.calculate()?, &a * &b);
		operation.operator = "/";
		assert_eq!(operation.calculate()?, &a / &b);
		operation.operator = "÷";
		assert_eq!(operation.calculate()?, &a / &b);
		// operation.operator = "^";
		// assert_eq!(operation.calculate()?, a.powf(b));

		Ok(())
	}

	#[test]
	fn test_parse() -> Result<()> {
		let mut expression = MathExpression::new("1+1");
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1", "+", "1"]);


		expression = MathExpression::new("1-1*5");
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1", "-", "1", "*", "5"]);

		expression = MathExpression::new("1.2x\n3\n");
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1.2", "x", "3"]);

		expression = MathExpression::new("1,2 ×5^ 2");
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["1.2", "×", "5", "^", "2"]);

		expression = MathExpression::new("112/13÷10000000000000000000           ");
		expression.parse()?;
		assert_eq!(expression.parsed_expression.unwrap(), vec!["112", "/", "13", "÷", "10000000000000000000"]);


		Ok(())
	}

	#[test]
	fn test_calculate() -> Result<()> {
		// basic calculations
		let mut expression = MathExpression::new("1+1");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("2")?);
		expression = MathExpression::new("1-1*5");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("-4")?);
		expression = MathExpression::new("1.2x3");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("3.6")?);
		expression = MathExpression::new("6*6/3");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("12")?);

		// big numbers
		expression = MathExpression::new("112/16*1000000000000000000");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("7000000000000000000")?);

		/*/ parenthesis
		expression = MathExpression::new("2+(1)");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("3")?);
		expression = MathExpression::new("2*(-1)");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("-2")?);
		expression = MathExpression::new("2/(-1)");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("-2")?);
		expression = MathExpression::new("12,4*(3+2)+5");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("67")?);
		expression = MathExpression::new("12,4*(3+2)+5/(6-1)+3");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("66")?);
		expression = MathExpression::new("12,4*(2-3)+5");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("-7.4")?);
		// assert_eq!(calculate(BigRational::from_dec_str("12,4*(2/(3-1)-3)+5")?, "-7.4")?); */

		// multiple operators
		expression = MathExpression::new("12,4-+0,4");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("12")?);
		expression = MathExpression::new("12,4++0,4");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("12.8")?);
		expression = MathExpression::new("12,4--0,4");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("12.8")?);
		expression = MathExpression::new("12,4+-0,4");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("12")?);
		expression = MathExpression::new("12,4*-0,5");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("-6.2")?);
		expression = MathExpression::new("12,4/-0,5");
		assert_eq!(expression.calculate()?, BigRational::from_dec_str("-24.8")?);

		Ok(())
	}

	#[test]
	fn test_nth_root() -> Result<()> {
		let tolerance = BigRational::new(BigInt::one(), BigInt::from(10).pow(20u32));

		let (mut a, mut b) = (BigRational::from_str("4")?, BigInt::from_str("2")?);
		assert_eq!(MathOperation::nth_root(&a, &b, &tolerance)?, BigRational::from_dec_str("2")?);

		(a, b) = (BigRational::from_str("256")?, BigInt::from_str("4")?);
		assert_eq!(MathOperation::nth_root(&a, &b, &tolerance)?, BigRational::from_dec_str("4")?);

		(a, b) = (BigRational::from_str("2")?, BigInt::from_str("2")?);
		assert!((MathOperation::nth_root(&a, &b, &tolerance)? - BigRational::from_dec_str("1.414213562373095048801688724209")?).abs() < tolerance);

		Ok(())
	}
}
