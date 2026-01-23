# Quick Start

This guide will get you up and running with Snap in under 5 minutes.

## Create a New Project

Create a new Snap project using the `new` command:

```bash
snap new hello_scratch
cd hello_scratch
```

This creates the following structure:

```
hello_scratch/
├── config.toml      # Project configuration
├── src/
│   └── main.sp      # Main source file
└── .gitignore
```

## Project Structure

### config.toml

The configuration file contains project metadata:

```toml
[project]
name = "hello_scratch"
author = "Your Name"
```

### src/main.sp

The main source file contains your Snap code:

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

## Build the Project

Compile your project to a Scratch file:

```bash
snap build
```

This creates `hello_scratch.sb3` in your project directory.

## Run in Scratch

You can now open the `.sb3` file in:

- [Scratch Online Editor](https://scratch.mit.edu/projects/editor/) - Click "File" > "Load from your computer"
- [Scratch Desktop](https://scratch.mit.edu/download) - The offline editor
- [TurboWarp](https://turbowarp.org/) - A faster Scratch mod

Or use the `run` command to open Scratch automatically:

```bash
snap run
```

## Check for Errors

To check your code for errors without building:

```bash
snap check
```

Output:

```
Checking hello_scratch
Found: 1 sprite(s), 0 global(s)
  + Sprite 'Sprite1': 1 event(s), 0 function(s)

Success: No errors found!
```

## What's Next?

- [Your First Project](./first-project.md) - A more detailed tutorial
- [Language Guide](../language/syntax.md) - Learn the full syntax
- [Examples](../examples/hello-world.md) - See more examples
