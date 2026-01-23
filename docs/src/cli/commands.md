# CLI Commands

Snap provides a command-line interface (CLI) similar to Cargo for managing projects.

## Overview

```bash
snap <COMMAND> [OPTIONS]
```

### Available Commands

| Command | Description |
|---------|-------------|
| [`new`](./new.md) | Create a new Snap project |
| [`init`](./init.md) | Initialize a project in the current directory |
| [`build`](./build.md) | Compile the project to .sb3 |
| [`check`](./check.md) | Check for errors without building |
| [`run`](./run.md) | Build and open in Scratch |
| `help` | Print help information |

### Global Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

## Quick Reference

```bash
# Create a new project
snap new my_project
snap new my_project --author "Your Name"

# Initialize in current directory
snap init
snap init --author "Your Name"

# Build a project
snap build                    # Build current directory
snap build path/to/project    # Build specific project
snap build --verbose          # Show detailed output
snap build -o output.sb3      # Custom output path

# Check for errors
snap check                    # Check current directory
snap check path/to/project    # Check specific project

# Build and run
snap run                      # Build and open Scratch
snap run path/to/project      # Run specific project

# Get help
snap --help                   # General help
snap build --help             # Command-specific help
```

## Typical Workflow

### Starting a New Project

```bash
# Create the project
snap new my_game --author "Your Name"
cd my_game

# Edit your code
code src/main.sp  # Or your preferred editor

# Check for errors
snap check

# Build
snap build

# Or build and run
snap run
```

### Working on an Existing Project

```bash
# Navigate to project
cd my_game

# Make changes to src/main.sp

# Check syntax
snap check

# Build
snap build --verbose

# Test in Scratch
snap run
```

## Exit Codes

| Code | Meaning |
|------|--------|
| 0 | Success |
| 1 | Error (compilation, file not found, etc.) |

## Environment

Snap reads the following:

- `config.toml` - Project configuration
- `src/main.sp` - Main source file
- Additional `.sp` files via imports

## See Also

- [snap new](./new.md) - Create projects
- [snap build](./build.md) - Build projects
- [snap check](./check.md) - Check for errors
