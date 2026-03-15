# Rust Tutorial: From Zero to Hero

Complete source code for the [Rust Tutorial](https://kemalcodes.com) series on [kemalcodes.com](https://kemalcodes.com).

A step-by-step Rust tutorial for developers who know another language (Kotlin, Python, JavaScript) but are new to Rust. 28 tutorials from your first `fn main()` to publishing a crate.

## How to Use This Repo

Each tutorial has its own branch. Switch to the branch you want:

```bash
git clone https://github.com/kemalcodes/rust-tutorial.git
cd rust-tutorial

# Switch to a specific tutorial
git checkout tutorial-01-what-is-rust
```

The `main` branch contains the base Rust project. Tutorial branches build on top of it.

## Tutorials

### Part 1: Foundations (Articles 1-8)

| # | Tutorial | Branch | Article |
|---|---------|--------|---------|
| 1 | What is Rust and Why Learn It? | `tutorial-01-what-is-rust` | [Read](https://kemalcodes.com/posts/why-learn-rust-2026/) |
| 2 | Installation and First Program | `tutorial-02-setup` | Coming soon |
| 3 | Variables, Types, and Functions | `tutorial-03-basics` | Coming soon |
| 4 | Ownership — The Key Concept | `tutorial-04-ownership` | Coming soon |
| 5 | Borrowing and References | `tutorial-05-borrowing` | Coming soon |
| 6 | Structs and Methods | `tutorial-06-structs` | Coming soon |
| 7 | Enums and Pattern Matching | `tutorial-07-enums` | Coming soon |
| 8 | Error Handling (Result, Option, ?) | `tutorial-08-errors` | Coming soon |

### Part 2: Core Concepts (Articles 9-14)

| # | Tutorial | Branch | Article |
|---|---------|--------|---------|
| 9 | Collections (Vec, HashMap, String) | `tutorial-09-collections` | Coming soon |
| 10 | Traits and Generics | `tutorial-10-traits` | Coming soon |
| 11 | Iterators and Closures | `tutorial-11-iterators` | Coming soon |
| 12 | Modules, Packages, and Cargo | `tutorial-12-modules` | Coming soon |
| 13 | Lifetimes Explained Simply | `tutorial-13-lifetimes` | Coming soon |
| 14 | Testing in Rust | `tutorial-14-testing` | Coming soon |

### Part 3: Building Things (Articles 15-20)

| # | Tutorial | Branch | Article |
|---|---------|--------|---------|
| 15 | Build a CLI Tool from Scratch | `tutorial-15-cli` | Coming soon |
| 16 | File I/O and JSON Parsing | `tutorial-16-file-io` | Coming soon |
| 17 | HTTP Requests with reqwest | `tutorial-17-http` | Coming soon |
| 18 | Async/Await and Tokio | `tutorial-18-async` | Coming soon |
| 19 | Build a REST API with Axum | `tutorial-19-axum` | Coming soon |
| 20 | Working with Databases (SQLx) | `tutorial-20-sqlx` | Coming soon |

### Part 4: Advanced (Articles 21-25)

| # | Tutorial | Branch | Article |
|---|---------|--------|---------|
| 21 | Smart Pointers (Box, Rc, Arc) | `tutorial-21-smart-pointers` | Coming soon |
| 22 | Concurrency (Threads, Channels, Mutex) | `tutorial-22-concurrency` | Coming soon |
| 23 | WebAssembly with Rust | `tutorial-23-wasm` | Coming soon |
| 24 | Macros Basics | `tutorial-24-macros` | Coming soon |
| 25 | Unsafe Rust (When and Why) | `tutorial-25-unsafe` | Coming soon |

### Part 5: Real Projects (Articles 26-28)

| # | Tutorial | Branch | Article |
|---|---------|--------|---------|
| 26 | Build a CLI Task Manager | `tutorial-26-task-manager` | Coming soon |
| 27 | Build a REST API with Authentication | `tutorial-27-auth-api` | Coming soon |
| 28 | Rust Cheat Sheet | — | Coming soon |

## Prerequisites

- Know at least one programming language (Kotlin, Python, JavaScript, etc.)
- [Rust installed](https://rustup.rs/) — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Run the Project

```bash
# Build
cargo build

# Run
cargo run

# Test
cargo test

# Check for errors without building
cargo check
```

## Tech Stack

- **Rust** (2024 edition)
- **Cargo** — build system and package manager
- **Tokio** — async runtime (Part 3+)
- **Axum** — web framework (Part 3+)
- **SQLx** — database access (Part 3+)
- **serde** — serialization/deserialization
- **clap** — CLI argument parsing

## Blog

All tutorials with detailed explanations at [kemalcodes.com](https://kemalcodes.com)

## License

[MIT](LICENSE)
