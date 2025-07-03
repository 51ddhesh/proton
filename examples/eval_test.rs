use std::collections::HashMap;
use proton::expr::Expr;
use proton::eval::evaluate;
// use proton::format::{pretty_print, strip_outer_parantheses};

fn test() {
    let var1: Expr = Expr::Variable("x".to_string());
    let var2: Expr = Expr::Variable("y".to_string());
    let mut vars: HashMap<String, f64> = HashMap::new();
    vars.insert("x".to_string(), 2.0);
    vars.insert("y".to_string(), 0.0);
    let op: Expr = Expr::Div(
        Box::new(var1),
        Box::new(var2),
    );
    let result = evaluate(&op, &vars);
    println!("{}", result);
}

fn main() {
    test();
}