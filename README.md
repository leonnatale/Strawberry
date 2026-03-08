# 🍓 Strawberry

An interpreted programming language created in Rust to explore the fundamentals of how programming languages work internally.

## What is Strawberry?

Strawberry is an imperative and interpreted programming language, developed as a practical study of compilers and interpreters. Created entirely in Rust, it implements the essential components of a language: lexical analysis, syntactic analysis (parser), and interpretation.

Originally inspired by esoteric languages, it evolved into a complete educational project that demonstrates in practice how a language is born and takes shape.

## Features

✨ **Interpreted** - Executes code directly without compilation
📝 **Supports multiple data types** - Strings, numbers, booleans, functions
🔄 **Variable scopes** - Local scope within functions and global scope
⚙️ **Mathematical expressions** - Evaluation of arithmetic operations
🔀 **Comparisons** - Equality and inequality operators  
❓ **Conditionals** - if/else structures for control flow  
📚 **Standard Library** - Integrated native functions  

## How it works internally

Strawberry follows the classic interpretation flow:

```
Source Code (.sb) 
       ↓
   Lexer (Lexical Analysis)
       ↓
Parser (Syntactic Analysis + Interpretation)
       ↓
AST (Abstract Syntax Tree)
       ↓
Interpretation (Visitor Pattern)
       ↓
Result
```

### Main components

- **Lexer** - Analyzes code character by character, generating tokens
- **Parser** - Groups tokens, builds the AST, and interprets the code
- **Standard Library** - Native functions like `strawberry()`, `if()`, and dynamic variables

## Usage examples

### Hello World

```strawberry
function hello(entity) {
    'Hello, ' + entity + '!'
}

strawberry(hello(beatle))
```

**Output:** `Hello, Paul McCartney!` (or another random Beatles member)

### Mathematical expressions

```strawberry
strawberry(
    1 + 2,
    10 - 5,
    3 * 4,
    20 / 2
)
```

### Comparisons

```strawberry
strawberry(
    1 == 2,
    1 == 1,
    2 >= 2,
    false != true
)
```

### Conditionals

```strawberry
if(beatle == 'Paul McCartney' {
    strawberry(beatle, 'is the best!')
}, {
    strawberry(beatle, 'is cool!')
})
```

## Standard Library

### Functions

- **`strawberry(args...)`** - Prints values to console
- **`if(condition, ifBlock, elseBlock)`** - Conditionally executes code blocks

### Dynamic variables

- **`beatle`** - Returns a random Beatles member
- **`fields_forever`** - Returns a random verse from "Strawberry Fields Forever"

## Building and running

### Prerequisites
- Rust 1.70+

### Build

```bash
cargo build --release
```

### Run a Strawberry file

```bash
cargo run -- examples/hello_world.sb
```

## Key learnings

Creating Strawberry provided valuable insights into:

- **Lexical Analysis** - How to transform text into meaningful tokens
- **Syntactic Analysis** - Construction of abstract syntax trees (AST)
- **Interpretation** - Traversal and execution of ASTs
- **Scopes** - Context and variable management
- **Type systems** - Representation of typed values through Rust enums
- **Rust integration** - How Rust integrates well in low-level projects, similar to Lua with C

## Limitations

⚠️ This is an educational language with limited scope:

- No support for full recursion
- Minimal standard library
- No module system
- No support for advanced functional patterns
- No code optimization

## License

See the [LICENSE](LICENSE) file for details.

---

**Note:** This is an educational project created to explore the fundamentals of programming languages. Not recommended for production use.
