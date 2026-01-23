# Simple Game

A complete collection game with a player, collectibles, and score.

## The Game

- Move the player with arrow keys
- Collect stars to earn points
- Avoid enemies
- Try to get the highest score!

## Complete Code

### main.sp

```snap
// Simple Collection Game
// Collect stars, avoid enemies!

// === Global Variables ===
let score: int = 0;
let lives: int = 3;
let gameOver: bool = false;

// === Stage ===
new Stage {
    implements Code {
        on GreenFlag {
            set score = 0;
            set lives = 3;
            set gameOver = false;
        }
        
        on Broadcast("game_over") {
            set gameOver = true;
        }
    }
}

// === Player ===
new Sprite("Player") {
    position: (0, -120),
    
    implements Code {
        fn reset() {
            motion::GoToXY(0, -120);
            looks::Show();
        }
        
        fn flash() {
            control::Repeat(5) {
                looks::SetEffect("ghost", 50);
                control::Wait(units::Sec(0.1));
                looks::SetEffect("ghost", 0);
                control::Wait(units::Sec(0.1));
            }
        }
        
        on GreenFlag {
            reset();
            
            control::Forever {
                // Stop if game over
                if gameOver {
                    control::Stop("this script");
                }
                
                // Movement
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(6);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-6);
                }
                if sensing::KeyPressed("up arrow") {
                    motion::ChangeY(6);
                }
                if sensing::KeyPressed("down arrow") {
                    motion::ChangeY(-6);
                }
                
                // Keep on screen
                motion::IfOnEdgeBounce();
            }
        }
        
        // Collision with enemy
        on GreenFlag {
            control::Forever {
                if gameOver {
                    control::Stop("this script");
                }
                
                if sensing::TouchingSprite("Enemy") {
                    change lives by -1;
                    flash();
                    reset();
                    
                    if lives <= 0 {
                        events::Broadcast("game_over");
                    }
                }
            }
        }
        
        on Broadcast("game_over") {
            looks::SayTimed("Game Over!", units::Sec(3));
        }
    }
}

// === Star (Collectible) ===
new Sprite("Star") {
    position: (100, 50),
    
    implements Code {
        fn respawn() {
            motion::GoToXY(
                operators::Random(-200, 200),
                operators::Random(-100, 150)
            );
        }
        
        on GreenFlag {
            respawn();
            looks::Show();
            
            control::Forever {
                if gameOver {
                    control::Stop("this script");
                }
                
                // Spin animation
                motion::TurnRight(3);
                
                // Check collection
                if sensing::TouchingSprite("Player") {
                    change score by 10;
                    respawn();
                }
            }
        }
        
        on Broadcast("game_over") {
            looks::Hide();
        }
    }
}

// === Enemy ===
new Sprite("Enemy") {
    position: (-100, 100),
    
    implements Code {
        fn respawn() {
            motion::GoToXY(
                operators::Random(-200, 200),
                180
            );
        }
        
        on GreenFlag {
            respawn();
            looks::Show();
            
            control::Forever {
                if gameOver {
                    control::Stop("this script");
                }
                
                // Move down
                motion::ChangeY(-3);
                
                // Respawn at top if off screen
                if motion::YPosition < -180 {
                    respawn();
                }
            }
        }
        
        on Broadcast("game_over") {
            looks::Hide();
        }
    }
}

// === Score Display ===
new Sprite("ScoreDisplay") {
    position: (-180, 160),
    
    implements Code {
        on GreenFlag {
            looks::Show();
            
            control::Forever {
                looks::Say(operators::Join("Score: ", score));
            }
        }
    }
}

// === Lives Display ===
new Sprite("LivesDisplay") {
    position: (180, 160),
    
    implements Code {
        on GreenFlag {
            looks::Show();
            
            control::Forever {
                looks::Say(operators::Join("Lives: ", lives));
            }
        }
    }
}
```

## Code Breakdown

### Global State

```snap
let score: int = 0;
let lives: int = 3;
let gameOver: bool = false;
```

These variables are accessible from all sprites and track the game state.

### Stage

The Stage initializes global variables and listens for game over:

```snap
new Stage {
    implements Code {
        on GreenFlag {
            set score = 0;
            set lives = 3;
            set gameOver = false;
        }
    }
}
```

### Player

The Player has:
- Movement controls (arrow keys)
- Collision detection with enemies
- Visual feedback when hit (flashing)
- Reset function for respawning

### Collectible

The Star:
- Spins continuously for visual appeal
- Detects collision with Player
- Respawns at random position when collected
- Adds to score

### Enemy

The Enemy:
- Falls from the top of the screen
- Respawns at top when reaching bottom
- Causes damage on contact with Player

### UI Sprites

ScoreDisplay and LivesDisplay:
- Stay in fixed positions
- Continuously update to show current values

## Enhancements

### Add More Enemies

```snap
// Add this after the Enemy sprite
new Sprite("Enemy2") {
    position: (100, 100),
    
    implements Code {
        // Same code as Enemy, maybe different speed
        on GreenFlag {
            motion::GoToXY(operators::Random(-200, 200), 180);
            looks::Show();
            
            control::Forever {
                if gameOver {
                    control::Stop("this script");
                }
                motion::ChangeY(-5);  // Faster!
                if motion::YPosition < -180 {
                    motion::GoToXY(operators::Random(-200, 200), 180);
                }
            }
        }
    }
}
```

### Add Power-Ups

```snap
new Sprite("PowerUp") {
    implements Code {
        on GreenFlag {
            looks::Hide();
            
            control::Forever {
                // Appear occasionally
                control::Wait(units::Sec(operators::Random(10, 20)));
                
                if !gameOver {
                    motion::GoToXY(
                        operators::Random(-200, 200),
                        operators::Random(-100, 100)
                    );
                    looks::Show();
                    
                    // Disappear after 5 seconds if not collected
                    control::Wait(units::Sec(5));
                    looks::Hide();
                }
            }
        }
        
        on GreenFlag {
            control::Forever {
                if sensing::TouchingSprite("Player") {
                    change lives by 1;
                    looks::Hide();
                }
            }
        }
    }
}
```

### Add Difficulty Scaling

```snap
let enemySpeed: int = 3;

// In Stage
on GreenFlag {
    set enemySpeed = 3;
    
    // Increase difficulty over time
    control::Forever {
        control::Wait(units::Sec(10));
        change enemySpeed by 1;
    }
}

// In Enemy, use enemySpeed instead of fixed value
motion::ChangeY(-enemySpeed);
```

### Add Sound Effects

```snap
// When collecting star
if sensing::TouchingSprite("Player") {
    sound::Play("coin");
    change score by 10;
    respawn();
}

// When hit by enemy
if sensing::TouchingSprite("Enemy") {
    sound::Play("ouch");
    change lives by -1;
    // ...
}
```

## Project Structure

For a larger game, split into files:

```
my_game/
├── config.toml
└── src/
    ├── main.sp           # Imports and globals
    ├── stage.sp          # Stage definition
    ├── player.sp         # Player sprite
    ├── enemies/
    │   └── enemy.sp      # Enemy sprites
    ├── items/
    │   ├── star.sp       # Collectibles
    │   └── powerup.sp    # Power-ups
    └── ui/
        ├── score.sp      # Score display
        └── lives.sp      # Lives display
```

## Next Steps

- Add more enemy types with different behaviors
- Add levels that get progressively harder
- Add a start screen and game over screen
- Add high score tracking
- Add more sound effects and music
