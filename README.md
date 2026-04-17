# 🦀 Lox Interpreter — Rust

> A fully-featured interpreter for the **Lox** programming language, built from scratch in Rust.
> The following project is developed by hand with AI assistance from Claude Sonnet 4.6.
> Inspired by Robert Nystrom's [*Crafting Interpreters*](https://craftinginterpreters.com/).

[![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Stage](https://img.shields.io/badge/stage-Tree--Walk%20Interpreter-yellow)]()

---

## 📖 Overview

This project is a handcrafted implementation of the **Lox** scripting language interpreter in Rust. Lox is a dynamically-typed, object-oriented language designed by Robert Nystrom as a teaching vehicle for interpreter design. This implementation covers the full pipeline from raw source text to evaluated output — scanner, parser, AST, and interpreter — all written in idiomatic Rust.

---

## 🏗️ Architecture

```
Source Code (.lox)
       │
       ▼
 ┌──────────────┐
 │    Scanner   │  → Tokenizes raw source into a stream of Tokens
 └──────┬───────┘
        │
        ▼
 ┌──────────────┐
 │    Parser    │  → Builds an Abstract Syntax Tree (AST) from tokens
 └──────┬───────┘
        │
        ▼
 ┌──────────────┐
 │  AST Printer │  → Pretty-prints AST for debugging & visualization
 └──────┬───────┘
        │
        ▼
 ┌──────────────┐
 │  Interpreter │  → Tree-walk evaluator; executes the AST directly
 └──────────────┘
```

---

## ✅ Features Implemented

### Lexer / Scanner
- Full tokenization of Lox source files
- Handles all literal types: numbers, strings, booleans, `nil`
- Identifier and reserved keyword recognition
- Robust UTF-8 character processing
- Meaningful lexer-level error messages with line context
- Comprehensive test suite for the scanner

### Parser
- Recursive descent parser for Lox grammar
- Full expression parsing (arithmetic, comparison, logical, grouping)
- Structured error handling and panic-mode recovery
- Generates a complete, well-formed Abstract Syntax Tree (AST)

### AST Printer
- Lisp-style parenthesized AST pretty-printer for debugging
- Full AST evaluation implementation
- Auto-generated AST boilerplate via custom utility tooling

### Evaluator / Interpreter
- Tree-walk interpreter over the AST
- Multiple object/value representations: `Number`, `String`, `Boolean`, `Nil`
- Runtime error handling with descriptive messages

---

## Roadmap

This interpreter is actively being developed. Upcoming milestones include:

| Feature | Status |
|---|---|
| Scanner | ✅ Complete |
| Expression Parser | ✅ Complete |
| AST Evaluation | ✅ Complete |
| Statement Parser (`print`, `var`, blocks) | 🔄 In Progress |
| Variables & Environments | 🔜 Planned |
| Control Flow (`if`, `while`, `for`) | 🔜 Planned |
| Functions & Closures | 🔜 Planned |
| Classes & Inheritance | 🔜 Planned |
| Bytecode VM (Part II) | 🔜 Planned |

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.78+)
- Cargo (bundled with Rust)

### Build

```bash
git clone https://github.com/Cletine/lox-interpreter.git
cd lox-interpreter
cargo build --release
```

### Run

```bash
# Run a .lox source file
cargo run --bin lox_interpreter -- /path/to/your/script.lox

```

### Test

```bash
cargo test
```

---

## 🧪 Example Lox Code

```lox
// Variables and arithmetic
var a = 10;
var b = 20;
print a + b; // 30

// String concatenation
var greeting = "Hello, " + "Lox!";
print greeting;

// Conditional logic
if (a < b) {
  print "a is smaller";
} else {
  print "b is smaller";
}
```

> **Note:** Not all of the above features may be executable yet depending on the current stage of the interpreter. See the roadmap above.


## 🔧 Design Decisions

**Why Rust?**
Rust's ownership model makes it exceptionally well-suited for interpreter development. Memory safety is guaranteed at compile time, and the type system enforces correctness across complex recursive data structures like ASTs — without a garbage collector.

**Why Tree-Walk First?**
Following Nystrom's approach, the tree-walk interpreter provides a clean mental model for language semantics before optimizing with a bytecode VM in Phase 2. It's the right foundation.

---

## 📚 Reference

- 📖 [*Crafting Interpreters* by Robert Nystrom](https://craftinginterpreters.com/) — the definitive guide this project is based on
- 🦀 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 🔬 [Rust Reference](https://doc.rust-lang.org/reference/)

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

<p align="center">
  <i>If you found this project interesting, consider giving it a ⭐ — it helps others find it!</i>
</p>
