# snap build

Compile a Snap project to a Scratch (.sb3) file.

## Synopsis

```bash
snap build [OPTIONS] [PATH]
```

## Description

Compiles the Snap project and generates a Scratch 3.0 project file (.sb3). The output file can be opened in:

- [Scratch Online Editor](https://scratch.mit.edu/projects/editor/)
- [Scratch Desktop](https://scratch.mit.edu/download)
- [TurboWarp](https://turbowarp.org/)

## Arguments

| Argument | Description |
|----------|-------------|
| `[PATH]` | Path to the project directory (defaults to current directory) |

## Options

| Option | Description |
|--------|-------------|
| `-o, --output <FILE>` | Output file path (defaults to `<project_name>.sb3`) |
| `-v, --verbose` | Enable verbose output |
| `-h, --help` | Print help information |

## Examples

### Basic Usage

```bash
# Build project in current directory
snap build

# Build project in specific directory
snap build path/to/project
```

Output:
```
Finished my_game (my_game.sb3)
```

### Verbose Output

```bash
snap build --verbose
```

Output:
```
Compiling project: my_game
  Found 3 sprite(s)
  Found stage definition
Finished my_game (my_game.sb3)
```

### Custom Output Path

```bash
snap build -o dist/game.sb3
snap build --output ~/Desktop/my_game.sb3
```

### Build Different Project

```bash
snap build ../other_project
snap build ~/projects/my_game
```

## Build Process

1. **Load Configuration** - Read `config.toml`
2. **Parse Source** - Tokenize and parse `src/main.sp`
3. **Process Imports** - Load and merge imported files
4. **Generate Code** - Convert AST to Scratch JSON format
5. **Package** - Create .sb3 ZIP archive with assets

## Output

The .sb3 file is a ZIP archive containing:

```
project.sb3 (ZIP)
├── project.json    # Scratch project data
├── *.svg          # Costume/backdrop images
└── *.wav          # Sound files (if any)
```

## Errors

### Config Not Found

```
error: Failed to read config file: No such file (config.toml)
```

Solution: Ensure you're in a Snap project directory with `config.toml`.

### Source File Not Found

```
error: Failed to read src/main.sp: No such file
```

Solution: Create `src/main.sp` or check the file path.

### Syntax Error

```
error: Unexpected token 'xyz' at 5:10
```

Solution: Fix the syntax error at the indicated line and column.

### Import Error

```
error: Error reading import sprites/player.sp: No such file
```

Solution: Check that the imported file exists and the path is correct.

## Performance

Build times depend on project size:

| Project Size | Typical Build Time |
|--------------|-------------------|
| Small (1-5 sprites) | < 100ms |
| Medium (10-20 sprites) | < 500ms |
| Large (50+ sprites) | < 2s |

## See Also

- [snap check](./check.md) - Check without building
- [snap run](./run.md) - Build and run
