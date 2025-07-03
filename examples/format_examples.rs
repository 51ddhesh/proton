use proton::format::{pretty_print, strip_outer_parantheses};
use proton::expr::Expr;

/// Example usage of `proton::format::pretty_print` function
/// # Arguments 
/// `exp` - a reference to `proton::expr::Expr` representing an expression
/// # Returns 
/// A formatted `String`
/// Example - create an expression of the form ((x + 2) ^ 2) / y
/// `proton::format::pretty_print` will return it as (((x + 2) ^ 2) / y)
/// `proton::format::strip_outer_parantheses` will convert it to ((x + 2) ^ 2) / y
fn pretty_print_example_1() {
    let expr: Expr = Expr::Div(
        Box::new(Expr::Pow(
            Box::new(Expr::Add(Box::new(Expr::Variable("x".to_string())), Box::new(Expr::Number(2.0)))),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Variable("y".to_string())),
    );
    println!("With outer parantheses: {}", pretty_print(&expr));
    println!("Without outer parantheses: {}", strip_outer_parantheses(pretty_print(&expr)));
}

/// Create an expression of the form: ((sin(x + y) + 3) / 2) ^ tan(x) 
fn pretty_print_example_2() {
    let expr: Expr = Expr::Pow(
        Box::new(Expr::Div(
            Box::new(
                Expr::Add(
                    Box::new(
                        Expr::Func(
                            "sin".to_string(),
                            vec![
                                Expr::Add(
                                    Box::new(Expr::Variable("x".to_string())),
                                    Box::new(Expr::Variable("y".to_string())),
                                )
                            ]    
                        )
                    ),
                    Box::new(Expr::Number(3.0)),
                )
            ),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(
            Expr::Func(
                "tan".to_string(),
                vec![
                    Expr::Variable("x".to_string())
                ]
            )
        ),
    );

    println!("{}", strip_outer_parantheses(pretty_print(&expr)));
}

fn main() {
    pretty_print_example_1();
    pretty_print_example_2();
}