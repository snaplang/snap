# Hello World

The simplest Snap program.

## Code

```snap
new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello, World!", units::Sec(2));
        }
    }
}
```

## What It Does

1. When the green flag is clicked
2. The sprite says "Hello, World!" for 2 seconds

## Step by Step

### 1. Create the Project

```bash
snap new hello_world
cd hello_world
```

### 2. Edit the Code

Open `src/main.sp` and replace with the code above.

### 3. Build and Run

```bash
snap build
snap run
```

### 4. Test in Scratch

1. Load the `.sb3` file in Scratch
2. Click the green flag
3. See "Hello, World!" appear!

## Variations

### Say Forever

```snap
new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::Say("Hello, World!");
        }
    }
}
```

### Think Instead of Say

```snap
new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::ThinkTimed("Hello, World!", units::Sec(2));
        }
    }
}
```

### Multiple Messages

```snap
new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello!", units::Sec(1));
            looks::SayTimed("Welcome to Snap!", units::Sec(2));
            looks::SayTimed("Let's make something cool!", units::Sec(2));
        }
    }
}
```

### With User Input

```snap
new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            sensing::AskAndWait("What's your name?");
            looks::SayTimed(
                operators::Join("Hello, ", sensing::Answer),
                units::Sec(2)
            );
        }
    }
}
```

## Understanding the Code

```snap
new Sprite("Sprite1") {       // Create a sprite named "Sprite1"
    implements Code {          // This sprite has code
        on GreenFlag {         // When green flag clicked
            looks::SayTimed(   // Say block with duration
                "Hello, World!",   // The message
                units::Sec(2)      // For 2 seconds
            );
        }
    }
}
```

## Next Steps

- [Moving Sprite](./moving-sprite.md) - Add movement
- [Simple Game](./simple-game.md) - Build a game
