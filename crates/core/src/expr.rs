use crate::error::{Error, Result};
use std::collections::HashMap;

/// Evaluates mathematical expressions with parameter substitution
pub struct ExpressionEvaluator {
    parameters: HashMap<String, f64>,
}

impl ExpressionEvaluator {
    pub fn new(parameters: HashMap<String, f64>) -> Self {
        Self { parameters }
    }

    pub fn eval(&self, expr: &str) -> Result<f64> {
        let expr = expr.trim();

        // Check for empty expression
        if expr.is_empty() {
            return Err(Error::ExpressionEval("Empty expression".to_string()));
        }

        // Try parsing as a number first
        if let Ok(val) = expr.parse::<f64>() {
            return Ok(val);
        }

        // Check if it's a parameter reference (with or without $)
        let param_key = if expr.starts_with('$') {
            &expr[1..]
        } else {
            expr
        };

        if let Some(val) = self.parameters.get(param_key) {
            return Ok(*val);
        }

        // Handle basic arithmetic operations
        if let Some(result) = self.eval_binary_op(expr)? {
            return Ok(result);
        }

        Err(Error::ExpressionEval(format!(
            "Cannot evaluate expression: {}",
            expr
        )))
    }

    fn eval_binary_op(&self, expr: &str) -> Result<Option<f64>> {
        // Find operators in reverse precedence order (+ and - before * and /)
        for op in &['+', '-'] {
            if let Some(result) = self.try_split_and_eval(expr, *op)? {
                return Ok(Some(result));
            }
        }

        for op in &['*', '/'] {
            if let Some(result) = self.try_split_and_eval(expr, *op)? {
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    fn try_split_and_eval(&self, expr: &str, op: char) -> Result<Option<f64>> {
        // Find the last occurrence of the operator (for left-associativity)
        let mut depth = 0;
        let mut split_pos = None;

        for (i, ch) in expr.char_indices().rev() {
            match ch {
                ')' => depth += 1,
                '(' => depth -= 1,
                c if c == op && depth == 0 => {
                    split_pos = Some(i);
                    break;
                }
                _ => {}
            }
        }

        if let Some(pos) = split_pos {
            let left = expr[..pos].trim();
            let right = expr[pos + 1..].trim();

            let left_val = self.eval(left)?;
            let right_val = self.eval(right)?;

            let result = match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => {
                    if right_val == 0.0 {
                        return Err(Error::ExpressionEval("Division by zero".to_string()));
                    }
                    left_val / right_val
                }
                _ => return Ok(None),
            };

            return Ok(Some(result));
        }

        // Handle functions
        if let Some(paren_pos) = expr.find('(') {
            let func_name = expr[..paren_pos].trim();
            if expr.ends_with(')') {
                let arg = &expr[paren_pos + 1..expr.len() - 1];
                let arg_val = self.eval(arg)?;

                match func_name {
                    "cos" => return Ok(Some(arg_val.to_radians().cos())),
                    "sin" => return Ok(Some(arg_val.to_radians().sin())),
                    "tan" => return Ok(Some(arg_val.to_radians().tan())),
                    "sqrt" => return Ok(Some(arg_val.sqrt())),
                    "abs" => return Ok(Some(arg_val.abs())),
                    _ => {}
                }
            }
        }

        // Handle parentheses (non-function)
        if expr.starts_with('(')
            && expr.ends_with(')')
            && !expr.contains("cos")
            && !expr.contains("sin")
        {
            return Ok(Some(self.eval(&expr[1..expr.len() - 1])?));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("42.5").unwrap(), 42.5);
        assert_eq!(eval.eval("0").unwrap(), 0.0);
        assert_eq!(eval.eval("-10.5").unwrap(), -10.5);
        assert_eq!(eval.eval(" 42.5 ").unwrap(), 42.5); // Test trimming
    }

    #[test]
    fn test_eval_parameter() {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 100.0);
        params.insert("H".to_string(), 50.0);

        let eval = ExpressionEvaluator::new(params);
        assert_eq!(eval.eval("W").unwrap(), 100.0);
        assert_eq!(eval.eval("H").unwrap(), 50.0);
        assert_eq!(eval.eval(" W ").unwrap(), 100.0); // Test trimming
    }

    #[test]
    fn test_eval_unknown() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("unknown");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ExpressionEval(msg) => assert!(msg.contains("Cannot evaluate")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_eval_addition() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("2 + 3").unwrap(), 5.0);
        assert_eq!(eval.eval("10.5 + 20.5").unwrap(), 31.0);
        assert_eq!(eval.eval("1 + 2 + 3").unwrap(), 6.0);
    }

    #[test]
    fn test_eval_subtraction() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("10 - 3").unwrap(), 7.0);
        assert_eq!(eval.eval("5.5 - 2.5").unwrap(), 3.0);
        assert_eq!(eval.eval("10 - 3 - 2").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_multiplication() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("3 * 4").unwrap(), 12.0);
        assert_eq!(eval.eval("2.5 * 4").unwrap(), 10.0);
        assert_eq!(eval.eval("2 * 3 * 4").unwrap(), 24.0);
    }

    #[test]
    fn test_eval_division() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("10 / 2").unwrap(), 5.0);
        assert_eq!(eval.eval("15 / 3").unwrap(), 5.0);
        assert_eq!(eval.eval("100 / 4 / 5").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_division_by_zero() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("10 / 0");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ExpressionEval(msg) => assert!(msg.contains("Division by zero")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_eval_mixed_operations() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("2 + 3 * 4").unwrap(), 14.0); // Tests precedence
        assert_eq!(eval.eval("10 - 2 * 3").unwrap(), 4.0);
        assert_eq!(eval.eval("10 / 2 + 3").unwrap(), 8.0);
    }

    #[test]
    fn test_eval_parentheses() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(eval.eval("(10 - 2) * 3").unwrap(), 24.0);
        assert_eq!(eval.eval("((2 + 3) * 4)").unwrap(), 20.0);
    }

    #[test]
    fn test_eval_with_parameters() {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 100.0);
        params.insert("H".to_string(), 40.0);
        params.insert("hole_d".to_string(), 5.0);

        let eval = ExpressionEvaluator::new(params);
        assert_eq!(eval.eval("W / 2").unwrap(), 50.0);
        assert_eq!(eval.eval("H / 2").unwrap(), 20.0);
        assert_eq!(eval.eval("W + H").unwrap(), 140.0);
        assert_eq!(eval.eval("W - H").unwrap(), 60.0);
        assert_eq!(eval.eval("(W + H) / 2").unwrap(), 70.0);
    }

    #[test]
    fn test_eval_complex_expression() {
        let mut params = HashMap::new();
        params.insert("a".to_string(), 10.0);
        params.insert("b".to_string(), 5.0);
        params.insert("c".to_string(), 2.0);

        let eval = ExpressionEvaluator::new(params);
        assert_eq!(eval.eval("a + b * c").unwrap(), 20.0);
        assert_eq!(eval.eval("(a + b) * c").unwrap(), 30.0);
        assert_eq!(eval.eval("a / b + c").unwrap(), 4.0);
        assert_eq!(eval.eval("a / (b + c)").unwrap(), 10.0 / 7.0);
    }

    #[test]
    fn test_eval_whitespace_handling() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("  2  +  3  ").unwrap(), 5.0);
        assert_eq!(eval.eval("10*2").unwrap(), 20.0); // No spaces
        assert_eq!(eval.eval(" ( 2 + 3 ) * 4 ").unwrap(), 20.0);
    }

    #[test]
    fn test_try_split_and_eval_no_op() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.try_split_and_eval("42", '+').unwrap(), None);
    }

    #[test]
    fn test_eval_binary_op_no_match() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval_binary_op("42").unwrap(), None);
    }

    #[test]
    fn test_eval_empty_expression() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("");
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::ExpressionEval(msg) => assert!(msg.contains("Empty expression")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_eval_parameter_with_dollar() {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 100.0);
        let eval = ExpressionEvaluator::new(params);
        assert_eq!(eval.eval("$W").unwrap(), 100.0);
    }

    #[test]
    fn test_eval_functions_cos() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("cos(0)").unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_functions_sin() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("sin(0)").unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_functions_tan() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("tan(0)").unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_functions_sqrt() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("sqrt(4)").unwrap(), 2.0);
        assert_eq!(eval.eval("sqrt(9)").unwrap(), 3.0);
    }

    #[test]
    fn test_eval_functions_abs() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("abs(-5)").unwrap(), 5.0);
        assert_eq!(eval.eval("abs(5)").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_functions_with_expressions() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("sqrt(4 + 5)").unwrap(), 3.0);
        assert_eq!(eval.eval("abs(-10 + 5)").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_unknown_function() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        let result = eval.eval("unknown(5)");
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_parentheses_without_function() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("(2 + 3)").unwrap(), 5.0);
        assert_eq!(eval.eval("((2 + 3))").unwrap(), 5.0);
    }

    #[test]
    fn test_eval_operator_precedence() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        // Multiplication should happen before addition
        assert_eq!(eval.eval("2 + 3 * 4").unwrap(), 14.0);
        // Division should happen before subtraction
        assert_eq!(eval.eval("10 - 8 / 2").unwrap(), 6.0);
    }

    #[test]
    fn test_eval_nested_parentheses() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("((2 + 3) * 4)").unwrap(), 20.0);
        assert_eq!(eval.eval("(2 + (3 * 4))").unwrap(), 14.0);
    }

    #[test]
    fn test_eval_parameter_in_expression() {
        let mut params = HashMap::new();
        params.insert("x".to_string(), 10.0);
        let eval = ExpressionEvaluator::new(params);
        assert_eq!(eval.eval("x + 5").unwrap(), 15.0);
        assert_eq!(eval.eval("x * 2").unwrap(), 20.0);
    }

    #[test]
    fn test_eval_negative_numbers() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("-5").unwrap(), -5.0);
        assert_eq!(eval.eval("-10 + 5").unwrap(), -5.0);
    }

    #[test]
    fn test_try_split_and_eval_with_operator() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.try_split_and_eval("2 + 3", '+').unwrap(), Some(5.0));
        assert_eq!(eval.try_split_and_eval("10 - 3", '-').unwrap(), Some(7.0));
        assert_eq!(eval.try_split_and_eval("4 * 5", '*').unwrap(), Some(20.0));
        assert_eq!(eval.try_split_and_eval("20 / 4", '/').unwrap(), Some(5.0));
    }

    #[test]
    fn test_try_split_and_eval_with_parentheses() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        // Should handle operators inside parentheses correctly
        assert_eq!(eval.eval("(2 + 3) * 4").unwrap(), 20.0);
    }

    #[test]
    fn test_try_split_and_eval_unknown_operator() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        // Unknown operator should return None
        assert_eq!(eval.try_split_and_eval("2 % 3", '%').unwrap(), None);
    }

    #[test]
    fn test_eval_decimal_numbers() {
        let eval = ExpressionEvaluator::new(HashMap::new());
        assert_eq!(eval.eval("3.14").unwrap(), 3.14);
        assert_eq!(eval.eval("0.5 + 0.5").unwrap(), 1.0);
    }

    #[test]
    fn test_eval_parameter_trimming() {
        let mut params = HashMap::new();
        params.insert("W".to_string(), 100.0);
        let eval = ExpressionEvaluator::new(params);
        assert_eq!(eval.eval(" $W ").unwrap(), 100.0);
        assert_eq!(eval.eval("$W ").unwrap(), 100.0);
    }
}
