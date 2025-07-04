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
fn format_example_1() {
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
fn format_example_2() {
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

// Creating complex expressions by breaking them down
// Example: f(x) =  2x^2 sin(x) + 3x cos^2(x) + sqrt(x) tan(x)
#[allow(unused)]
fn format_example_3() {
    let term_1_part_1: Expr = Expr::Pow(
        Box::new(Expr::Mul(
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Variable("x".to_string()))
        )),
        Box::new(Expr::Number(2.0))
    );
    let term_1_part_1: Box<Expr> = Box::new(term_1_part_1);

    let term_1_part_2: Expr = Expr::Func(
        "sin".to_string(),
        vec![Expr::Variable("x".to_string())],
    );
    let term_1_part_2: Box<Expr> = Box::new(term_1_part_2);

    let term_1: Expr = Expr::Mul(
        term_1_part_1,
        term_1_part_2,
    );
    let term_1: Box<Expr> = Box::new(term_1);

    // ------------------------------------

    let term_2_part_1: Expr = Expr::Mul(
        Box::new(Expr::Number(3.0)),
        Box::new(Expr::Variable("x".to_string()))
    );
    let term_2_part_1: Box<Expr> = Box::new(term_2_part_1);

    let term_2_part_2: Expr = Expr::Pow(
        Box::new(Expr::Func(
            "cos".to_string(),
            vec![Expr::Variable("x".to_string())],
        )),
        Box::new(Expr::Number(2.0)),
    );
    let term_2_part_2: Box<Expr> = Box::new(term_2_part_2);

    let term_2: Expr = Expr::Mul(
        term_2_part_1,
        term_2_part_2,
    );
    let term_2: Box<Expr> = Box::new(term_2);

    // -----------------------------------

    let term_3_part_1: Expr = Expr::Func(
        "sqrt".to_string(),
        vec![Expr::Variable("x".to_string())],
    );
    let term_3_part_1: Box<Expr> = Box::new(term_3_part_1);

    let term_3_part_2: Expr = Expr::Func(
        "tan".to_string(),
        vec![Expr::Variable("x".to_string())],
    );
    let term_3_part_2: Box<Expr> = Box::new(term_3_part_2);

    let term_3: Expr = Expr::Mul(
        term_3_part_1,
        term_3_part_2,
    );
    let term_3: Box<Expr> = Box::new(term_3);

    // -----------------------------------

    let exp: Expr = Expr::Add(
        term_1,
        term_2,
    );

    let exp: Expr = Expr::Add(
        Box::new(exp),
        term_3
    );

    println!("{}", strip_outer_parantheses(pretty_print(&exp)));

}

fn main() {
    format_example_1();
    format_example_2();
    format_example_3();
}