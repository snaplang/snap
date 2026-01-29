# Variables

Variables store data that can change during your program's execution.

## Declaring Variables

Global variables are declared at the top level of your `.sp` file:

```snap
let score: int = 0;
let player_name: string = "Player 1";
let game_over: bool = false;
let speed: float = 2.5;
```

### Variable Monitors

By default, variable monitors (the on-screen displays) are **hidden**. You can configure a variable's monitor to be visible and set its position on the screen:

```snap
let score: int = 0 monitor(x: 10, y: 20, visible: true);
let lives: int = 3 monitor(x: 10, y: 60, visible: true);
let speed: float = 2.5;  // Monitor hidden by default
```

Monitor configuration is optional. You can specify:

- `x`: X position on screen (default: 5.0)
- `y`: Y position on screen (default: auto-incremented from 5.0)
- `visible`: Whether the monitor is visible (default: `false`)

All monitor parameters are optional - you can specify only the ones you need:

```snap
let score: int = 0 monitor(visible: true);  // Visible at default position
let lives: int = 3 monitor(x: 100, y: 200); // Hidden at custom position
let speed: float = 2.5 monitor(visible: true, x: 200); // Visible at x=200, auto y
```

### Variable Types

| Type     | Description     | Default Value | Example Values          |
| -------- | --------------- | ------------- | ----------------------- |
| `int`    | Integer numbers | `0`           | `0`, `42`, `-10`        |
| `float`  | Decimal numbers | `0.0`         | `3.14`, `-0.5`, `2.0`   |
| `bool`   | Boolean values  | `false`       | `true`, `false`         |
| `string` | Text strings    | `""`          | `"Hello"`, `"Player 1"` |

## Setting Variables

Use the `set` keyword to assign a new value:

```snap
set score = 100;
set player_name = "Alice";
set game_over = true;
set speed = 5.0;
```

You can set variables to expressions:

```snap
set score = score + 10;
set speed = operators::Random(1, 5);
set player_name = sensing::Answer;
```

## Changing Variables

Use `change ... by` to add to a numeric variable:

```snap
change score by 10;      // score = score + 10
change score by -5;      // score = score - 5
change speed by 0.5;     // speed = speed + 0.5
change lives by -1;      // lives = lives - 1
```

## Variable Scope

Currently, all variables in Snap are **global**, meaning they can be accessed from any sprite.

```snap
// In main.sp
let score: int = 0;

// In Player sprite
on GreenFlag {
    set score = 0;  // Access global variable
}

// In Enemy sprite
on Broadcast("enemy_defeated") {
    change score by 100;  // Same global variable
}
```

## Using Variables

### In Expressions

```snap
if score > 100 {
    looks::Say("High score!");
}

let doubled = score * 2;
let message = operators::Join("Score: ", score);
```

### In Block Arguments

```snap
motion::Move(speed);
looks::SayTimed(player_name, units::Sec(2));
control::Repeat(level * 10) {
    // ...
}
```

### In Conditions

```snap
if lives == 0 {
    events::Broadcast("game_over");
}

control::RepeatUntil(score >= 1000) {
    // Game loop
}

if game_over == false {
    // Still playing
}
```

## Examples

### Score System

```snap
let score: int = 0;
let high_score: int = 0;

new Sprite("ScoreManager") {
    implements Code {
        on GreenFlag {
            set score = 0;
        }

        on Broadcast("add_points") {
            change score by 10;

            if score > high_score {
                set high_score = score;
            }
        }

        on GreenFlag {
            control::Forever {
                looks::Say(operators::Join("Score: ", score));
            }
        }
    }
}
```

### Lives System

```snap
let lives: int = 3;

new Sprite("Player") {
    implements Code {
        on GreenFlag {
            set lives = 3;
        }

        on Broadcast("player_hit") {
            change lives by -1;

            if lives <= 0 {
                events::Broadcast("game_over");
            } else {
                // Respawn
                motion::GoToXY(0, -100);
            }
        }
    }
}
```

### Speed Control

```snap
let speed: float = 5.0;

new Sprite("Player") {
    implements Code {
        on GreenFlag {
            set speed = 5.0;

            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(speed);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-speed);
                }
            }
        }

        on KeyPressed("up arrow") {
            change speed by 1.0;
        }

        on KeyPressed("down arrow") {
            change speed by -1.0;
        }
    }
}
```

### Game State

```snap
let game_started: bool = false;
let game_over: bool = false;
let paused: bool = false;

new Stage {
    implements Code {
        on GreenFlag {
            set game_started = false;
            set game_over = false;
            set paused = false;
        }

        on KeyPressed("space") {
            if game_started == false {
                set game_started = true;
                events::Broadcast("start_game");
            }
        }

        on KeyPressed("p") {
            if paused {
                set paused = false;
                events::Broadcast("resume");
            } else {
                set paused = true;
                events::Broadcast("pause");
            }
        }
    }
}
```

### Level System

```snap
let level: int = 1;
let enemies_remaining: int = 10;

on GreenFlag {
    set level = 1;
    set enemies_remaining = 10;
}

on Broadcast("enemy_defeated") {
    change enemies_remaining by -1;

    if enemies_remaining == 0 {
        change level by 1;
        set enemies_remaining = level * 10;  // More enemies each level
        events::Broadcast("next_level");
    }
}
```

## Best Practices

1. **Initialize in GreenFlag** - Always reset variables when the green flag is clicked

2. **Use descriptive names** - `player_score` is better than `ps`

3. **Use appropriate types** - Use `int` for counts, `float` for speeds, `bool` for flags

4. **Group related variables** - Keep score, lives, and level variables together

5. **Comment complex variables** - Explain what the variable is used for

```snap
// Game state
let score: int = 0;           // Player's current score
let high_score: int = 0;      // Best score this session
let lives: int = 3;           // Remaining lives
let level: int = 1;           // Current level (1-10)

// Player settings
let speed: float = 5.0;       // Movement speed (pixels per frame)
let invincible: bool = false; // True during brief invincibility
```

## Future Features

> **Planned:** Local variables (sprite-scoped) and lists are planned for future versions.
