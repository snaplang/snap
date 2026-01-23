# Imports

Imports allow you to organize your code across multiple files.

## Basic Imports

Use the `use` keyword to import another `.sp` file:

```snap
use "sprites/player.sp";
use "sprites/enemy.sp";
use "sprites/ui.sp";
```

Import statements must be at the top of the file, before any other code.

## Project Structure

A well-organized project might look like:

```
my_game/
├── config.toml
├── src/
│   ├── main.sp           # Main file with imports
│   ├── sprites/
│   │   ├── player.sp     # Player sprite
│   │   ├── enemy.sp      # Enemy sprite
│   │   └── coin.sp       # Collectible sprite
│   └── stage.sp          # Stage definition
└── assets/
    ├── costumes/
    └── sounds/
```

## Example

### main.sp

```snap
// Import all game components
use "sprites/player.sp";
use "sprites/enemy.sp";
use "sprites/coin.sp";
use "stage.sp";

// Global variables
let score: int = 0;
let lives: int = 3;
let level: int = 1;
```

### sprites/player.sp

```snap
new Sprite("Player") {
    position: (0, -100),
    
    implements Code {
        on GreenFlag {
            motion::GoToXY(0, -100);
            looks::Show();
            
            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(5);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-5);
                }
            }
        }
        
        on Broadcast("player_hit") {
            change lives by -1;
            motion::GoToXY(0, -100);
        }
    }
}
```

### sprites/enemy.sp

```snap
new Sprite("Enemy") {
    position: (0, 100),
    
    implements Code {
        on GreenFlag {
            looks::Show();
            
            control::Forever {
                motion::PointTowards("Player");
                motion::Move(2);
                
                if sensing::TouchingSprite("Player") {
                    events::Broadcast("player_hit");
                    motion::GoToXY(
                        operators::Random(-200, 200),
                        operators::Random(50, 150)
                    );
                }
            }
        }
    }
}
```

### sprites/coin.sp

```snap
new Sprite("Coin") {
    position: (100, 0),
    
    implements Code {
        on GreenFlag {
            looks::Show();
            
            control::Forever {
                if sensing::TouchingSprite("Player") {
                    change score by 10;
                    sound::Play("coin");
                    
                    // Respawn at random position
                    motion::GoToXY(
                        operators::Random(-200, 200),
                        operators::Random(-100, 100)
                    );
                }
            }
        }
        
        // Spinning animation
        on GreenFlag {
            control::Forever {
                motion::TurnRight(5);
            }
        }
    }
}
```

### stage.sp

```snap
new Stage {
    implements Code {
        on GreenFlag {
            set score = 0;
            set lives = 3;
            set level = 1;
        }
        
        on Broadcast("next_level") {
            change level by 1;
            looks::NextBackdrop();
        }
    }
}
```

## Import Rules

### File Paths

- Paths are relative to the project root (where `config.toml` is)
- Use forward slashes (`/`) even on Windows
- Include the `.sp` extension

```snap
// Correct
use "src/sprites/player.sp";
use "sprites/player.sp";

// Incorrect
use "sprites\\player.sp";   // Wrong slash
use "sprites/player";        // Missing extension
```

### What Gets Imported

When you import a file, all of its contents are merged:

- Sprites are added to the project
- Global variables are combined
- The Stage definition (if present) is used

### Stage Restriction

Only one Stage can be defined across all files:

```snap
// main.sp
use "other.sp";

new Stage { }  // Error if other.sp also defines Stage
```

### Variable Sharing

Global variables are shared across all files:

```snap
// main.sp
let score: int = 0;
use "player.sp";

// player.sp
// Can use 'score' variable
on GreenFlag {
    change score by 10;  // Works!
}
```

## Organizing Large Projects

### By Feature

```
src/
├── main.sp
├── player/
│   ├── player.sp
│   └── player_abilities.sp
├── enemies/
│   ├── basic_enemy.sp
│   └── boss.sp
├── items/
│   ├── coin.sp
│   └── powerup.sp
└── ui/
    ├── score_display.sp
    └── lives_display.sp
```

### By Type

```
src/
├── main.sp
├── sprites.sp      # All sprite definitions
├── stage.sp        # Stage definition
├── variables.sp    # Global variables
└── functions.sp    # Shared functions (future)
```

## Best Practices

1. **One sprite per file** - Makes code easier to find and maintain

2. **Group related files** - Use folders for organization

3. **Import order** - Import files in a logical order (dependencies first)

4. **Shared variables in main.sp** - Define global variables in the main file

5. **Comment imports** - Explain what each import provides

```snap
// Game entities
use "sprites/player.sp";    // Player character and controls
use "sprites/enemy.sp";     // Enemy AI and behavior
use "sprites/coin.sp";      // Collectible items

// UI elements
use "ui/hud.sp";            // Score and lives display
use "ui/menu.sp";           // Start menu

// Stage
use "stage.sp";             // Background and level logic
```

## Troubleshooting

### File Not Found

```
Error reading import sprites/player.sp: No such file
```

- Check the file path is correct
- Ensure the file exists
- Verify you're running from the project root

### Multiple Stages

```
Multiple Stage definitions found across files
```

- Only define `new Stage { }` in one file
- Remove Stage from imported files

### Undefined Variable

- Make sure the variable is declared before the import that uses it
- Or declare variables in imported files
