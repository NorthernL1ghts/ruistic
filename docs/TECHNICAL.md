# Ruistic Technical Documentation

## Architecture Overview

Ruistic is a tree-walk interpreter implemented in Rust. The interpreter follows a classic three-phase design:

1. **Lexical Analysis (Scanner)**
2. **Syntax Analysis (Parser)**
3. **Semantic Analysis (Interpreter)**

## Component Breakdown

### 1. Scanner (`src/scanner.rs`)
The scanner is responsible for converting source code into tokens. It:
- Reads the source code character by character
- Groups characters into meaningful tokens
- Handles whitespace and comments
- Reports lexical errors
- Supports the following token types:
  - Keywords (if, else, for, while, etc.)
  - Identifiers
  - Literals (numbers, strings)
  - Operators (+, -, *, /, etc.)
  - Punctuation (;, (, ), {, }, etc.)

### 2. Parser (`src/parser.rs`)
The parser converts the stream of tokens into an Abstract Syntax Tree (AST). It:
- Implements recursive descent parsing
- Handles operator precedence
- Generates a tree structure representing the program's syntax
- Supports the following expressions:
  - Binary expressions (a + b)
  - Unary expressions (-a)
  - Literals (numbers, strings)
  - Grouping expressions ((a + b))
  - Variable expressions
  - Assignment expressions
- Supports the following statements:
  - Expression statements
  - Print statements
  - Variable declarations
  - Block statements
  - If statements
  - While loops
  - For loops

### 3. Interpreter (`src/interpreter.rs`)
The interpreter walks the AST and executes the program. It:
- Implements the Visitor pattern for tree traversal
- Maintains an environment for variable storage
- Handles runtime errors
- Supports:
  - Variable assignment and lookup
  - Expression evaluation
  - Control flow
  - Function calls (if implemented)

### 4. Environment (`src/environment.rs`)
The environment system manages variable scoping and storage. It:
- Implements lexical scoping
- Uses a chain of environments for nested scopes
- Handles variable declaration and assignment
- Supports variable shadowing

## Memory Management

Ruistic uses Rust's ownership system for memory safety:
- AST nodes are owned by their parent nodes
- Environments use `Rc<RefCell<>>` for shared mutable state
- Values are stored in an enum to support different types

## Error Handling

The interpreter implements a robust error handling system:
- Lexical errors during scanning
- Syntax errors during parsing
- Runtime errors during interpretation
- All errors include line numbers and meaningful messages

## Example: How a Program is Processed

Let's walk through how Ruistic processes a simple program:

```rust
var x = 10;
print x + 5;
```

1. **Scanner Phase**:
   - Converts the input into tokens:
     ```
     VAR
     IDENTIFIER(x)
     EQUAL
     NUMBER(10)
     SEMICOLON
     PRINT
     IDENTIFIER(x)
     PLUS
     NUMBER(5)
     SEMICOLON
     ```

2. **Parser Phase**:
   - Creates an AST:
     ```
     Program
     ├── VarDecl
     │   ├── x
     │   └── Literal(10)
     └── Print
         └── Binary
             ├── Variable(x)
             └── Literal(5)
     ```

3. **Interpreter Phase**:
   - Creates a new environment
   - Declares variable x with value 10
   - Evaluates x + 5
   - Prints the result (15)

## Performance Considerations

- The tree-walk interpreter design prioritizes simplicity over performance
- No bytecode compilation or JIT optimization
- Suitable for scripting and educational purposes
- Memory usage is proportional to the AST size

## Future Improvements

Potential areas for enhancement:
1. Bytecode compilation
2. JIT optimization
3. Garbage collection
4. Standard library
5. Module system
6. Error recovery
7. Debugging support

## Testing

The interpreter includes:
- Unit tests for each component
- Integration tests for complete programs
- Error case testing
- Performance benchmarks

## Contributing

When contributing to Ruistic:
1. Follow Rust's style guidelines
2. Add tests for new features
3. Update documentation
4. Consider performance implications
5. Handle error cases gracefully
