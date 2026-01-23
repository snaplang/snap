# Control Blocks

Control blocks manage program flow, including loops, waits, and cloning.

## Waiting

### Wait

Pause execution for a duration.

```snap
control::Wait(units::Sec(1));      // Wait 1 second
control::Wait(units::Sec(0.5));    // Wait half a second
control::Wait(units::Sec(delay));  // Wait variable seconds
```

## Loops

### Forever

Repeat blocks indefinitely.

```snap
control::Forever {
    motion::Move(1);
    motion::IfOnEdgeBounce();
}
```

> **Note:** Code after a `Forever` block will never run.

### Repeat

Repeat blocks a specific number of times.

```snap
control::Repeat(10) {
    motion::Move(10);
    motion::TurnRight(36);
}
```

### Repeat Until

Repeat blocks until a condition is true.

```snap
control::RepeatUntil(sensing::TouchingSprite("Goal")) {
    motion::Move(5);
}
looks::Say("Made it!");
```

```snap
control::RepeatUntil(score >= 100) {
    // Game loop
}
events::Broadcast("game_won");
```

## Conditionals

### If

Run blocks if a condition is true.

```snap
if score > 100 {
    looks::Say("High score!");
}
```

### If-Else

Run different blocks based on a condition.

```snap
if lives > 0 {
    looks::Say("Keep going!");
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
        looks::Say("Good start!");
    }
}
```

## Stopping

### Stop

Stop scripts from running.

```snap
control::Stop("all");                      // Stop everything
control::Stop("this script");              // Stop only this script
control::Stop("other scripts in sprite"); // Stop other scripts in this sprite
```

#### Stop Modes

| Mode | Effect |
|------|--------|
| `"all"` | Stops all scripts in all sprites |
| `"this script"` | Stops only the current script |
| `"other scripts in sprite"` | Stops other scripts in this sprite |

## Cloning

### Create Clone

Create a copy of a sprite.

```snap
control::CreateClone("_myself_");     // Clone this sprite
control::CreateClone("Bullet");       // Clone another sprite
```

### Delete Clone

Delete the current clone.

```snap
control::DeleteClone();
```

This only works in clones - it does nothing in the original sprite.

### Clone Lifecycle

```snap
new Sprite("Bullet") {
    implements Code {
        // Original sprite - hide it
        on GreenFlag {
            looks::Hide();
        }
        
        // Clone behavior
        on CloneStart {
            looks::Show();
            motion::GoToXY(sensing::MouseX, sensing::MouseY);
            
            control::Repeat(50) {
                motion::Move(10);
                if sensing::TouchingEdge {
                    control::DeleteClone();
                }
            }
            control::DeleteClone();
        }
    }
}
```

## Examples

### Game Loop Pattern

```snap
on GreenFlag {
    // Setup
    set score = 0;
    set lives = 3;
    motion::GoToXY(0, -100);
    
    // Main game loop
    control::RepeatUntil(lives == 0) {
        // Handle input
        if sensing::KeyPressed("right arrow") {
            motion::ChangeX(5);
        }
        if sensing::KeyPressed("left arrow") {
            motion::ChangeX(-5);
        }
        
        // Check collisions
        if sensing::TouchingSprite("Enemy") {
            change lives by -1;
            motion::GoToXY(0, -100);
        }
        
        if sensing::TouchingSprite("Coin") {
            change score by 10;
        }
    }
    
    // Game over
    looks::Say("Game Over!");
    control::Stop("all");
}
```

### Countdown Timer

```snap
let time_left: int = 30;

on GreenFlag {
    set time_left = 30;
    
    control::RepeatUntil(time_left == 0) {
        control::Wait(units::Sec(1));
        change time_left by -1;
    }
    
    events::Broadcast("time_up");
}
```

### Spawn Enemies

```snap
on GreenFlag {
    control::Forever {
        control::CreateClone("_myself_");
        control::Wait(units::Sec(2));  // Spawn every 2 seconds
    }
}

on CloneStart {
    motion::GoToXY(operators::Random(-200, 200), 180);
    looks::Show();
    
    control::RepeatUntil(motion::YPosition < -180) {
        motion::ChangeY(-3);
        
        if sensing::TouchingSprite("Player") {
            events::Broadcast("player_hit");
            control::DeleteClone();
        }
    }
    control::DeleteClone();
}
```

### Delayed Action

```snap
on Broadcast("start_sequence") {
    looks::SayTimed("3", units::Sec(1));
    looks::SayTimed("2", units::Sec(1));
    looks::SayTimed("1", units::Sec(1));
    looks::SayTimed("Go!", units::Sec(0.5));
    events::Broadcast("game_start");
}
```
