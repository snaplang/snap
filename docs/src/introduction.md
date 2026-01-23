# Snap Programming Language

<div class="warning">

**Note:** Snap is currently in early development. APIs and syntax may change.

</div>

**Snap** is a programming language that compiles to Scratch 3.0 projects (`.sb3` files). It allows you to write Scratch projects using a clean, text-based syntax with features like:

- **Familiar syntax** inspired by Rust and JavaScript
- **Multiple files** with import support
- **All Scratch blocks** available through namespaced functions
- **Fast compilation** to valid `.sb3` files
- **CLI tools** similar to Cargo for project management

## Why Snap?

Scratch is an amazing platform for learning programming, but its block-based interface can become limiting for larger projects. Snap bridges the gap by letting you:

- Write code in your favorite text editor
- Use version control (Git) effectively
- Organize code across multiple files
- Benefit from syntax highlighting and editor features
- Generate valid Scratch projects that run anywhere Scratch does

## Quick Example

```snap
// A simple Snap program
new Sprite("Player") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello from Snap!", units::Sec(2));

            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(10);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-10);
                }
            }
        }
    }
}
```

This compiles to a fully functional Scratch project with a sprite that responds to arrow key presses!

## Getting Started

Ready to try Snap? Head to the [Installation](./getting-started/installation.md) guide to get started.

## Project Status

| Feature               | Status   |
| --------------------- | -------- |
| Basic sprites & stage | Complete |
| Motion blocks         | Complete |
| Looks blocks          | Complete |
| Control blocks        | Complete |
| Events                | Complete |
| Variables             | Complete |
| Custom functions      | Complete |
| Imports               | Complete |
| Lists                 | Planned  |
| Pen extension         | Planned  |
| Custom costumes       | Planned  |
| Custom sounds         | Planned  |
