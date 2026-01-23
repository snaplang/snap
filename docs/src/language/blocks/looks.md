# Looks Blocks

Looks blocks control sprite appearance, speech bubbles, and visual effects.

## Speech Bubbles

### Say

Display a speech bubble.

```snap
looks::Say("Hello!");              // Say indefinitely
looks::Say("");                     // Clear speech bubble
looks::Say(score);                  // Say a variable value
```

### Say for Time

Display a speech bubble for a duration.

```snap
looks::SayTimed("Hello!", units::Sec(2));    // Say for 2 seconds
looks::SayTimed(message, units::Sec(1));     // Say variable for 1 second
```

### Think

Display a thought bubble.

```snap
looks::Think("Hmm...");                      // Think indefinitely
looks::ThinkTimed("Hmm...", units::Sec(2)); // Think for 2 seconds
```

## Visibility

### Show and Hide

```snap
looks::Show();    // Make sprite visible
looks::Hide();    // Make sprite invisible
```

Hidden sprites:
- Cannot be clicked
- Can still run code
- Can still detect collisions (in Scratch)

## Costumes

### Switch Costume

Change to a specific costume.

```snap
looks::SwitchCostume("costume2");    // Switch by name
```

### Next Costume

Switch to the next costume in the list.

```snap
looks::NextCostume();
```

Useful for animations:

```snap
control::Forever {
    looks::NextCostume();
    control::Wait(units::Sec(0.1));
}
```

## Backdrops (Stage Only)

These blocks only work on the Stage.

### Switch Backdrop

```snap
looks::SwitchBackdrop("level1");     // Switch by name
```

### Next Backdrop

```snap
looks::NextBackdrop();
```

## Size

### Set Size

Set the sprite's size as a percentage.

```snap
looks::SetSize(100);    // Normal size
looks::SetSize(50);     // Half size
looks::SetSize(200);    // Double size
```

### Change Size

Change size by an amount.

```snap
looks::ChangeSize(10);     // Grow by 10%
looks::ChangeSize(-10);    // Shrink by 10%
```

## Graphic Effects

### Set Effect

Set a graphic effect to a value.

```snap
looks::SetEffect("color", 50);          // Shift color
looks::SetEffect("fisheye", 100);       // Fisheye effect
looks::SetEffect("whirl", 180);         // Whirl effect
looks::SetEffect("pixelate", 10);       // Pixelate
looks::SetEffect("mosaic", 5);          // Mosaic tiles
looks::SetEffect("brightness", 50);     // Brighten
looks::SetEffect("ghost", 50);          // 50% transparent
```

### Change Effect

Change an effect by an amount.

```snap
looks::ChangeEffect("color", 25);       // Shift color more
looks::ChangeEffect("ghost", 10);       // More transparent
```

### Clear Effects

Reset all graphic effects.

```snap
looks::ClearEffects();
```

## Layers

### Go to Layer

Move to front or back layer.

```snap
looks::GoToLayer("front");    // Move to front
looks::GoToLayer("back");     // Move to back
```

### Change Layer

Move forward or backward in layers.

```snap
looks::ChangeLayer("forward", 1);     // Move forward 1 layer
looks::ChangeLayer("backward", 2);    // Move backward 2 layers
```

## Reporter Blocks

### Costume Number

```snap
let costume_num = looks::CostumeNumber;
```

### Size

```snap
let current_size = looks::Size;
```

### Backdrop Number (Stage)

```snap
let backdrop_num = looks::BackdropNumber;
```

## Examples

### Animated Character

```snap
on GreenFlag {
    control::Forever {
        looks::NextCostume();
        control::Wait(units::Sec(0.2));
    }
}
```

### Fade In

```snap
on GreenFlag {
    looks::SetEffect("ghost", 100);  // Start invisible
    looks::Show();
    
    control::Repeat(20) {
        looks::ChangeEffect("ghost", -5);
        control::Wait(units::Sec(0.05));
    }
}
```

### Fade Out and Hide

```snap
fn fadeOut() {
    control::Repeat(20) {
        looks::ChangeEffect("ghost", 5);
        control::Wait(units::Sec(0.05));
    }
    looks::Hide();
    looks::ClearEffects();
}
```

### Pulsing Size

```snap
on GreenFlag {
    control::Forever {
        control::Repeat(10) {
            looks::ChangeSize(5);
            control::Wait(units::Sec(0.05));
        }
        control::Repeat(10) {
            looks::ChangeSize(-5);
            control::Wait(units::Sec(0.05));
        }
    }
}
```

### Color Cycling

```snap
on GreenFlag {
    control::Forever {
        looks::ChangeEffect("color", 5);
        control::Wait(units::Sec(0.1));
    }
}
```

### Speech Sequence

```snap
on GreenFlag {
    looks::SayTimed("Hello!", units::Sec(1));
    looks::SayTimed("Welcome to my game.", units::Sec(2));
    looks::SayTimed("Press SPACE to start!", units::Sec(2));
    looks::Say("");
}
```
