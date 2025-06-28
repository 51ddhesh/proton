// Unit tests for the `evaluate` function in the Proton mathematical engine.
// These tests cover literals, variables, arithmetic operations, built-in functions, and error handling.

use std::collections::HashMap;
use proton::expr::Expr;
use proton::eval::evaluate;

/// Test evaluation of a numeric literal.
#[test]
fn test_number_literal() {
    let expr = Expr::Number(42.0);
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), 42.0);
}

/// Test evaluation of a variable when the variable is present in the environment.
#[test]
fn test_variable_lookup() {
    let expr = Expr::Variable("x".to_string());
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 3.14);
    assert_eq!(evaluate(&expr, &vars), 3.14);
}

/// Test evaluation of a variable when the variable is missing from the environment.
/// Should return NaN.
#[test]
fn test_variable_not_found() {
    let expr = Expr::Variable("y".to_string());
    let vars = HashMap::new();
    assert!(evaluate(&expr, &vars).is_nan());
}

/// Test evaluation of an addition expression.
#[test]
fn test_addition() {
    let expr = Expr::Add(Box::new(Expr::Number(2.0)), Box::new(Expr::Number(3.0)));
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), 5.0);
}

/// Test evaluation of a subtraction expression.
#[test]
fn test_subtraction() {
    let expr = Expr::Sub(Box::new(Expr::Number(5.0)), Box::new(Expr::Number(2.0)));
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), 3.0);
}

/// Test evaluation of a multiplication expression.
#[test]
fn test_multiplication() {
    let expr = Expr::Mul(Box::new(Expr::Number(4.0)), Box::new(Expr::Number(2.5)));
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), 10.0);
}

/// Test evaluation of a division expression.
#[test]
fn test_division() {
    let expr = Expr::Div(Box::new(Expr::Number(10.0)), Box::new(Expr::Number(2.0)));
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), 5.0);
}

/// Test evaluation of an exponentiation expression.
#[test]
fn test_exponentiation() {
    let expr = Expr::Pow(Box::new(Expr::Number(2.0)), Box::new(Expr::Number(3.0)));
    let vars = HashMap::new();
    assert_eq!(evaluate(&expr, &vars), 8.0);
}

/// Test evaluation of built-in mathematical functions (sin, cos, sqrt).
#[test]
fn test_builtin_functions() {
    let vars = HashMap::new();

    // Test sine function
    let expr = Expr::Func("sin".to_string(), vec![Expr::Number(0.0)]);
    assert_eq!(evaluate(&expr, &vars), 0.0);

    // Test cosine function
    let expr = Expr::Func("cos".to_string(), vec![Expr::Number(0.0)]);
    assert_eq!(evaluate(&expr, &vars), 1.0);

    // Test square root function
    let expr = Expr::Func("sqrt".to_string(), vec![Expr::Number(9.0)]);
    assert_eq!(evaluate(&expr, &vars), 3.0);
}

/// Test that evaluating an unknown function name results in a panic.
#[test]
#[should_panic(expected = "Unknown Function")]
fn test_unknown_function_panics() {
    let vars = HashMap::new();
    let expr = Expr::Func("unknown".to_string(), vec![Expr::Number(1.0)]);
    evaluate(&expr, &vars);
}