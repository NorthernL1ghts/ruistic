![Ruistic Logo](resources/LOGO.jpg)

# Ruistic

Ruistic is a lightweight, custom programming language interpreter written in Rust. Features include a custom lexer, recursive descent parser, and tree-walk interpreter. Supports variables, control flow, functions, and arithmetic operations. Built as a learning exercise in language design and Rust.

## Features

- Custom lexer/scanner implementation
- Recursive descent parser
- Tree-walk interpreter
- Support for variables and control flow
- Function definitions and calls
- Basic arithmetic operations

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Building from Source

```bash
git clone https://github.com/NorthernL1ghts/ruistic.git
cd ruistic
cargo build --release
```

## Usage

```bash
# Run the interpreter
cargo run

# Run with a specific file
cargo run -- path/to/script.ru
```

## Language Features

- Variable declarations and assignments
- Control flow (if/else, while loops)
- Function definitions and calls
- Basic arithmetic operations
- Print statements for output

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

- NorthernL1ghts

## Acknowledgments

- Inspired by the book "Crafting Interpreters" by Robert Nystrom
- Built as a learning exercise in Rust and language design
