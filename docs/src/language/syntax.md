# Basic Syntax

This page covers the fundamental syntax of the Snap programming language.

## Comments

Snap supports single-line and multi-line comments:

```snap
// This is a single-line comment

/* This is a
   multi-line comment */
```

## Literals

### Numbers

```snap
42        // Integer
3.14      // Float
-10       // Negative integer
-0.5      // Negative float
```

### Strings

```snap
"Hello, world!"    // Basic string
"Line 1\nLine 2"   // With escape sequences
"Say \"Hi\""       // Escaped quotes
```

Supported escape sequences:
- `\n` - Newline
- `\t` - Tab
- `\r` - Carriage return
- `\\` - Backslash
- `\"` - Double quote

### Booleans

```snap
true
false
```

## Variables

### Global Variables

Global variables are declared at the top level and are accessible from all sprites:

```snap
let score: int = 0;
let player_name: string = "Player 1";
let game_over: bool = false;
let speed: float = 2.5;
```

### Variable Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer numbers | `0`, `42`, `-10` |
| `float` | Decimal numbers | `3.14`, `-0.5` |
| `bool` | Boolean values | `true`, `false` |
| `string` | Text strings | `"Hello"` |

### Setting Variables

```snap
set score = 100;
set player_name = "Alice";
```

### Changing Variables

```snap
change score by 10;    // Adds 10 to score
change speed by -0.5;  // Subtracts 0.5 from speed
```

## Operators

### Arithmetic

```snap
10 + 5     // Addition: 15
10 - 5     // Subtraction: 5
10 * 5     // Multiplication: 50
10 / 5     // Division: 2
10 % 3     // Modulo: 1
-x         // Negation
```

### Comparison

```snap
a == b     // Equal
a != b     // Not equal
a < b      // Less than
a > b      // Greater than
a <= b     // Less than or equal
a >= b     // Greater than or equal
```

### Logical

```snap
a && b     // Logical AND
a || b     // Logical OR
!a         // Logical NOT
```

### Operator Precedence

From highest to lowest:

1. `!`, `-` (unary)
2. `*`, `/`, `%`
3. `+`, `-`
4. `<`, `>`, `<=`, `>=`
5. `==`, `!=`
6. `&&`
7. `||`

Use parentheses to override precedence:

```snap
(a + b) * c
!(x && y)
```

## Units

Time values use the `units::` namespace:

```snap
units::Sec(2)      // 2 seconds
units::Sec(0.5)    // 0.5 seconds
```

These are used with blocks that require time durations:

```snap
looks::SayTimed("Hello!", units::Sec(2));
control::Wait(units::Sec(1));
```

## Block Calls

Scratch blocks are called using namespace syntax:

```snap
category::BlockName(arg1, arg2);
```

### Statement Blocks

Blocks that perform actions end with a semicolon:

```snap
motion::Move(10);
looks::Say("Hello!");
control::Wait(units::Sec(1));
```

### Reporter Blocks

Blocks that return values are used in expressions:

```snap
let x = motion::XPosition;
let touching = sensing::TouchingSprite("Enemy");
let random = operators::Random(1, 10);
```

### Blocks with Bodies

Control blocks that contain other blocks use curly braces:

```snap
control::Forever {
    motion::Move(1);
}

control::Repeat(10) {
    motion::TurnRight(36);
}
```

## Control Flow

### If Statements

```snap
if score > 100 {
    looks::Say("High score!");
}

if lives == 0 {
    looks::Say("Game Over");
} else {
    looks::Say("Keep playing!");
}
```

### Loops

```snap
// Forever loop
control::Forever {
    motion::Move(1);
}

// Repeat N times
control::Repeat(10) {
    motion::TurnRight(36);
}

// Repeat until condition
control::RepeatUntil(sensing::TouchingEdge) {
    motion::Move(5);
}
```

## Next Steps

- [Sprites & Stage](./sprites.md) - Learn about creating sprites
- [Events](./events.md) - Handle user input and events
- [Blocks](./blocks.md) - Complete block reference
