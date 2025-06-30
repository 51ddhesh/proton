#[allow(unused)]
use proton::format::{pretty_print, strip_outer_parantheses};
#[allow(unused)]
use proton::expr::Expr;

/// Example usage of `proton::format::pretty_print` function
/// # Arguments 
/// `exp` - a reference to `proton::expr::Expr` representing an expression
/// # Returns 
/// A formatted `String`
/// Example - create an expression of the form ((x + 2) ^ 2) / y
fn pretty_print_example() {
    let expr: Expr = Expr::Div(
        Box::new(Expr::Pow(
            Box::new(Expr::Add(Box::new(Expr::Variable("x".to_string())), Box::new(Expr::Number(2.0)))),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Variable("y".to_string())),
    );
    let mut result = pretty_print(&expr);
    result = strip_outer_parantheses(result);
    println!("{}", result);
}


fn main() {
    pretty_print_example();
}