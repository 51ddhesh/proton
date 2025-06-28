use proton::expr::Expr;
use proton::format::pretty_print;

/// Test case to validate pretty printing a number
#[test]
fn test_number() {
    let expr = Expr::Number(3.14); // Create an Expression that represents the number 3.14
    assert_eq!(pretty_print(&expr), "3.14");    
}

/// Test case to validate the pretty printing a variable
#[test]
fn test_variable() {
    let expr = Expr::Variable("x".to_string()); // Create an expression that represents the variable 'x'
    assert_eq!(pretty_print(&expr), "x");
}

/// Test case to validate pretty printing of addition
#[test]
fn test_add() {
    let expr = Expr::Add(Box::new(Expr::Number(2.0)), Box::new(Expr::Variable("y".to_string()))); // Create an expression that represents '(2 + y)'
    assert_eq!(pretty_print(&expr), "(2 + y)");
}

/// Validate prety printing the nested operations
#[test]
fn test_nested() {
    let expr = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)))),
        Box::new(Expr::Variable("z".to_string())),
    );
    assert_eq!(pretty_print(&expr), "((1 + 2) * z)");
}

/// Validate pretty printing the algebraic and trigonometric functions
#[test]
fn test_func() {
    let expr = Expr::Func(
        "sin".to_string(),
        vec![Expr::Variable("theta".to_string())],
    );
    assert_eq!(pretty_print(&expr), "sin(theta)");
}

/// Validate pretty printing functions with multiple arguments
#[test]
fn test_func_multiple_args() {
    let expr = Expr::Func(
        "max".to_string(),
        vec![Expr::Number(1.0), Expr::Number(2.0)],
    );
    assert_eq!(pretty_print(&expr), "max(1, 2)");
}
