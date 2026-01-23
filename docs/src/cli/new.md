# snap new

Create a new Snap project.

## Synopsis

```bash
snap new [OPTIONS] <NAME>
```

## Description

Creates a new Snap project with the given name. This command creates a new directory with the project name containing:

- `config.toml` - Project configuration
- `src/main.sp` - Main source file with example code
- `.gitignore` - Git ignore file for build outputs

## Arguments

| Argument | Description |
|----------|-------------|
| `<NAME>` | Name of the project to create |

## Options

| Option | Description |
|--------|-------------|
| `-a, --author <AUTHOR>` | Author name (optional) |
| `-h, --help` | Print help information |

## Examples

### Basic Usage

```bash
snap new my_game
```

Output:
```
Created `my_game` project

To get started:
  cd my_game
  snap build
```

### With Author

```bash
snap new my_game --author "Alice Smith"
```

### Created Structure

```
my_game/
├── config.toml
├── src/
│   └── main.sp
└── .gitignore
```

### config.toml

```toml
[project]
name = "my_game"
author = "Alice Smith"
```

### src/main.sp

```snap
// Welcome to Snap!
// This is your main source file.

new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello, Snap!", units::Sec(2));
        }
    }
}
```

### .gitignore

```
# Build output
*.sb3
```

## Errors

### Directory Already Exists

```bash
$ snap new my_game
error: Directory 'my_game' already exists
```

Solution: Choose a different name or delete the existing directory.

## See Also

- [snap init](./init.md) - Initialize in current directory
- [snap build](./build.md) - Build the project
