# Snap

[![Build Status](https://github.com/snaplang/snap/workflows/CI/badge.svg)](https://github.com/snaplang/snap/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Snap** is a programming language that compiles to Scratch 3.0 projects (`.sb3` files).

Write Scratch projects using clean, text-based syntax with your favorite editor, version control, and all the tools you're used to.

## Features

- **Clean syntax** inspired by Rust and JavaScript
- **Multiple files** with import support
- **All Scratch blocks** through namespaced functions
- **Fast compilation** to valid `.sb3` files
- **Cargo-like CLI** for project management

## Quick Example

```snap
// A simple Snap program
new Sprite("Player") {
    position: (0, -100),

    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello from Snap!", units::Sec(2));

            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(5);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-5);
                }
            }
        }
    }
}
```

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/snaplang/snap.git
cd snap

# Build
cargo build --release

# Add to PATH (Unix)
export PATH="$PATH:$(pwd)/target/release"

# Or create a symlink
sudo ln -s $(pwd)/target/release/snap /usr/local/bin/snap
```

## Getting Started

### Create a New Project

```bash
snap new my_game
cd my_game
```

### Edit Your Code

Open `src/main.sp` in your editor:

```snap
new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello, Snap!", units::Sec(2));
        }
    }
}
```

### Build

```bash
snap build
```

This creates `my_game.sb3` which you can open in [Scratch](https://scratch.mit.edu/projects/editor/) or [TurboWarp](https://turbowarp.org/).

## CLI Commands

| Command           | Description                     |
| ----------------- | ------------------------------- |
| `snap new <name>` | Create a new project            |
| `snap init`       | Initialize in current directory |
| `snap build`      | Compile to .sb3                 |
| `snap check`      | Check for errors                |
| `snap run`        | Build and open Scratch          |

```bash
snap new my_project          # Create project
snap build                   # Build current directory
snap build path/to/project   # Build specific project
snap build --verbose         # Show detailed output
snap check                   # Check without building
```

## Language Overview

### Sprites

```snap
new Sprite("Player") {
    position: (0, -100),
    size: 75,

    implements Code {
        on GreenFlag {
            // code here
        }
    }
}
```

### Events

```snap
on GreenFlag { }
on KeyPressed("space") { }
on Clicked { }
on Broadcast("message") { }
```

### Blocks

```snap
motion::Move(10);
motion::GoToXY(100, 50);
looks::Say("Hello!");
looks::SayTimed("Hi!", units::Sec(2));
control::Wait(units::Sec(1));
control::Forever { }
control::Repeat(10) { }
```

### Variables

```snap
let score: int = 0;
let name: string = "Player";

set score = 100;
change score by 10;
```

### Control Flow

```snap
if score > 100 {
    looks::Say("High score!");
} else {
    looks::Say("Keep trying!");
}

control::Forever {
    motion::Move(1);
}

control::Repeat(10) {
    motion::TurnRight(36);
}
```

### Custom Functions

```snap
fn jump(height: int) {
    control::Repeat(height) {
        motion::ChangeY(10);
    }
    control::Repeat(height) {
        motion::ChangeY(-10);
    }
}

on KeyPressed("space") {
    jump(10);
}
```

### Imports

```snap
use "sprites/player.sp";
use "sprites/enemy.sp";
```

## Documentation

Full documentation is available at [https://snaplang.github.io/snap](https://snaplang.github.io/snap/)

Or build locally:

```bash
cargo install mdbook
cd docs
mdbook serve
```

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
| Pen extension         | Complete |
| Lists                 | Planned  |
| Custom costumes       | Planned  |
| Custom sounds         | Planned  |

## Contributing

Contributions are welcome! Please see the [contributing guide](docs/src/contributing/setup.md) for details.

```bash
# Setup
git clone https://github.com/snaplang/snap.git
cd snap
cargo build

# Run tests
cargo test

# Format
cargo fmt
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [Scratch](https://scratch.mit.edu/) by MIT Media Lab
- The Rust community
