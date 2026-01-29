# Pen Blocks

Pen blocks allow sprites to draw on the stage. To use pen blocks, you must first enable the pen extension with:

```snap
use pen;
```

## Enabling the Pen Extension

The pen extension must be enabled at the top of your `.sp` file before using any pen blocks:

```snap
use pen;

let speed: int = 5;

new Sprite("Drawer") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();

            control::Forever {
                motion::Move(speed);
                pen::ChangePenSize(1);
            }
        }
    }
}
```

## Pen Control

### Clear

Clear all pen marks from the stage:

```snap
pen::Clear();
```

### Stamp

Stamp the sprite's costume onto the stage:

```snap
pen::Stamp();
```

### Pen Down / Pen Up

Control whether the pen is drawing:

```snap
pen::PenDown();  // Start drawing
pen::PenUp();    // Stop drawing
```

Example:

```snap
on GreenFlag {
    pen::Clear();
    pen::PenDown();

    motion::GoToXY(-100, 0);
    motion::GoToXY(100, 0);  // Draws a line

    pen::PenUp();
    motion::GoToXY(0, 100);  // Moves without drawing
}
```

## Pen Color

### Set Pen Color

Set the pen color to a specific color value. You can use:

- Hue values (0-360) for Scratch's color system
- RGB values using `units::Rgb(r, g, b)`
- RGBA values using `units::Rgba(r, g, b, a)`

```snap
// Using hue values (0-360)
pen::SetPenColor(operators::Random(1, 360));  // Random hue
pen::SetPenColor(0);   // Red
pen::SetPenColor(120);  // Green
pen::SetPenColor(240); // Blue

// Using RGB values (0-255 for each component)
pen::SetPenColor(units::Rgb(255, 0, 0));      // Red
pen::SetPenColor(units::Rgb(0, 255, 0));      // Green
pen::SetPenColor(units::Rgb(0, 0, 255));      // Blue
pen::SetPenColor(units::Rgb(255, 255, 0));    // Yellow
pen::SetPenColor(units::Rgb(255, 0, 255));   // Magenta
pen::SetPenColor(units::Rgb(0, 255, 255));    // Cyan
pen::SetPenColor(units::Rgb(128, 128, 128));  // Gray

// Using RGBA values (0-255 for each component, alpha for transparency)
pen::SetPenColor(units::Rgba(255, 0, 0, 255));    // Opaque red
pen::SetPenColor(units::Rgba(0, 255, 0, 128));   // Semi-transparent green
pen::SetPenColor(units::Rgba(0, 0, 255, 0));     // Fully transparent blue
```

The `units::Rgb(r, g, b)` function calculates the color as: `r + g*256 + b*65536`

The `units::Rgba(r, g, b, a)` function calculates the color as: `r + g*256 + b*65536 + a*16777216`

### Change Pen Color

Change the pen color by a relative amount:

```snap
pen::ChangePenColor(10);  // Shift color by 10
pen::ChangePenColor(-5);  // Shift color back by 5
```

### Set Pen Color Parameter

Set a specific color parameter (hue, saturation, or brightness):

```snap
pen::SetPenColorParam("hue", 180);        // Set hue to 180 (cyan)
pen::SetPenColorParam("saturation", 100); // Full saturation
pen::SetPenColorParam("brightness", 50);   // Half brightness
```

## Pen Size

### Change Pen Size

Change the pen size by a relative amount:

```snap
pen::ChangePenSize(1);   // Make pen thicker
pen::ChangePenSize(-1);  // Make pen thinner
```

### Set Pen Size

Set the pen size to a specific value:

```snap
pen::SetPenSize(5);   // Thin line
pen::SetPenSize(20);  // Thick line
```

## Examples

### Drawing a Square

```snap
use pen;

new Sprite("Square") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();
            pen::SetPenSize(3);
            pen::SetPenColor(120);  // Green

            motion::GoToXY(-50, -50);
            motion::GoToXY(50, -50);
            motion::GoToXY(50, 50);
            motion::GoToXY(-50, 50);
            motion::GoToXY(-50, -50);  // Close the square

            pen::PenUp();
        }
    }
}
```

### Spiral Drawing

```snap
use pen;

let size: int = 1;
let angle: int = 0;

new Sprite("Spiral") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();
            pen::SetPenSize(2);

            motion::GoToXY(0, 0);

            control::Repeat(100) {
                motion::Move(size);
                motion::TurnRight(15);
                change size by 1;
                pen::ChangePenColor(5);
            }

            pen::PenUp();
        }
    }
}
```

### Rainbow Trail

```snap
use pen;

let hue: int = 0;

new Sprite("Rainbow") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();
            pen::SetPenSize(5);

            motion::GoToXY(-200, 0);

            control::Forever {
                motion::Move(2);
                pen::SetPenColor(hue);
                change hue by 1;

                if hue > 360 {
                    set hue = 0;
                }
            }
        }
    }
}
```

### RGB Color Gradient

```snap
use pen;

let r: int = 255;
let g: int = 0;
let b: int = 0;

new Sprite("Gradient") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();
            pen::SetPenSize(3);

            motion::GoToXY(-200, 0);

            control::Forever {
                motion::Move(2);
                pen::SetPenColor(units::Rgb(r, g, b));

                // Create a gradient from red to green to blue
                if r > 0 && b == 0 {
                    change r by -1;
                    change g by 1;
                } else if g > 0 && r == 0 {
                    change g by -1;
                    change b by 1;
                } else if b > 0 && g == 0 {
                    change b by -1;
                    change r by 1;
                }
            }
        }
    }
}
```

### Drawing Patterns

```snap
use pen;

new Sprite("Pattern") {
    implements Code {
        on GreenFlag {
            pen::Clear();
            pen::PenDown();
            pen::SetPenSize(2);

            control::Repeat(8) {
                pen::SetPenColor(operators::Random(0, 360));
                pen::SetPenSize(operators::Random(1, 10));

                motion::GoToXY(0, 0);
                motion::PointInDirection(operators::Random(0, 360));
                motion::Move(100);

                motion::TurnRight(45);
            }

            pen::PenUp();
        }
    }
}
```

## Notes

- Pen blocks only work for sprites, not the stage
- The pen draws a trail as the sprite moves when `PenDown()` is active
- Colors can be specified in multiple ways:
  - **Hue values** (0-360) for Scratch's color system:
    - 0 = Red
    - 60 = Yellow
    - 120 = Green
    - 180 = Cyan
    - 240 = Blue
    - 300 = Magenta
  - **RGB values** using `units::Rgb(r, g, b)` where each component is 0-255
  - **RGBA values** using `units::Rgba(r, g, b, a)` where each component is 0-255 (alpha controls transparency)
- The RGB/RGBA functions calculate the color value automatically:
  - `units::Rgb(r, g, b)` = `r + g*256 + b*65536`
  - `units::Rgba(r, g, b, a)` = `r + g*256 + b*65536 + a*16777216`
- Pen size is measured in pixels (typically 1-100)
- Use `Clear()` to erase all pen marks from the stage
