use crate::expr::Expr;  // Import the Expr enum from the expr module for handling different expression types.
use std::collections::HashMap;  // Import HashMap to store variables and their associated values for variable lookups.

/// Evaluates a mathematical expression represented by an `Expr` enum.
/// This function supports literals, variables, arithmetic operations, and built-in mathematical functions.
/// 
/// # Parameters
/// - `expr`: A reference to an `Expr` enum that represents the expression to evaluate.
/// - `vars`: A reference to a `HashMap<String, f64>` mapping variable names to their current values.
/// 
/// # Returns
/// The evaluated result as an `f64` value. If the expression is invalid or a variable is not found,
/// appropriate error handling is performed, and the result may be `NaN`.
///
/// # Panics
/// - If an unknown function name is encountered, the function will panic with a descriptive error message.
pub fn evaluate(expr: &Expr, vars: &HashMap<String, f64>) -> f64 {
    match expr {
        // Case 1: Number literal. Simply return the value of the number.
        Expr::Number(n) => *n,

        // Case 2: Variable lookup. If the variable exists in the map, return its value.
        // If not found, print a warning and return `NaN`.
        Expr::Variable(x) => {
            // Attempt to find the variable in the HashMap, or return NaN if not found
            *vars.get(x)
                .unwrap_or_else(|| {
                    // Log a warning for missing variables
                    println!("Warning: variable '{}' not found...", x);
                    &f64::NAN  // Return NaN if the variable is not found
                })
        },
        
        // Case 3: Addition. Recursively evaluate both sides and return the sum.
        Expr::Add(left, right) => evaluate(&left, vars) + evaluate(&right, vars),
        
        // Case 4: Subtraction. Recursively evaluate both sides and return the difference.
        Expr::Sub(left, right) => evaluate(&left, vars) - evaluate(&right, vars),
        
        // Case 5: Multiplication. Recursively evaluate both sides and return the product.
        Expr::Mul(left, right) => evaluate(&left, vars) * evaluate(&right, vars),
        
        // Case 6: Division. Recursively evaluate both sides and return the quotient.
        // A potential division by zero is not handled here, as that would result in infinity or NaN.
        Expr::Div(left, right) => evaluate(&left, vars) / evaluate(&right, vars),

        // Case 7: Exponentiation. Recursively evaluate both sides and return the power.
        Expr::Pow(left, right) => evaluate(&left, vars).powf(evaluate(&right, vars)),

        // Case 8: Function application. Supports various built-in functions such as sin, cos, log, etc.
        Expr::Func(name, args) => {
            // Evaluate arguments for the function and store them in a vector
            let values: Vec<f64> = args.iter().map(|arg| evaluate(arg, vars)).collect();
            
            // Ensure the function expects exactly one argument.
            if values.len() != 1 {
                let mut name = name.clone();
                // Panic if the function doesn't match the expected number of arguments.
                panic!("{:?} expects exactly one argument, but got {}", name.push_str("()"), values.len());
            } else {
                // Match the function name and apply the corresponding mathematical function.
                match name.as_str() {
                    "sin" => values[0].sin(),      // Apply sine function
                    "cos" => values[0].cos(),      // Apply cosine function
                    "tan" => values[0].tan(),      // Apply tangent function
                    "ln" => values[0].ln(),        // Apply natural logarithm (ln)
                    "log10" => values[0].log10(),  // Apply base-10 logarithm
                    "log2" => values[0].log2(),    // Apply base-2 logarithm
                    "sqrt" => values[0].sqrt(),    // Apply square root
                    "max" => values.iter().cloned().fold(std::f64::NEG_INFINITY, f64::max),  // Apply max function (returns the max of all arguments)
                    "min" => values.iter().cloned().fold(std::f64::INFINITY, f64::min),   // Apply min function (returns the min of all arguments)
                    _ => panic!("Unknown Function: {}...", name),  // Panic if an unsupported function name is encountered.
                }
            }
        }
    }
}
