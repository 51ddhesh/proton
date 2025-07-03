// use std::collections::HashMap;
// use proton::eval::evaluate;

/// Example usage of a `proton::expr::Expr::Number`
/// exp is a proton::expr::Expr::Number
/// the `proton::format::pretty_print` function returns a formatted string to `s`
/// Expected Output: `2`
// fn number_example() {
//     let exp: Expr = Expr::Number(2.0);
//     let s = pretty_print(&exp);
//       // Expected output is 2
//     println!("{}", s);
// }

/// Example usage of a `proton::expr::Expr::Variable`
/// `exp` is a `proton::expr::Expr::Variable`
/// The `proton::eval::evaluate` function evaluates the variable using the provided environment `vars`
/// Expected Output: `5`
// fn variable_example() {
//     let mut vars: HashMap<String, f64> = HashMap::new();
//     let exp: Expr = Expr::Variable("x".to_string());
//     vars.insert("x".to_string(), 5.0);
//     let result = evaluate(&exp, &vars);
//     println!("{}", result);
// }


/// Example usage of `proton::format::pretty_print` function
/// # Arguments 
/// `exp` - a reference to `proton::expr::Expr` representing an expression
/// # Returns 
/// A formatted `String`
/// Example - create an expression of the form ((x + 2) ^ 2) / y
// fn pretty_print_example() {
//     let expr: Expr = Expr::Div(
//         Box::new(Expr::Pow(
//             Box::new(Expr::Add(Box::new(Expr::Variable("x".to_string())), Box::new(Expr::Number(2.0)))),
//             Box::new(Expr::Number(2.0)),
//         )),
//         Box::new(Expr::Variable("y".to_string())),
//     );
//     let mut result = pretty_print(&expr);
//     result = strip_outer_parantheses(result);
//     println!("{}", result);
// }


fn main() { 
    // number_example();
    // variable_example();
    // pretty_print_example();
}

