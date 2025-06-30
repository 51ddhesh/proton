use std::collections::HashMap;
use proton::eval::evaluate;
use proton::expr::Expr;
use proton::format::{pretty_print, strip_outer_parantheses};

/// Creating an expression using `proton::expr::Expr`
/// An expression made using `proton::expr::Expr` can be of any type
/// It can be a number, a variable, an algebraic function using trigonometric functions as well
/// In the function below, an expression containing only a number is made 
fn create_expression_number() {
    
}

fn main() {
    create_expression_number();
}






#[allow(unused)]
fn remove_warnings() {
    let vars: HashMap<String, f64> = HashMap::new();
    let expr: Expr = Expr::Number(1.0);
    let result = evaluate(&expr, &vars);
    let ans = pretty_print(&expr);
    let ans = strip_outer_parantheses(ans);
}