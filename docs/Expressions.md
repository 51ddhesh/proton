# Expressions

Expressions are the core data structure in Proton for representing mathematical formulas, variables, and operations. They are defined as an enum in [`expr.rs`](/src/expr.rs) and support a variety of mathematical constructs, including numbers, variables, arithmetic operations, and functions.

## Expression Types

The `Expr` enum typically includes variants such as:

- `Number(f64)`: Represents a numeric literal (e.g., `2.0`, `3.14`).
- `Variable(String)`: Represents a variable by name (e.g., `x`, `y`).
- `Add(Box<Expr>, Box<Expr>)`: Addition of two expressions.
- `Sub(Box<Expr>, Box<Expr>)`: Subtraction of two expressions.
- `Mul(Box<Expr>, Box<Expr>)`: Multiplication of two expressions.
- `Div(Box<Expr>, Box<Expr>)`: Division of two expressions.
- `Pow(Box<Expr>, Box<Expr>)`: Exponentiation (power) of two expressions.
- `Func(String, Vec<Expr>)`: Application of a function (e.g., `sin(x)`, `sqrt(y)`).

## Example: Creating Simple Expressions Step by Step

Here is how you can construct expressions programmatically using the `Expr` enum:

### 1. Numeric Literal
```rust
let expr = Expr::Number(2.0); // Represents the number 2
```

### 2. Variable
```rust
let expr = Expr::Variable("x".to_string()); // Represents the variable x
```

### 3. Simple Arithmetic
```rust
let expr = Expr::Add(Box::new(Expr::Number(2.0)), Box::new(Expr::Variable("x".to_string())));
// Represents the expression (2 + x)
```

### 4. Nested Expressions
```rust
let expr = Expr::Mul(
    Box::new(Expr::Add(Box::new(Expr::Number(2.0)), Box::new(Expr::Variable("x".to_string())))),
    Box::new(Expr::Number(3.0)),
);
// Represents the expression (2 + x) * 3
```

### 5. Function Application
```rust
let expr = Expr::Func("sin".to_string(), vec![Expr::Variable("theta".to_string())]);
// Represents the expression sin(theta)
```

## Creating More Complex Expressions

### Step 1: Get an Idea of the expression to create

Consider the following expression

```math
    \left(\frac{\sin(x + y) + 3}{2}\right)^{\tan(x)}
```

### Step 2: Break down the expression into smaller parts

Understand that the expression can be divided into two different parts, the base:
```math
    \frac{sin(x + y) + 3}{2} 
```
and the power:
```math
    tan(x)
```
The base can again be broken down into two parts, the numerator:
```math
    sin(x + y) + 3
```
and the denominator
```math
    2
```

The numerator can again be broken down into two terms
```math
    sin(x + y) \hspace{2mm} and \hspace{2mm} 3
```
The arguments of `sine` can also be broken down into two variables `x` and `y`  

### Step 3: Working from the outside to the inside
Since, the outermost function is a power function, the rough structure of the `expression` would look like:
```rust
let expr: Expr = Expr::Pow(
    Box::new(),
    Box::new(),
);
```

First, the power is implpemented, that is `tan`. In Proton, the `tan` is part of the `proton::expr::Expr` `enum` of type `Expr::Func`, which takes in the input as `Expr::Func(String, Vec<Expr, Global>)`.

```rust
let power_term: Expr = Expr::Func(
    "tan".to_string(),
    vec![
        Expr::Variable("x".to_string())
    ]
);
```

We now define the arguments of the `sin` in the numerator of the base. The arguments of `sin` are same as `tan`, `Expr::Func(String, Vec<Expr, Global>)`, hence, we create a normal `Expr::Add()`, and then convert it into a `Vec`. The `Expr::Add` enum has arguments in the form of `Expr::Add(Box<Expr, Global>, Box<Expr, Global>)`. Hence, we pass `x` and `y` as `Expr::Variable`

```rust
let sin_args: Expr = Expr::Add(
    Box::new(Expr::Variable("x".to_string())),
    Box::new(Expr::Variable("y".to_string()))
);
let sin_args = vec![sin_args];
```

We now can create the numerator of the base by adding the `sin` with 3 as `Expr::Number`.

```rust
let numerator: Expr = Expr::Add(
    Box::new(Expr::Func(
        "sin".to_string(),
        sin_args,
    )),
    Box::new(Expr::Number(3.0))
);
```

Since, the numerator is a simple, single number, there is no need to create a separate `Expr` for it. We can now make create the base

```rust
let base_term: Expr = Expr::Div(
    Box::new(numerator),
    Box::new(Expr::Number(2.0)),
);
```
And now, the complete expression would look like:
```rust
let expr: Expr = Expr::Pow(
    Box::new(base),
    Box::new(power_term)
)
```

Creating this example in a single `expr` is a bit complex and would look like this:
```rust
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
```
> Snippet taken from [`examples/format_examples.rs`](/examples/format_examples.rs)

---

## Usage

Expressions can be evaluated using the `evaluate` function, pretty-printed using `pretty_print`, and manipulated for symbolic computation. See the [`examples`](../examples/) directory for practical usage.

---
