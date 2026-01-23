# Sensing Blocks

Sensing blocks detect user input, sprite collisions, and other environmental information.

## Touch Detection

### Touching Sprite

Check if touching another sprite.

```snap
if sensing::TouchingSprite("Enemy") {
    looks::Say("Ouch!");
}
```

### Touching Edge

Check if touching the edge of the stage.

```snap
if sensing::TouchingEdge {
    motion::IfOnEdgeBounce();
}
```

### Touching Color

Check if touching a specific color.

```snap
if sensing::TouchingColor("#FF0000") {
    looks::Say("Touching red!");
}
```

## Keyboard Input

### Key Pressed

Check if a key is currently pressed.

```snap
if sensing::KeyPressed("space") {
    looks::Say("Space pressed!");
}

if sensing::KeyPressed("right arrow") {
    motion::ChangeX(10);
}
```

#### Key Names

| Key | Name |
|-----|------|
| Arrow keys | `"right arrow"`, `"left arrow"`, `"up arrow"`, `"down arrow"` |
| Space | `"space"` |
| Enter | `"enter"` |
| Letters | `"a"` through `"z"` |
| Numbers | `"0"` through `"9"` |
| Any key | `"any"` |

## Mouse Input

### Mouse Down

Check if the mouse button is pressed.

```snap
if sensing::MouseDown {
    motion::GoToXY(sensing::MouseX, sensing::MouseY);
}
```

### Mouse Position

Get the mouse cursor position.

```snap
let mx = sensing::MouseX;    // -240 to 240
let my = sensing::MouseY;    // -180 to 180

motion::GoToXY(sensing::MouseX, sensing::MouseY);
```

## Distance

### Distance To

Get the distance to another sprite or the mouse.

```snap
let dist = sensing::DistanceTo("Goal");
if dist < 50 {
    looks::Say("Getting close!");
}

let mouse_dist = sensing::DistanceTo("_mouse_");
```

## Asking Questions

### Ask and Wait

Ask the user a question and wait for input.

```snap
sensing::AskAndWait("What is your name?");
looks::SayTimed(operators::Join("Hello, ", sensing::Answer), units::Sec(2));
```

### Answer

Get the most recent answer to an `Ask` block.

```snap
let name = sensing::Answer;
```

## Timer

### Timer

Get the time since the project started (or timer was reset).

```snap
let elapsed = sensing::Timer;

if sensing::Timer > 30 {
    events::Broadcast("time_up");
}
```

### Reset Timer

Reset the timer to 0.

```snap
sensing::ResetTimer();
```

## Loudness

### Loudness

Get the microphone loudness (0-100).

```snap
if sensing::Loudness > 50 {
    looks::Say("That's loud!");
}
```

## Examples

### Keyboard Movement

```snap
on GreenFlag {
    control::Forever {
        if sensing::KeyPressed("right arrow") {
            motion::ChangeX(5);
        }
        if sensing::KeyPressed("left arrow") {
            motion::ChangeX(-5);
        }
        if sensing::KeyPressed("up arrow") {
            motion::ChangeY(5);
        }
        if sensing::KeyPressed("down arrow") {
            motion::ChangeY(-5);
        }
    }
}
```

### Mouse Following

```snap
on GreenFlag {
    control::Forever {
        motion::PointTowards("_mouse_");
        if sensing::MouseDown {
            motion::Move(5);
        }
    }
}
```

### Collision Detection

```snap
on GreenFlag {
    control::Forever {
        if sensing::TouchingSprite("Coin") {
            change score by 10;
            events::Broadcast("coin_collected");
        }
        
        if sensing::TouchingSprite("Enemy") {
            change lives by -1;
            events::Broadcast("player_hit");
            control::Wait(units::Sec(1));  // Brief invincibility
        }
        
        if sensing::TouchingEdge {
            motion::IfOnEdgeBounce();
        }
    }
}
```

### Name Input

```snap
on GreenFlag {
    sensing::AskAndWait("What's your name?");
    set player_name = sensing::Answer;
    looks::SayTimed(operators::Join("Welcome, ", player_name), units::Sec(2));
}
```

### Timer-Based Game

```snap
let time_remaining: int = 60;

on GreenFlag {
    sensing::ResetTimer();
    set time_remaining = 60;
    
    control::Forever {
        set time_remaining = 60 - sensing::Timer;
        
        if time_remaining <= 0 {
            events::Broadcast("game_over");
            control::Stop("this script");
        }
    }
}
```

### Sound Activated

```snap
on GreenFlag {
    control::Forever {
        if sensing::Loudness > 30 {
            looks::NextCostume();
            motion::ChangeY(10);
        } else {
            motion::ChangeY(-2);
        }
        
        // Don't fall through floor
        if motion::YPosition < -150 {
            motion::SetY(-150);
        }
    }
}
```

### Drag and Drop

```snap
on GreenFlag {
    control::Forever {
        if sensing::MouseDown {
            if sensing::DistanceTo("_mouse_") < 50 {
                motion::GoToXY(sensing::MouseX, sensing::MouseY);
            }
        }
    }
}
```
