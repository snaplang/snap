# snap check

Check a Snap project for errors without building.

## Synopsis

```bash
snap check [PATH]
```

## Description

Parses and validates the Snap project without generating an output file. This is useful for:

- Quick syntax checking during development
- Validating code before committing
- Getting project statistics

## Arguments

| Argument | Description |
|----------|-------------|
| `[PATH]` | Path to the project directory (defaults to current directory) |

## Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help information |

## Examples

### Basic Usage

```bash
# Check project in current directory
snap check

# Check specific project
snap check path/to/project
```

### Successful Check

```bash
$ snap check
Checking my_game
Found: 3 sprite(s), 2 global(s)
  + Stage definition
  + Sprite 'Player': 2 event(s), 3 function(s)
  + Sprite 'Enemy': 1 event(s), 0 function(s)
  + Sprite 'Coin': 2 event(s), 0 function(s)

Success: No errors found!
```

### Failed Check

```bash
$ snap check
Checking my_game
error: Unexpected token 'implements' at 5:5
```

## Output Information

The check command reports:

- **Sprite count** - Number of sprites defined
- **Global count** - Number of global variables
- **Stage** - Whether a Stage is defined
- **Per-sprite details**:
  - Event handler count
  - Custom function count

## Use Cases

### Quick Syntax Check

```bash
# After editing, quickly check for errors
snap check
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit
snap check || exit 1
```

### CI/CD Pipeline

```yaml
# GitHub Actions example
- name: Check Snap project
  run: snap check
```

## Errors

Same errors as `snap build`:

- Config not found
- Source file not found
- Syntax errors
- Import errors

## Performance

Check is faster than build since it skips:

- Code generation
- File packaging
- Asset processing

## See Also

- [snap build](./build.md) - Build the project
- [snap run](./run.md) - Build and run
