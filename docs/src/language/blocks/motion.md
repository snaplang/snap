# Motion Blocks

Motion blocks control sprite movement and position. These blocks are **not available for the Stage**.

## Movement

### Move

Move the sprite forward in its current direction.

```snap
motion::Move(10);      // Move 10 steps
motion::Move(steps);   // Move variable steps
```

### Turn

Rotate the sprite.

```snap
motion::TurnRight(15);   // Turn clockwise 15 degrees
motion::TurnLeft(15);    // Turn counter-clockwise 15 degrees
```

## Position

### Go To

Instantly move to a position.

```snap
motion::GoToXY(0, 0);           // Go to center
motion::GoToXY(100, -50);       // Go to specific coordinates
motion::GoToXY(x, y);           // Go to variable position
```

### Glide To

Smoothly glide to a position over time.

```snap
motion::GlideToXY(units::Sec(1), 100, 50);   // Glide over 1 second
motion::GlideToXY(units::Sec(0.5), x, y);    // Glide to variables
```

### Set Position

Set individual coordinates.

```snap
motion::SetX(100);    // Set x position
motion::SetY(-50);    // Set y position
```

### Change Position

Change position by an amount.

```snap
motion::ChangeX(10);    // Move right 10
motion::ChangeX(-10);   // Move left 10
motion::ChangeY(10);    // Move up 10
motion::ChangeY(-10);   // Move down 10
```

## Direction

### Point In Direction

Set the sprite's direction.

```snap
motion::PointInDirection(90);    // Point right
motion::PointInDirection(0);     // Point up
motion::PointInDirection(180);   // Point down
motion::PointInDirection(-90);   // Point left
```

Direction values:
- `0` = Up
- `90` = Right
- `180` / `-180` = Down
- `-90` = Left

### Point Towards

Point towards another sprite or the mouse.

```snap
motion::PointTowards("Player");       // Point at Player sprite
motion::PointTowards("_mouse_");      // Point at mouse cursor
```

## Edge Handling

### If On Edge, Bounce

Bounce when hitting the edge of the stage.

```snap
control::Forever {
    motion::Move(10);
    motion::IfOnEdgeBounce();
}
```

## Rotation Style

Set how the sprite rotates.

```snap
motion::SetRotationStyle("all around");     // Normal rotation
motion::SetRotationStyle("left-right");     // Flip horizontally only
motion::SetRotationStyle("don't rotate");   // No rotation
```

## Reporter Blocks

These blocks return values and can be used in expressions.

### Position Reporters

```snap
let x = motion::XPosition;    // Current x position (-240 to 240)
let y = motion::YPosition;    // Current y position (-180 to 180)
```

### Direction Reporter

```snap
let dir = motion::Direction;   // Current direction (-180 to 180)
```

## Examples

### Smooth Movement

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

### Follow Mouse

```snap
on GreenFlag {
    control::Forever {
        motion::GoToXY(sensing::MouseX, sensing::MouseY);
    }
}
```

### Patrol Back and Forth

```snap
on GreenFlag {
    motion::GoToXY(-200, 0);
    motion::PointInDirection(90);
    
    control::Forever {
        motion::Move(5);
        motion::IfOnEdgeBounce();
    }
}
```

### Orbit Around Center

```snap
on GreenFlag {
    motion::GoToXY(100, 0);
    
    control::Forever {
        motion::TurnRight(5);
        motion::Move(10);
        motion::TurnRight(5);
    }
}
```
