# Sprites & Stage

Sprites and the Stage are the core building blocks of any Scratch project.

## Sprites

Sprites are the characters and objects in your project. Create a sprite with the `new Sprite` syntax:

```snap
new Sprite("MySprite") {
    implements Code {
        // Event handlers and code go here
    }
}
```

### Sprite Properties

Sprites can have optional properties:

```snap
new Sprite("Player") {
    position: (0, -100),    // Starting x, y position
    size: 75,               // Size percentage (100 = normal)
    costumes: ["player.png", "player2.png"],  // Costume files (planned)
    
    implements Code {
        on GreenFlag {
            looks::Say("Ready!");
        }
    }
}
```

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `position` | `(x, y)` | `(0, 0)` | Starting position |
| `size` | `number` | `100` | Size percentage |
| `costumes` | `[strings]` | Default costume | List of costume files |

### Multiple Sprites

You can define multiple sprites in a single file:

```snap
new Sprite("Player") {
    position: (0, -100),
    
    implements Code {
        on GreenFlag {
            looks::Say("I'm the player!");
        }
    }
}

new Sprite("Enemy") {
    position: (100, 50),
    
    implements Code {
        on GreenFlag {
            looks::Say("I'm the enemy!");
        }
    }
}

new Sprite("Collectible") {
    position: (-100, 50),
    
    implements Code {
        on GreenFlag {
            looks::Say("Collect me!");
        }
    }
}
```

## The Stage

The Stage is a special target that represents the background. There can only be one Stage per project:

```snap
new Stage {
    backdrops: ["backdrop1.png", "backdrop2.png"],  // Backdrop files (planned)
    
    implements Code {
        on GreenFlag {
            looks::SwitchBackdrop("backdrop1");
        }
    }
}
```

### Stage vs Sprites

| Feature | Stage | Sprite |
|---------|-------|--------|
| Position | Fixed at (0, 0) | Movable |
| Size | Fixed | Changeable |
| Costumes | Backdrops | Costumes |
| Motion blocks | Not available | Available |
| Layer | Always at back | Configurable |

### Stage-Specific Blocks

The Stage can use most blocks except motion blocks:

```snap
new Stage {
    implements Code {
        on GreenFlag {
            looks::SwitchBackdrop("level1");
        }
        
        on BackdropSwitch("level2") {
            events::Broadcast("start_level_2");
        }
    }
}
```

## Code Blocks

The `implements Code` block contains all the sprite's programming:

```snap
new Sprite("Example") {
    implements Code {
        // Event handlers
        on GreenFlag {
            // ...
        }
        
        on KeyPressed("space") {
            // ...
        }
        
        // Custom functions
        fn jump(height: int) {
            // ...
        }
    }
}
```

## Sprite Interactions

### Touching Detection

```snap
if sensing::TouchingSprite("Enemy") {
    looks::Say("Ouch!");
}

if sensing::TouchingEdge {
    motion::IfOnEdgeBounce();
}
```

### Distance

```snap
let dist = sensing::DistanceTo("Goal");
if dist < 50 {
    looks::Say("Almost there!");
}
```

### Broadcasting

Sprites can communicate using broadcasts:

```snap
// In Sprite 1
on GreenFlag {
    events::Broadcast("game_start");
}

// In Sprite 2
on Broadcast("game_start") {
    looks::Show();
    motion::GoToXY(0, 0);
}
```

## Cloning

Create copies of sprites at runtime:

```snap
new Sprite("Bullet") {
    implements Code {
        on GreenFlag {
            looks::Hide();  // Hide the original
        }
        
        on CloneStart {
            looks::Show();
            control::Repeat(50) {
                motion::Move(10);
            }
            control::DeleteClone();
        }
    }
}

new Sprite("Player") {
    implements Code {
        on KeyPressed("space") {
            control::CreateClone("Bullet");
        }
    }
}
```

## Best Practices

1. **Use descriptive names** - Name sprites based on their role: "Player", "Enemy", "ScoreDisplay"

2. **Organize with imports** - Put each sprite in its own file for larger projects

3. **Initialize in GreenFlag** - Reset sprite state when the green flag is clicked

4. **Use broadcasts for coordination** - Communicate between sprites using broadcasts rather than checking variables constantly

## Next Steps

- [Events](./events.md) - Learn about event handlers
- [Blocks](./blocks.md) - Complete block reference
- [Imports](./imports.md) - Organize code across files
