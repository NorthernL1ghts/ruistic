# Ruistic

A lightweight, fun programming language interpreter written in Rust. Ruistic is a simple yet powerful scripting language that combines the safety of Rust with an easy-to-learn syntax.

## Features

- Simple and intuitive syntax
- Fast execution
- Built with Rust for performance and safety
- Easy to extend and modify

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/NorthernL1ghts/ruistic.git
cd ruistic
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available in `target/release/ruistic`.

## Usage

### Running a Script

```bash
cargo run path/to/your/script.rst
```

### Example Script

See [test.rst](ruistic/test.rst) for a simple example:

```rust
// test.rst
for (var i = 0; i < 4; i = i + 1) {
    print i;
}
```

## Language Features

- Variables and basic data types
- Control flow (if/else, loops)
- Functions
- Basic arithmetic operations
- Print statements

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

- NorthernL1ghts

## Acknowledgments

- Inspired by the Lox language from "Crafting Interpreters"
- Built with Rust's excellent tooling and ecosystem "# ruistic" 
"# ruistic" 
