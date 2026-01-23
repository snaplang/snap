# Moving Sprite

A sprite that responds to keyboard input.

## Code

```snap
new Sprite("Player") {
    position: (0, 0),
    
    implements Code {
        on GreenFlag {
            // Reset position
            motion::GoToXY(0, 0);
            
            // Main movement loop
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
                
                // Stay on screen
                motion::IfOnEdgeBounce();
            }
        }
    }
}
```

## What It Does

1. Centers the sprite when green flag is clicked
2. Continuously checks for arrow key presses
3. Moves 5 pixels in the pressed direction
4. Bounces off the edges of the stage

## Variations

### Smooth Movement with Speed Variable

```snap
let speed: float = 5.0;

new Sprite("Player") {
    implements Code {
        on GreenFlag {
            motion::GoToXY(0, 0);
            
            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(speed);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-speed);
                }
                if sensing::KeyPressed("up arrow") {
                    motion::ChangeY(speed);
                }
                if sensing::KeyPressed("down arrow") {
                    motion::ChangeY(-speed);
                }
            }
        }
        
        // Speed controls
        on KeyPressed("w") {
            change speed by 1.0;
            looks::SayTimed(operators::Join("Speed: ", speed), units::Sec(0.5));
        }
        
        on KeyPressed("s") {
            change speed by -1.0;
            looks::SayTimed(operators::Join("Speed: ", speed), units::Sec(0.5));
        }
    }
}
```

### WASD Controls

```snap
new Sprite("Player") {
    implements Code {
        on GreenFlag {
            control::Forever {
                if sensing::KeyPressed("d") {
                    motion::ChangeX(5);
                }
                if sensing::KeyPressed("a") {
                    motion::ChangeX(-5);
                }
                if sensing::KeyPressed("w") {
                    motion::ChangeY(5);
                }
                if sensing::KeyPressed("s") {
                    motion::ChangeY(-5);
                }
            }
        }
    }
}
```

### Mouse Following

```snap
new Sprite("Follower") {
    implements Code {
        on GreenFlag {
            control::Forever {
                motion::PointTowards("_mouse_");
                motion::Move(5);
            }
        }
    }
}
```

### Smooth Mouse Following

```snap
new Sprite("Follower") {
    implements Code {
        on GreenFlag {
            control::Forever {
                // Move 10% of the distance to mouse
                motion::ChangeX((sensing::MouseX - motion::XPosition) / 10);
                motion::ChangeY((sensing::MouseY - motion::YPosition) / 10);
            }
        }
    }
}
```

### Click to Move

```snap
new Sprite("Player") {
    implements Code {
        on GreenFlag {
            control::Forever {
                if sensing::MouseDown {
                    motion::GlideToXY(
                        units::Sec(0.5),
                        sensing::MouseX,
                        sensing::MouseY
                    );
                }
            }
        }
    }
}
```

### Tank Controls (Rotate and Move)

```snap
new Sprite("Tank") {
    implements Code {
        on GreenFlag {
            motion::PointInDirection(90);
            
            control::Forever {
                if sensing::KeyPressed("up arrow") {
                    motion::Move(3);
                }
                if sensing::KeyPressed("down arrow") {
                    motion::Move(-3);
                }
                if sensing::KeyPressed("right arrow") {
                    motion::TurnRight(5);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::TurnLeft(5);
                }
                
                motion::IfOnEdgeBounce();
            }
        }
    }
}
```

### Jumping (Simple)

```snap
let grounded: bool = true;

new Sprite("Player") {
    implements Code {
        fn jump() {
            if grounded {
                set grounded = false;
                
                // Jump up
                control::Repeat(15) {
                    motion::ChangeY(8);
                }
                // Fall down
                control::Repeat(15) {
                    motion::ChangeY(-8);
                }
                
                set grounded = true;
            }
        }
        
        on GreenFlag {
            motion::GoToXY(0, -100);
            
            control::Forever {
                if sensing::KeyPressed("right arrow") {
                    motion::ChangeX(5);
                }
                if sensing::KeyPressed("left arrow") {
                    motion::ChangeX(-5);
                }
            }
        }
        
        on KeyPressed("space") {
            jump();
        }
    }
}
```

## Tips

### Diagonal Movement

The basic code allows diagonal movement (pressing two arrows at once). This makes diagonal movement faster (about 1.4x). To fix:

```snap
// Normalize diagonal speed
let dx: int = 0;
let dy: int = 0;

control::Forever {
    set dx = 0;
    set dy = 0;
    
    if sensing::KeyPressed("right arrow") {
        change dx by 5;
    }
    if sensing::KeyPressed("left arrow") {
        change dx by -5;
    }
    if sensing::KeyPressed("up arrow") {
        change dy by 5;
    }
    if sensing::KeyPressed("down arrow") {
        change dy by -5;
    }
    
    motion::ChangeX(dx);
    motion::ChangeY(dy);
}
```

### Screen Boundaries

To keep sprite fully on screen (not just center point):

```snap
control::Forever {
    // Movement code...
    
    // Clamp to screen bounds (assuming 40x40 sprite)
    if motion::XPosition > 220 {
        motion::SetX(220);
    }
    if motion::XPosition < -220 {
        motion::SetX(-220);
    }
    if motion::YPosition > 160 {
        motion::SetY(160);
    }
    if motion::YPosition < -160 {
        motion::SetY(-160);
    }
}
```

## Next Steps

- [Simple Game](./simple-game.md) - Build a complete game
