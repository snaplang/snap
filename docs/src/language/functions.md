# Custom Functions

Custom functions (called "My Blocks" in Scratch) let you create reusable code blocks.

## Defining Functions

Define functions inside the `implements Code` block:

```snap
new Sprite("Player") {
    implements Code {
        fn jump() {
            control::Repeat(10) {
                motion::ChangeY(10);
            }
            control::Repeat(10) {
                motion::ChangeY(-10);
            }
        }

        on KeyPressed("space") {
            jump();
        }
    }
}
```

## Parameters

Functions can accept parameters:

```snap
fn moveSteps(steps: int) {
    control::Repeat(steps) {
        motion::Move(1);
        control::Wait(units::Sec(0.05));
    }
}

fn sayMessage(message: string, duration: float) {
    looks::SayTimed(message, units::Sec(duration));
}

fn setPosition(x: int, y: int) {
    motion::GoToXY(x, y);
}
```

### Parameter Types

| Type     | Description     | Example           |
| -------- | --------------- | ----------------- |
| `int`    | Integer numbers | `steps: int`      |
| `float`  | Decimal numbers | `speed: float`    |
| `string` | Text            | `message: string` |
| `bool`   | Boolean         | `visible: bool`   |

## Calling Functions

Call functions by name with arguments:

```snap
jump();                          // No parameters
moveSteps(10);                   // One parameter
sayMessage("Hello!", 2.0);       // Multiple parameters
setPosition(100, -50);           // Multiple parameters
```

## Examples

### Animation Function

```snap
fn animate(frames: int, delay: float) {
    control::Repeat(frames) {
        looks::NextCostume();
        control::Wait(units::Sec(delay));
    }
}

on GreenFlag {
    control::Forever {
        animate(4, 0.1);  // 4 frames at 0.1s each
    }
}
```

### Movement Functions

```snap
fn smoothMove(targetX: int, targetY: int, steps: int) {
    control::Repeat(steps) {
        motion::ChangeX((targetX - motion::XPosition) / 10);
        motion::ChangeY((targetY - motion::YPosition) / 10);
    }
}

fn teleportWithEffect(x: int, y: int) {
    looks::SetEffect("ghost", 100);
    motion::GoToXY(x, y);
    control::Repeat(10) {
        looks::ChangeEffect("ghost", -10);
        control::Wait(units::Sec(0.05));
    }
}

fn bounceMove(distance: int) {
    control::Repeat(distance) {
        motion::Move(1);
        motion::IfOnEdgeBounce();
    }
}
```

### Visual Effect Functions

```snap
fn flash(times: int) {
    control::Repeat(times) {
        looks::SetEffect("brightness", 100);
        control::Wait(units::Sec(0.1));
        looks::SetEffect("brightness", 0);
        control::Wait(units::Sec(0.1));
    }
}

fn fadeOut() {
    control::Repeat(20) {
        looks::ChangeEffect("ghost", 5);
        control::Wait(units::Sec(0.02));
    }
    looks::Hide();
    looks::ClearEffects();
}

fn fadeIn() {
    looks::SetEffect("ghost", 100);
    looks::Show();
    control::Repeat(20) {
        looks::ChangeEffect("ghost", -5);
        control::Wait(units::Sec(0.02));
    }
}

fn spin(rotations: int) {
    control::Repeat(rotations * 36) {
        motion::TurnRight(10);
        control::Wait(units::Sec(0.01));
    }
}
```

### Game Logic Functions

```snap
fn takeDamage(amount: int) {
    change lives by -amount;
    flash(3);

    if lives <= 0 {
        events::Broadcast("game_over");
    }
}

fn addScore(points: int) {
    change score by points;

    // Check for extra life
    if score % 1000 == 0 {
        change lives by 1;
        events::Broadcast("extra_life");
    }
}

fn spawnAt(x: int, y: int) {
    motion::GoToXY(x, y);
    looks::Show();
    looks::SetEffect("ghost", 0);
}
```

### Reusable Patterns

```snap
fn waitForClick() {
    control::RepeatUntil(sensing::MouseDown) {
        // Wait
    }
    control::RepeatUntil(!sensing::MouseDown) {
        // Wait for release
    }
}

fn typeText(text: string, charDelay: float) {
    let i: int = 1;
    control::Repeat(operators::Length(text)) {
        looks::Say(operators::Join("", operators::LetterOf(i, text)));
        change i by 1;
        control::Wait(units::Sec(charDelay));
    }
}
```

## Complete Example

```snap
new Sprite("Player") {
    implements Code {
        // === Custom Functions ===

        fn jump(height: int) {
            // Jump up
            control::Repeat(height) {
                motion::ChangeY(10);
                control::Wait(units::Sec(0.02));
            }
            // Fall down
            control::Repeat(height) {
                motion::ChangeY(-10);
                control::Wait(units::Sec(0.02));
            }
        }

        fn takeDamage() {
            change lives by -1;

            // Flash effect
            control::Repeat(5) {
                looks::SetEffect("ghost", 50);
                control::Wait(units::Sec(0.1));
                looks::SetEffect("ghost", 0);
                control::Wait(units::Sec(0.1));
            }

            if lives == 0 {
                events::Broadcast("game_over");
            }
        }

        fn reset() {
            motion::GoToXY(0, -100);
            looks::Show();
            looks::ClearEffects();
            set lives = 3;
        }

        // === Event Handlers ===

        on GreenFlag {
            reset();

            control::Forever {
                // Movement
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(5);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-5);
                }

                // Collision
                if sensing::TouchingSprite("Enemy") {
                    takeDamage();
                }
            }
        }

        on KeyPressed("space") {
            jump(10);
        }

        on Broadcast("restart") {
            reset();
        }
    }
}
```

## Warp Mode (Run Without Screen Refresh)

By default, custom functions refresh the screen after each block executes. This can make loops slow when you need to perform many operations quickly. The `warp` modifier makes a function run without screen refresh, executing all blocks as fast as possible before updating the display.

### Syntax

Add the `warp` keyword after `fn` to enable warp mode:

```snap
fn warp drawLine(length: int) {
    control::Repeat(length) {
        motion::Move(1);
        pen::PenDown();
    }
}
```

### When to Use Warp Mode

**Good use cases:**

- Drawing complex shapes with the pen
- Performing many calculations
- Initializing large amounts of data
- Any operation where visual feedback during execution isn't needed

```snap
// Drawing a filled square quickly
fn warp fillSquare(size: int) {
    control::Repeat(size) {
        control::Repeat(size) {
            pen::PenDown();
            motion::Move(1);
        }
        pen::PenUp();
        motion::ChangeX(-size);
        motion::ChangeY(1);
    }
}

// Fast calculation
fn warp calculateSum(n: int) {
    set result = 0;
    control::Repeat(n) {
        change result by n;
        change n by -1;
    }
}
```

**Avoid warp mode when:**

- You want to show animation progress
- User interaction is needed during execution
- You want the user to see step-by-step execution

### Comparison

Without warp (default):

```snap
fn drawSpiral(steps: int) {
    control::Repeat(steps) {
        motion::Move(steps);
        motion::TurnRight(90);
        // Screen refreshes here - user sees each step
    }
}
```

With warp:

```snap
fn warp drawSpiral(steps: int) {
    control::Repeat(steps) {
        motion::Move(steps);
        motion::TurnRight(90);
        // No screen refresh - executes instantly
    }
}
```

## Best Practices

1. **Use descriptive names** - `moveToCenter()` is better than `mtc()`

2. **Keep functions focused** - Each function should do one thing well

3. **Use parameters for flexibility** - `jump(height)` is more reusable than `jump()`

4. **Document complex functions** - Add comments explaining what the function does

5. **Avoid deep nesting** - Extract nested logic into separate functions

6. **Use warp for performance** - When you don't need visual feedback, use `fn warp` to speed up execution

## Limitations

- Functions cannot return values (Scratch limitation)
- Functions are sprite-local (cannot be called from other sprites)
- Recursive functions should be used carefully to avoid stack issues
- Warp functions cannot be interrupted by other scripts while running
