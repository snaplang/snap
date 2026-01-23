# Events

Events are the entry points for your code. They define when blocks of code should run.

## Event Handlers

Event handlers are defined with the `on` keyword inside an `implements Code` block:

```snap
new Sprite("Example") {
    implements Code {
        on GreenFlag {
            // Runs when green flag is clicked
        }

        on KeyPressed("space") {
            // Runs when space is pressed
        }
    }
}
```

## Available Events

### GreenFlag

Runs when the green flag is clicked. This is the most common event for initializing sprites.

```snap
on GreenFlag {
    motion::GoToXY(0, 0);
    looks::Show();
    set score = 0;
}
```

### KeyPressed

Runs when a specific key is pressed.

```snap
on KeyPressed("space") {
    looks::Say("Jump!");
}

on KeyPressed("a") {
    motion::ChangeX(-10);
}
```

#### Key Names

| Key        | Name                                                          |
| ---------- | ------------------------------------------------------------- |
| Arrow keys | `"right arrow"`, `"left arrow"`, `"up arrow"`, `"down arrow"` |
| Space      | `"space"`                                                     |
| Enter      | `"enter"`                                                     |
| Letters    | `"a"` through `"z"`                                           |
| Numbers    | `"0"` through `"9"`                                           |
| Any key    | `"any"`                                                       |

### Clicked

Runs when this sprite is clicked.

```snap
on Clicked {
    looks::SayTimed("You clicked me!", units::Sec(2));
    looks::NextCostume();
}
```

### Broadcast

Runs when a specific broadcast message is received.

```snap
on Broadcast("game_over") {
    looks::Say("Game Over!");
    control::Stop("other scripts in sprite");
}

on Broadcast("level_complete") {
    looks::SayTimed("Level Complete!", units::Sec(2));
}
```

### BackdropSwitch

Runs when the backdrop switches to a specific backdrop. Only works on the Stage and sprites.

```snap
on BackdropSwitch("level2") {
    motion::GoToXY(-200, 0);
    set speed = 5;
}
```

### CloneStart

Runs when this sprite starts as a clone.

```snap
on CloneStart {
    looks::Show();
    control::Repeat(100) {
        motion::Move(5);
        if sensing::TouchingEdge {
            control::DeleteClone();
        }
    }
    control::DeleteClone();
}
```

## Multiple Event Handlers

A sprite can have multiple handlers for the same event:

```snap
new Sprite("Player") {
    implements Code {
        // First GreenFlag handler - setup
        on GreenFlag {
            motion::GoToXY(0, 0);
            looks::Show();
        }

        // Second GreenFlag handler - main loop
        on GreenFlag {
            control::Forever {
                // Movement code
            }
        }
    }
}
```

Both handlers run simultaneously when the green flag is clicked, just like in Scratch.

## Broadcasting

Send broadcasts to trigger `on Broadcast` handlers:

```snap
// Send a broadcast (doesn't wait)
events::Broadcast("my_message");

// Send a broadcast and wait for all handlers to finish
events::BroadcastAndWait("my_message");
```

### Broadcast Patterns

#### Game State Management

```snap
// In Stage
on GreenFlag {
    events::Broadcast("setup");
    events::BroadcastAndWait("intro");
    events::Broadcast("start_game");
}

// In Player sprite
on Broadcast("setup") {
    motion::GoToXY(0, -100);
    set lives = 3;
}

on Broadcast("start_game") {
    control::Forever {
        // Game loop
    }
}
```

#### Triggering Actions

```snap
// In Player sprite
on GreenFlag {
    control::Forever {
        if sensing::TouchingSprite("Enemy") {
            events::Broadcast("player_hit");
        }
    }
}

// In Enemy sprite
on Broadcast("player_hit") {
    looks::SayTimed("Got you!", units::Sec(1));
}

// In Lives Display sprite
on Broadcast("player_hit") {
    change lives by -1;
    if lives == 0 {
        events::Broadcast("game_over");
    }
}
```

## Event Flow

```
User clicks green flag
        │
        ▼
┌─────────────────┐
│ on GreenFlag    │──► All sprites' GreenFlag handlers
│ handlers run    │    run simultaneously
└─────────────────┘
        │
        ▼
┌─────────────────┐
│ Broadcast sent  │──► events::Broadcast("start")
└─────────────────┘
        │
        ▼
┌─────────────────┐
│ on Broadcast    │──► All matching handlers
│ handlers run    │    run simultaneously
└─────────────────┘
```

## Best Practices

1. **Use GreenFlag for initialization** - Reset all variables and positions

2. **Name broadcasts descriptively** - Use names like `"game_over"`, `"level_complete"`, `"player_hit"`

3. **Use BroadcastAndWait for sequences** - When you need things to happen in order

4. **Avoid too many broadcasts** - They can make code hard to follow

5. **Document broadcast contracts** - Comment what each broadcast means and who sends/receives it

## Next Steps

- [Blocks](./blocks.md) - Complete block reference
- [Control Flow](./control-flow.md) - Loops and conditionals
