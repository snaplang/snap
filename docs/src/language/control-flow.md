# Control Flow

Control flow statements determine the order in which code executes.

## Conditionals

### If Statement

Execute code only if a condition is true:

```snap
if score > 100 {
    looks::Say("High score!");
}
```

### If-Else Statement

Execute different code based on a condition:

```snap
if lives > 0 {
    looks::Say("Keep playing!");
} else {
    looks::Say("Game Over");
}
```

### Nested Conditions

```snap
if score > 1000 {
    looks::Say("Amazing!");
} else {
    if score > 500 {
        looks::Say("Great!");
    } else {
        if score > 100 {
            looks::Say("Good!");
        } else {
            looks::Say("Keep trying!");
        }
    }
}
```

### Condition Expressions

Conditions can use:

**Comparisons:**
```snap
if score == 100 { }     // Equal
if score != 0 { }       // Not equal
if score > 50 { }       // Greater than
if score < 50 { }       // Less than
if score >= 50 { }      // Greater than or equal
if score <= 50 { }      // Less than or equal
```

**Logical operators:**
```snap
if score > 100 && lives > 0 { }   // AND
if gameOver || paused { }          // OR
if !gameStarted { }                // NOT
```

**Boolean reporters:**
```snap
if sensing::KeyPressed("space") { }
if sensing::TouchingSprite("Enemy") { }
if sensing::MouseDown { }
```

**Combined:**
```snap
if sensing::KeyPressed("space") && !jumping {
    set jumping = true;
    events::Broadcast("jump");
}
```

## Loops

### Forever Loop

Repeat indefinitely:

```snap
control::Forever {
    motion::Move(1);
    motion::IfOnEdgeBounce();
}
```

> **Note:** Code after a `Forever` block never runs.

### Repeat Loop

Repeat a specific number of times:

```snap
control::Repeat(10) {
    motion::Move(10);
    motion::TurnRight(36);
}
// This runs after the loop completes
looks::Say("Done!");
```

### Repeat Until Loop

Repeat until a condition becomes true:

```snap
control::RepeatUntil(sensing::TouchingSprite("Goal")) {
    motion::Move(5);
}
looks::Say("Reached the goal!");
```

```snap
control::RepeatUntil(score >= 100) {
    // Game loop
    if sensing::TouchingSprite("Coin") {
        change score by 10;
    }
}
events::Broadcast("level_complete");
```

## Waiting

### Wait

Pause execution:

```snap
looks::Say("Ready?");
control::Wait(units::Sec(1));
looks::Say("Set...");
control::Wait(units::Sec(1));
looks::Say("Go!");
```

## Stopping

### Stop All

Stop all scripts in all sprites:

```snap
if lives == 0 {
    looks::Say("Game Over!");
    control::Stop("all");
}
```

### Stop This Script

Stop only the current script:

```snap
on GreenFlag {
    control::Forever {
        if gameOver {
            control::Stop("this script");  // Exit the forever loop
        }
        // Game logic
    }
}
```

### Stop Other Scripts

Stop other scripts in this sprite:

```snap
on Broadcast("freeze") {
    control::Stop("other scripts in sprite");
    looks::Say("Frozen!");
}
```

## Patterns

### Game Loop Pattern

```snap
on GreenFlag {
    // Initialization
    set score = 0;
    set lives = 3;
    set gameOver = false;
    
    // Main game loop
    control::RepeatUntil(gameOver) {
        // Input handling
        if sensing::KeyPressed("right arrow") {
            motion::ChangeX(5);
        }
        
        // Collision detection
        if sensing::TouchingSprite("Enemy") {
            change lives by -1;
            if lives == 0 {
                set gameOver = true;
            }
        }
        
        // Scoring
        if sensing::TouchingSprite("Coin") {
            change score by 10;
        }
    }
    
    // Game over
    looks::Say("Game Over!");
}
```

### State Machine Pattern

```snap
let state: string = "menu";

on GreenFlag {
    set state = "menu";
    
    control::Forever {
        if state == "menu" {
            // Menu logic
            looks::Say("Press SPACE to start");
            if sensing::KeyPressed("space") {
                set state = "playing";
            }
        }
        
        if state == "playing" {
            // Game logic
            looks::Say("");
            // ...
            if lives == 0 {
                set state = "gameover";
            }
        }
        
        if state == "gameover" {
            looks::Say("Game Over! Press R to restart");
            if sensing::KeyPressed("r") {
                set state = "menu";
                set lives = 3;
                set score = 0;
            }
        }
    }
}
```

### Animation Loop

```snap
on GreenFlag {
    control::Forever {
        looks::NextCostume();
        control::Wait(units::Sec(0.1));
    }
}
```

### Countdown Pattern

```snap
on Broadcast("start_countdown") {
    control::Repeat(3) {
        looks::SayTimed("3", units::Sec(1));
        looks::SayTimed("2", units::Sec(1));
        looks::SayTimed("1", units::Sec(1));
    }
    looks::SayTimed("Go!", units::Sec(0.5));
    events::Broadcast("game_start");
}
```

### Polling Pattern

```snap
// Check something continuously
on GreenFlag {
    control::Forever {
        if sensing::TouchingSprite("Button") && sensing::MouseDown {
            events::Broadcast("button_clicked");
            // Debounce - wait for release
            control::RepeatUntil(!sensing::MouseDown) {
                // Wait
            }
        }
    }
}
```

### Parallel Actions

Multiple event handlers run simultaneously:

```snap
new Sprite("Player") {
    implements Code {
        // Movement loop
        on GreenFlag {
            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(5);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-5);
                }
            }
        }
        
        // Animation loop (runs in parallel)
        on GreenFlag {
            control::Forever {
                looks::NextCostume();
                control::Wait(units::Sec(0.2));
            }
        }
        
        // Collision detection (runs in parallel)
        on GreenFlag {
            control::Forever {
                if sensing::TouchingSprite("Enemy") {
                    events::Broadcast("player_hit");
                    control::Wait(units::Sec(1));  // Cooldown
                }
            }
        }
    }
}
```
