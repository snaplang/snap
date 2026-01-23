# snap init

Initialize a new Snap project in the current directory.

## Synopsis

```bash
snap init [OPTIONS]
```

## Description

Initializes a Snap project in the current directory. Unlike `snap new`, this command does not create a new directory - it sets up the project structure in place.

This is useful when:

- You already have a directory for your project
- You're converting an existing folder to a Snap project
- You cloned an empty repository and want to initialize it

## Options

| Option | Description |
|--------|-------------|
| `-a, --author <AUTHOR>` | Author name (optional) |
| `-h, --help` | Print help information |

## Examples

### Basic Usage

```bash
mkdir my_game
cd my_game
snap init
```

Output:
```
Initialized Snap project in current directory
```

### With Author

```bash
snap init --author "Your Name"
```

### In Cloned Repository

```bash
git clone https://github.com/user/my-scratch-game.git
cd my-scratch-game
snap init --author "Your Name"
```

## Created Files

`snap init` creates:

```
current_directory/
├── config.toml       # Created
└── src/
    └── main.sp       # Created if doesn't exist
```

### config.toml

The project name is derived from the current directory name:

```toml
[project]
name = "my_game"        # From directory name
author = "Your Name"    # If --author provided
```

### src/main.sp

Only created if it doesn't already exist:

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

## Behavior

### Existing Files

- **config.toml exists**: Error - project already initialized
- **src/main.sp exists**: Kept as-is (not overwritten)
- **src/ directory exists**: Used (not recreated)

### Project Name

The project name is automatically derived from the current directory name:

```bash
$ pwd
/home/user/my-awesome-game

$ snap init
# Creates config.toml with name = "my-awesome-game"
```

## Errors

### Already Initialized

```bash
$ snap init
error: Project already initialized (config.toml exists)
```

Solution: The project is already set up. Use `snap build` to compile.

## Comparison: init vs new

| Feature | `snap new` | `snap init` |
|---------|-----------|-------------|
| Creates directory | Yes | No |
| Works in empty dir | N/A | Yes |
| Works in existing project | No | No |
| Creates .gitignore | Yes | No |
| Project name source | Argument | Directory name |

## See Also

- [snap new](./new.md) - Create a new project directory
- [snap build](./build.md) - Build the project
