# Release Notes - Snap v0.2.0

**Release Date:** January 29, 2026

## Highlights

This release brings the **Pen Extension** support, a new **exponentiation operator**, **warp mode** for custom functions, and several quality-of-life improvements.

---

## New Features

### Pen Extension Support

The pen extension is now fully supported! Enable it with `use pen;` at the top of your file to draw on the stage.

```snap
use pen;

new Sprite("Artist") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();
            motion::Move(100);
        }
    }
}
```

**Available pen blocks:**

- `pen::Clear()` - Clear all pen marks
- `pen::Stamp()` - Stamp sprite costume
- `pen::PenDown()` / `pen::PenUp()` - Control drawing
- `pen::SetPenColor(color)` - Set pen color (supports hue values, `units::Rgb()`, and `units::Rgba()`)
- `pen::ChangePenColor(amount)` - Change pen color parameter
- `pen::SetPenColorParam(param, value)` - Set specific color parameter
- `pen::SetPenSize(size)` / `pen::ChangePenSize(amount)` - Control pen size

### RGB/RGBA Color Support

New unit types for specifying colors:

```snap
pen::SetPenColor(units::Rgb(255, 0, 0));       // Red
pen::SetPenColor(units::Rgba(0, 255, 0, 128)); // Semi-transparent green
```

### Exponentiation Operator (`^`)

A new power operator for mathematical expressions:

```snap
let squared = 5 ^ 2;        // 25
let cubed = 3 ^ 3;          // 27
let power_of_2 = 2 ^ 10;    // 1024
```

- Right-associative: `2 ^ 3 ^ 2` = `2 ^ (3 ^ 2)` = `512`
- Higher precedence than multiplication
- Optimized for constant exponents (0-100) using repeated multiplication

### Warp Mode for Custom Functions

Functions can now run without screen refresh using the `warp` keyword:

```snap
fn warp drawComplexShape(size: int) {
    control::Repeat(size) {
        motion::Move(1);
        pen::PenDown();
    }
}
```

Warp mode executes all blocks as fast as possible before updating the display - ideal for:

- Drawing complex shapes with the pen
- Performing many calculations
- Initializing large amounts of data

### Variable Monitor Configuration

Global variables can now have monitor display settings:

```snap
let score: int = 0 monitor(x: 10, y: 10, visible: true);
```

---

## Improvements

### Build Output Organization

- Build artifacts now go to a `target/` directory instead of the project root
- Updated `.gitignore` template to ignore `target/*.sb3`

### Motion Block Fixes

- Fixed `motion::ChangeX()` to use correct input key `DX` instead of `X`
- Fixed `motion::ChangeY()` to use correct input key `DY` instead of `Y`

### Operator Enhancements

- Added `operators::Round()` support
- Added `operators::MathOp()` support for advanced math operations

### Documentation

- Added comprehensive pen blocks documentation
- Updated operator precedence documentation
- Added warp mode documentation to custom functions guide
- Improved code examples throughout

---

## Project Status Update

| Feature               | Status          |
| --------------------- | --------------- |
| Basic sprites & stage | Complete        |
| Motion blocks         | Complete        |
| Looks blocks          | Complete        |
| Control blocks        | Complete        |
| Events                | Complete        |
| Variables             | Complete        |
| Custom functions      | Complete        |
| Imports               | Complete        |
| **Pen extension**     | **Complete** ✨ |
| Lists                 | Planned         |
| Custom costumes       | Planned         |
| Custom sounds         | Planned         |

---

## Files Changed

- `Cargo.toml` - Version bump to 0.2.0
- `src/lexer.rs` - Added `warp` keyword and `^` operator token
- `src/parser.rs` - Added pen extension parsing, warp mode, power operator, RGB/RGBA units, monitor config
- `src/ast.rs` - Added extensions field, warp flag, monitor config fields, Power operator
- `src/codegen.rs` - Added pen block generation, power operation generation, monitor support
- `src/scratch.rs` - Added Monitor constructor
- `src/main.rs` - Updated build output to target directory
- `docs/src/language/blocks/pen.md` - New comprehensive pen documentation
- `docs/src/language/functions.md` - Added warp mode documentation
- `docs/src/language/blocks/operators.md` - Added exponentiation documentation
- `docs/src/language/syntax.md` - Updated operator precedence

---

## Upgrading

No breaking changes. Simply update to v0.2.0 and enjoy the new features!
