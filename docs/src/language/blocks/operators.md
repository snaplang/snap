# Operators

Operators perform math calculations, string operations, and logical comparisons.

## Native Operators

For most operations, use Snap's native operators instead of block calls:

### Arithmetic

```snap
let sum = a + b;        // Addition
let diff = a - b;       // Subtraction
let product = a * b;    // Multiplication
let quotient = a / b;   // Division
let remainder = a % b;  // Modulo
let negative = -a;      // Negation
```

### Comparison

```snap
a == b    // Equal
a != b    // Not equal
a < b     // Less than
a > b     // Greater than
a <= b    // Less than or equal
a >= b    // Greater than or equal
```

### Logical

```snap
a && b    // AND
a || b    // OR
!a        // NOT
```

### Grouping

```snap
let result = (a + b) * c;
let check = (x > 0) && (y > 0);
```

## Operator Blocks

Some operations require using the `operators::` namespace.

### Random

Generate a random number.

```snap
let roll = operators::Random(1, 6);       // Random 1-6
let x = operators::Random(-240, 240);     // Random x position
let chance = operators::Random(1, 100);   // Percentage chance
```

### Round

Round to nearest integer.

```snap
let rounded = operators::Round(3.7);    // 4
let rounded = operators::Round(3.2);    // 3
```

### Math Operations

Advanced math functions.

```snap
let abs_val = operators::MathOp("abs", -5);        // 5
let floored = operators::MathOp("floor", 3.7);    // 3
let ceiling = operators::MathOp("ceiling", 3.2);  // 4
let root = operators::MathOp("sqrt", 16);         // 4
let sine = operators::MathOp("sin", 90);          // 1
let cosine = operators::MathOp("cos", 0);         // 1
let tangent = operators::MathOp("tan", 45);       // 1
let log_val = operators::MathOp("ln", 2.718);     // ~1
let log10 = operators::MathOp("log", 100);        // 2
let exp_val = operators::MathOp("e ^", 2);        // ~7.39
let power10 = operators::MathOp("10 ^", 3);       // 1000
```

#### Available Math Operations

| Operation | Description |
|-----------|-------------|
| `"abs"` | Absolute value |
| `"floor"` | Round down |
| `"ceiling"` | Round up |
| `"sqrt"` | Square root |
| `"sin"` | Sine (degrees) |
| `"cos"` | Cosine (degrees) |
| `"tan"` | Tangent (degrees) |
| `"asin"` | Arc sine |
| `"acos"` | Arc cosine |
| `"atan"` | Arc tangent |
| `"ln"` | Natural logarithm |
| `"log"` | Base-10 logarithm |
| `"e ^"` | e to the power |
| `"10 ^"` | 10 to the power |

## String Operations

### Join

Concatenate strings.

```snap
let greeting = operators::Join("Hello, ", name);
let message = operators::Join("Score: ", score);

// Chain joins for multiple parts
let full = operators::Join(operators::Join(first, " "), last);
```

### Letter Of

Get a character at a position (1-indexed).

```snap
let first_char = operators::LetterOf(1, "Hello");  // "H"
let third_char = operators::LetterOf(3, "Hello");  // "l"
```

### Length

Get string length.

```snap
let len = operators::Length("Hello");  // 5
let name_len = operators::Length(player_name);
```

### Contains

Check if a string contains another string.

```snap
if operators::Contains("Hello World", "World") {
    looks::Say("Found it!");
}
```

## Examples

### Random Movement

```snap
on GreenFlag {
    control::Forever {
        motion::GlideToXY(
            units::Sec(1),
            operators::Random(-200, 200),
            operators::Random(-150, 150)
        );
    }
}
```

### Dice Roll

```snap
on KeyPressed("space") {
    let roll = operators::Random(1, 6);
    looks::SayTimed(operators::Join("You rolled: ", roll), units::Sec(2));
}
```

### Distance Formula

```snap
// Calculate distance between two points
let dx = x2 - x1;
let dy = y2 - y1;
let distance = operators::MathOp("sqrt", dx * dx + dy * dy);
```

### Percentage Chance

```snap
if operators::Random(1, 100) <= 25 {
    // 25% chance to run this code
    looks::Say("Lucky!");
}
```

### Text Score Display

```snap
on GreenFlag {
    control::Forever {
        looks::Say(operators::Join("Score: ", score));
    }
}
```

### Circular Motion

```snap
let angle: int = 0;

on GreenFlag {
    control::Forever {
        motion::GoToXY(
            100 * operators::MathOp("cos", angle),
            100 * operators::MathOp("sin", angle)
        );
        change angle by 5;
    }
}
```

### Clamp Value

```snap
// Keep a value between min and max
fn clamp(value: int, min: int, max: int) {
    if value < min {
        set value = min;
    }
    if value > max {
        set value = max;
    }
}
```

### Random Color

```snap
on KeyPressed("c") {
    looks::SetEffect("color", operators::Random(0, 200));
}
```
