# Proton

> Note: This project is under development.

`Proton` is a mathematical engine written in `Rust`.

`Proton` is built on [`proton_lite`](https://github.com/51ddhesh/proton_lite.git).


## Examples
Example usage is in the [`examples/`](./examples/) directory. Run the examples using:

```bash
cargo run --example <file>
```
Example usage for running the [formatting examples](./examples/format_examples.rs)

```bash
cargo run --example format_examples
```


## Features under development
- Mathematical Parsing
- Integration
- Differentiation
- Numeric Calculus



## Current Structure 

```
proton/                    # Project root
├── src/                   # Source code directory
│   ├── main.rs            # Placeholder for an API service
│   ├── expr.rs            # Expression parsing and representation
│   ├── eval.rs            # Expression evaluation
│   ├── format.rs          # Formatting utilities for expressions/output
│   └── lib.rs             # Library entry point
├── examples/
│   ├── format_example.rs  # Examples related to formatting (from proton::format)
│   └──
├── tests/                 # Integration and unit tests
│   ├── pretty_print.rs    # Tests for pretty-printing functionality
│   └── eval.rs            # Tests for expression evaluation  
├── README.md              # Project documentation
├── Cargo.toml             # Rust package manifest
├── Cargo.lock             # Cargo dependency lockfile
└── .gitignore             # Git ignored files list
```
 

