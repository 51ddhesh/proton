// Unit tests for the pretty-printing functionality of the Proton mathematical engine.
// These tests verify that expressions are formatted as expected for various expression types.

use proton::expr::Expr;
use proton::format::pretty_print;

/// Test pretty-printing of a numeric literal.
#[test]
fn test_pretty_print_number() {
    let expr = Expr::Number(3.14);
    assert_eq!(pretty_print(&expr), "3.14");
}

/// Test pretty-printing of a variable.
#[test]
fn test_pretty_print_variable() {
    let expr = Expr::Variable("x".to_string());
    assert_eq!(pretty_print(&expr), "x");
}

/// Test pretty-printing of an addition expression.
#[test]
fn test_pretty_print_addition() {
    let expr = Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)));
    assert_eq!(pretty_print(&expr), "(1 + 2)");
}

/// Test pretty-printing of a subtraction expression.
#[test]
fn test_pretty_print_subtraction() {
    let expr = Expr::Sub(Box::new(Expr::Number(5.0)), Box::new(Expr::Number(3.0)));
    assert_eq!(pretty_print(&expr), "(5 - 3)");
}

/// Test pretty-printing of a multiplication expression.
#[test]
fn test_pretty_print_multiplication() {
    let expr = Expr::Mul(Box::new(Expr::Number(2.0)), Box::new(Expr::Variable("y".to_string())));
    assert_eq!(pretty_print(&expr), "(2 * y)");
}

/// Test pretty-printing of a division expression.
#[test]
fn test_pretty_print_division() {
    let expr = Expr::Div(Box::new(Expr::Variable("a".to_string())), Box::new(Expr::Number(4.0)));
    assert_eq!(pretty_print(&expr), "(a / 4)");
}

/// Test pretty-printing of an exponentiation expression.
#[test]
fn test_pretty_print_exponentiation() {
    let expr = Expr::Pow(Box::new(Expr::Number(2.0)), Box::new(Expr::Number(8.0)));
    assert_eq!(pretty_print(&expr), "(2 ^ 8)");
}

/// Test pretty-printing of a function application.
#[test]
fn test_pretty_print_function() {
    let expr = Expr::Func("sin".to_string(), vec![Expr::Variable("theta".to_string())]);
    assert_eq!(pretty_print(&expr), "sin(theta)");
}