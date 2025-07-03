use crate::expr::Expr;

/// `fn pretty_print(&Expr) -> String`
/// Recursively generates a human-readable string representation of an expression tree.
/// # Arguments
/// `expr` - A reference to an `Expr` enum variant representing a mathematical expression.
/// # Returns
/// A `String` containing the pretty-printed version of the expression, with appropriate parentheses and formatting.
pub fn pretty_print(expr: &Expr) -> String {
    match expr {
        // Numeric literal, e.g., 2.0
        Expr::Number(n) => n.to_string(),

        // Variable, e.g., x or y
        Expr::Variable(x) => x.clone(),

        // Addition: recursively pretty-print left and right, wrap in parentheses
        Expr::Add(left, right) => format!("({} + {})", pretty_print(&left), pretty_print(&right)),

        // Subtraction: recursively pretty-print left and right, wrap in parentheses
        Expr::Sub(left, right) => format!("({} - {})", pretty_print(&left), pretty_print(&right)),

        // Multiplication: recursively pretty-print left and right, wrap in parentheses
        Expr::Mul(left, right) => format!("({} * {})", pretty_print(&left), pretty_print(&right)),

        // Division: recursively pretty-print left and right, wrap in parentheses
        Expr::Div(left, right) => format!("({} / {})", pretty_print(&left), pretty_print(&right)),

        // Exponentiation: recursively pretty-print left and right, wrap in parentheses
        Expr::Pow(left, right) => format!("({} ^ {})", pretty_print(&left), pretty_print(&right)),

        // Function call: pretty-print all arguments and join with commas, e.g., sin(x), max(a, b)
        Expr::Func(name, args) => {
            // Map each argument to its pretty-printed string
            let arg_str: Vec<String> = args.iter().map(pretty_print).collect();
            // Join arguments with commas and format as function call
            format!("{}({})", name, arg_str.join(", "))
        }
    }
}


/// `fn strip_outer_parantheses(String) -> String`
/// Strips the outer parantheses of the formatted expression
/// Generally, should be used after receiving the formatted string of the expression from `proton::format::pretty_print`
/// # Arguments 
/// `String` - the formatted string form of the `proton::expr::Expr` which contains outer parantheses
/// # Returns
/// `String` - A string without its outer parantheses
pub fn strip_outer_parantheses(mut s: String) -> String {
    // Check if the `String` has outer parantheses
    if s.starts_with(&['(']) && s.ends_with(&[')']) {
        s.pop();
        s.remove(0);
        return s;
    }
    // Default return value when the `String` does not contain any outer parantheses
    s
}


   