# Your First Project

In this tutorial, we'll build a simple interactive project with a sprite that moves around and responds to user input.

## Project Setup

Create a new project:

```bash
snap new my_game
cd my_game
```

## Understanding the Code

Open `src/main.sp` in your favorite editor. Let's replace it with something more interesting:

```snap
// My first Snap game!

// Define a global variable to track the score
let score: int = 0;

// Create the Stage
new Stage {
    implements Code {
        on GreenFlag {
            set score = 0;
        }
    }
}

// Create the player sprite
new Sprite("Player") {
    position: (0, -100),
    
    implements Code {
        on GreenFlag {
            looks::Say("Use arrow keys to move!");
            control::Wait(units::Sec(2));
            looks::Say("");
            
            control::Forever {
                // Move right
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(5);
                }
                
                // Move left
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-5);
                }
                
                // Move up
                if sensing::KeyPressed("up arrow") {
                    motion::ChangeY(5);
                }
                
                // Move down
                if sensing::KeyPressed("down arrow") {
                    motion::ChangeY(-5);
                }
                
                // Bounce off edges
                motion::IfOnEdgeBounce();
            }
        }
        
        on KeyPressed("space") {
            looks::SayTimed("Jump!", units::Sec(0.5));
        }
    }
}

// Create a collectible
new Sprite("Star") {
    position: (100, 50),
    
    implements Code {
        on GreenFlag {
            looks::Show();
            
            control::Forever {
                // Check if player touches the star
                if sensing::TouchingSprite("Player") {
                    // Hide and reposition
                    looks::Hide();
                    change score by 1;
                    
                    // Wait and respawn at random position
                    control::Wait(units::Sec(1));
                    motion::GoToXY(
                        operators::Random(-200, 200),
                        operators::Random(-100, 100)
                    );
                    looks::Show();
                }
            }
        }
    }
}
```

## Code Breakdown

### Global Variables

```snap
let score: int = 0;
```

Declares a global variable named `score` of type `int` with initial value `0`. This variable is accessible from all sprites.

### Stage Definition

```snap
new Stage {
    implements Code {
        on GreenFlag {
            set score = 0;
        }
    }
}
```

The Stage is a special target that represents the background. Here we reset the score when the green flag is clicked.

### Sprite Properties

```snap
new Sprite("Player") {
    position: (0, -100),
    
    implements Code {
        // ...
    }
}
```

Sprites can have optional properties:
- `position: (x, y)` - Starting position
- `size: number` - Size percentage (100 = normal)
- `costumes: ["file1.png", "file2.png"]` - Costume files (coming soon)

### Event Handlers

```snap
on GreenFlag {
    // Runs when green flag is clicked
}

on KeyPressed("space") {
    // Runs when space is pressed
}
```

### Control Flow

```snap
control::Forever {
    // Loops forever
}

if condition {
    // Runs if condition is true
}
```

### Block Calls

Blocks are organized by category:

```snap
motion::ChangeX(5);           // Motion category
looks::Say("Hello!");         // Looks category
control::Wait(units::Sec(1)); // Control category
sensing::KeyPressed("space"); // Sensing category (reporter)
```

## Build and Test

Build your project:

```bash
snap build --verbose
```

Output:

```
Compiling project: my_game
  Found 3 sprite(s)
  Found stage definition
Finished my_game (my_game/my_game.sb3)
```

Open the generated `.sb3` file in Scratch and click the green flag to play!

## Exercises

Try extending the game:

1. **Add a score display** - Use the Scratch editor to add a score monitor
2. **Add more collectibles** - Clone the Star sprite code with different positions
3. **Add obstacles** - Create sprites that end the game when touched
4. **Add sound effects** - Use `sound::Play()` blocks

## Next Steps

- [Basic Syntax](../language/syntax.md) - Learn the full language syntax
- [Blocks Reference](../language/blocks.md) - See all available blocks
- [Examples](../examples/hello-world.md) - More example projects
