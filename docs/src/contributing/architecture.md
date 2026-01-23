# Architecture

This document describes the internal architecture of the Snap compiler.

## Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                       Source Code                               │
│                     (main.sp files)                             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                           Lexer                                 │
│                         (lexer.rs)                              │
│                 Source Text → Token Stream                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                          Parser                                 │
│                        (parser.rs)                              │
│                    Token Stream → AST                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Code Generator                             │
│                       (codegen.rs)                              │
│                   AST → Scratch Project                         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                          Packager                               │
│                       (packager.rs)                             │
│                 Scratch Project → .sb3 File                     │
└─────────────────────────────────────────────────────────────────┘
```

## Components

### Lexer (`lexer.rs`)

The lexer (tokenizer) converts source text into a stream of tokens.

```rust
// Input
"motion::Move(10);"

// Output
[
    Token { kind: Identifier("motion"), line: 1, column: 1 },
    Token { kind: ColonColon, line: 1, column: 7 },
    Token { kind: Identifier("Move"), line: 1, column: 9 },
    Token { kind: LParen, line: 1, column: 13 },
    Token { kind: IntLiteral(10), line: 1, column: 14 },
    Token { kind: RParen, line: 1, column: 16 },
    Token { kind: Semicolon, line: 1, column: 17 },
]
```

**Key types:**

- `TokenKind` - Enum of all token types
- `Token` - Token with position info
- `Lexer` - Stateful tokenizer

**Key functions:**

- `tokenize(source) -> Vec<Token>` - Main entry point
- `next_token()` - Get next token
- `skip_whitespace()` - Skip spaces and comments

### Parser (`parser.rs`)

The parser converts tokens into an Abstract Syntax Tree (AST).

```rust
// Input: Token stream for "new Sprite("Test") { ... }"

// Output
Program {
    sprites: [
        SpriteNode {
            name: "Test",
            code: Some(CodeBlock {
                event_handlers: [...],
                functions: [...],
            }),
        }
    ],
    ...
}
```

**Key types:**

- `Parser` - Stateful parser with token stream
- `Program` - Root AST node

**Key functions:**

- `parse(tokens) -> Program` - Main entry point
- `parse_sprite()` - Parse sprite definition
- `parse_statement()` - Parse a statement
- `parse_expression()` - Parse an expression

**Parsing strategy:**

- Recursive descent parser
- Pratt parsing for expressions (operator precedence)
- Lookahead for disambiguation

### AST (`ast.rs`)

Defines the Abstract Syntax Tree data structures.

```rust
pub struct Program {
    pub imports: Vec<Import>,
    pub globals: Vec<GlobalVar>,
    pub stage: Option<StageNode>,
    pub sprites: Vec<SpriteNode>,
}

pub struct SpriteNode {
    pub name: String,
    pub costumes: Vec<String>,
    pub position: Option<(f64, f64)>,
    pub size: Option<f64>,
    pub code: Option<CodeBlock>,
}

pub enum Statement {
    BlockCall(BlockCall),
    SetVariable { name: String, value: Expression },
    If { condition: Expression, then_body: Vec<Statement>, ... },
    Forever { body: Vec<Statement> },
    // ...
}

pub enum Expression {
    IntLiteral(i64),
    StringLiteral(String),
    Variable(String),
    BinaryOp { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    // ...
}
```

### Code Generator (`codegen.rs`)

Converts the AST into Scratch's JSON block format.

**Key types:**

- `CodeGenerator` - Stateful generator with ID counters
- Block/variable/broadcast ID maps

**Key functions:**

- `generate(config, program) -> ScratchProject` - Main entry point
- `generate_sprite()` - Generate a target
- `generate_statement()` - Generate blocks for a statement
- `generate_input()` - Generate block inputs

**Scratch block format:**

```json
{
  "opcode": "motion_movesteps",
  "next": "next_block_id",
  "parent": "parent_block_id",
  "inputs": {
    "STEPS": [1, [4, "10"]]
  },
  "fields": {},
  "shadow": false,
  "topLevel": false
}
```

### Scratch Structures (`scratch.rs`)

Defines Rust structs that serialize to Scratch's JSON format.

```rust
pub struct ScratchProject {
    pub targets: Vec<Target>,
    pub monitors: Vec<Monitor>,
    pub extensions: Vec<String>,
    pub meta: Meta,
}

pub struct Target {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, Variable>,
    pub blocks: HashMap<String, Block>,
    // ...
}

pub struct Block {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    // ...
}
```

### Packager (`packager.rs`)

Creates the final .sb3 file (ZIP archive).

**Contents of .sb3:**

- `project.json` - Serialized ScratchProject
- `*.svg` - Costume/backdrop images
- `*.wav` - Sound files

**Key functions:**

- `package(project, output_path)` - Create .sb3 file

## Data Flow Example

For this input:

```snap
new Sprite("Cat") {
    implements Code {
        on GreenFlag {
            motion::Move(10);
        }
    }
}
```

### 1. Lexer Output

```
new, Sprite, (, "Cat", ), {, implements, Code, {, on, GreenFlag, {,
motion, ::, Move, (, 10, ), ;, }, }, }
```

### 2. Parser Output (AST)

```rust
Program {
    sprites: [
        SpriteNode {
            name: "Cat",
            code: Some(CodeBlock {
                event_handlers: [
                    EventHandler {
                        event: GreenFlag,
                        body: [
                            BlockCall {
                                category: "motion",
                                block_name: "Move",
                                args: [IntLiteral(10)]
                            }
                        ]
                    }
                ]
            })
        }
    ]
}
```

### 3. Code Generator Output

```json
{
  "targets": [
    {
      "isStage": true,
      "name": "Stage",
      "blocks": {}
    },
    {
      "isStage": false,
      "name": "Cat",
      "blocks": {
        "abc123": {
          "opcode": "event_whenflagclicked",
          "next": "def456",
          "parent": null,
          "topLevel": true
        },
        "def456": {
          "opcode": "motion_movesteps",
          "next": null,
          "parent": "abc123",
          "inputs": {
            "STEPS": [1, [4, "10"]]
          }
        }
      }
    }
  ]
}
```

### 4. Packager Output

```
Cat.sb3 (ZIP)
├── project.json
├── cd21514d0531fdffb22204e0ec5ed84a.svg  (backdrop)
└── bcf454acf82e4504149f7ffe07081571.svg  (costume)
```

## Scratch Block Format Reference

### Input Types

| Type            | Code | Example                              |
| --------------- | ---- | ------------------------------------ |
| Number          | 4    | `[1, [4, "10"]]`                     |
| Positive Number | 5    | `[1, [5, "10"]]`                     |
| String          | 10   | `[1, [10, "hello"]]`                 |
| Broadcast       | 11   | `[1, [11, "message", "id"]]`         |
| Variable        | 12   | `[3, [12, "score", "id"], [4, "0"]]` |

### Block Reference

See [Scratch Wiki - Scratch File Format](https://en.scratch-wiki.info/wiki/Scratch_File_Format) for complete documentation.

## Adding New Features

### New Block Type

1. **codegen.rs**: Add opcode to `get_opcode()`
2. **codegen.rs**: Add input handling to `add_block_inputs()`
3. **Test**: Create test project using the block

### New Expression Type

1. **ast.rs**: Add variant to `Expression` enum
2. **parser.rs**: Add parsing in `parse_primary_expr()`
3. **codegen.rs**: Add generation in `generate_input()`

### New Statement Type

1. **ast.rs**: Add variant to `Statement` enum
2. **parser.rs**: Add parsing in `parse_statement()`
3. **codegen.rs**: Add generation in `generate_statement()`
