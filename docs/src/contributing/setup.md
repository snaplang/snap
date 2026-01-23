# Development Setup

This guide covers setting up a development environment for contributing to Snap.

## Prerequisites

- [Rust](https://rustup.rs/) (stable, 1.70+)
- [Git](https://git-scm.com/)
- A code editor (VS Code recommended)

## Clone the Repository

```bash
git clone https://github.com/snaplang/snap.git
cd snap
```

## Build

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release
```

## Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_lexer

# Run with output
cargo test -- --nocapture
```

## Run the Compiler

```bash
# Run in debug mode
cargo run -- new test_project
cargo run -- build test_project

# Run release build
cargo run --release -- build test_project
```

## Project Structure

```
snap/
├── Cargo.toml           # Rust dependencies and metadata
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config.rs        # config.toml parsing
│   ├── lexer.rs         # Tokenizer
│   ├── parser.rs        # Parser (tokens → AST)
│   ├── ast.rs           # Abstract Syntax Tree definitions
│   ├── codegen.rs       # Code generator (AST → Scratch JSON)
│   ├── scratch.rs       # Scratch project structures
│   └── packager.rs      # .sb3 file creation
├── docs/                # Documentation (mdBook)
│   ├── book.toml
│   └── src/
└── test_project/        # Test project for development
```

## Development Workflow

### 1. Create a Test Project

```bash
cargo run -- new dev_test
```

### 2. Edit Test Code

Modify `dev_test/src/main.sp` to test the feature you're working on.

### 3. Build and Check Output

```bash
cargo run -- build dev_test --verbose

# Extract and inspect the output
cd dev_test
unzip -o dev_test.sb3 -d extracted
cat extracted/project.json | jq .
```

### 4. Test in Scratch

Open the `.sb3` file in [Scratch](https://scratch.mit.edu/projects/editor/) or [TurboWarp](https://turbowarp.org/) to verify it works.

## Code Style

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Before Committing

```bash
cargo fmt
cargo clippy
cargo test
cargo build
```

## Debugging

### Print AST

Add debug output in `main.rs`:

```rust
let program = compile_source(&source, project_path)?;
println!("{:#?}", program);  // Pretty print AST
```

### Print Generated JSON

```rust
let scratch_project = codegen::generate(&config, &program);
println!("{}", serde_json::to_string_pretty(&scratch_project).unwrap());
```

### Inspect .sb3 Contents

```bash
unzip -l project.sb3           # List contents
unzip -p project.sb3 project.json | jq .  # View JSON
```

## Common Tasks

### Adding a New Block

1. Add opcode mapping in `codegen.rs` → `get_opcode()`
2. Add input handling in `codegen.rs` → `add_block_inputs()`
3. Test with a sample project

### Adding a New Event

1. Add variant to `Event` enum in `ast.rs`
2. Add parsing in `parser.rs` → `parse_event()`
3. Add hat block creation in `codegen.rs` → `create_hat_block()`

### Adding a New Statement

1. Add variant to `Statement` enum in `ast.rs`
2. Add parsing in `parser.rs` → `parse_statement()`
3. Add code generation in `codegen.rs` → `generate_statement()`

## Documentation

### Building Docs

```bash
# Install mdBook
cargo install mdbook

# Build docs
cd docs
mdbook build

# Serve locally
mdbook serve
```

Open http://localhost:3000 to view the documentation.

## Getting Help

- Check existing issues on GitHub
- Read the [Architecture](./architecture.md) guide
- Ask questions in discussions/issues
