# snap run

Build the project and open Scratch.

## Synopsis

```bash
snap run [PATH]
```

## Description

Builds the Snap project and opens the Scratch editor in your default web browser. This is a convenience command that combines building with launching.

## Arguments

| Argument | Description                                                   |
| -------- | ------------------------------------------------------------- |
| `[PATH]` | Path to the project directory (defaults to current directory) |

## Options

| Option       | Description            |
| ------------ | ---------------------- |
| `-h, --help` | Print help information |

## Examples

### Basic Usage

```bash
# Run project in current directory
snap run

# Run specific project
snap run path/to/project
```

### Output

```bash
$ snap run
Finished my_game (my_game.sb3)
Info: Opening Scratch editor...
  Load your project by clicking File > Load from your computer
  Project file: my_game.sb3
```

## Workflow

1. **Build** - Compiles the project (same as `snap build`)
2. **Open Browser** - Opens Scratch editor in default browser
3. **Manual Load** - User loads the .sb3 file manually

## Loading the Project

After `snap run`:

1. Scratch editor opens in your browser
2. Click **File** in the menu bar
3. Select **Load from your computer**
4. Navigate to your project folder
5. Select the `.sb3` file
6. Click the **Green Flag** to run!

## Platform Support

| Platform | Browser Opening         |
| -------- | ----------------------- |
| macOS    | Uses `open` command     |
| Windows  | Uses `start` command    |
| Linux    | Uses `xdg-open` command |

## Alternative Scratch Platforms

The generated .sb3 file works with:

- **[Scratch Online](https://scratch.mit.edu/projects/editor/)** - Official editor
- **[Scratch Desktop](https://scratch.mit.edu/download)** - Offline editor
- **[TurboWarp](https://turbowarp.org/)** - Faster Scratch mod
- **[Forkphorus](https://forkphorus.github.io/)** - High-performance player

## Errors

Same errors as `snap build`:

- Config not found
- Source file not found
- Syntax errors
- Import errors

If the build fails, the browser will not open.

## Tips

### Quick Iteration

```bash
# Edit code, then:
snap run

# In Scratch, just reload the file (File > Load from your computer)
```

### Keep Browser Open

Keep the Scratch editor open in a browser tab. After rebuilding, just reload the .sb3 file without closing the tab.

## See Also

- [snap build](./build.md) - Build without opening
- [snap check](./check.md) - Check without building
