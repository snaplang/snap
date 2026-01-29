# Blocks

Snap provides access to all Scratch blocks through namespaced function calls. Blocks are organized into categories that match Scratch's block palette.

## Block Syntax

### Statement Blocks

Blocks that perform actions (stack blocks in Scratch):

```snap
category::BlockName(arg1, arg2);
```

Examples:

```snap
motion::Move(10);
looks::Say("Hello!");
control::Wait(units::Sec(1));
```

### Reporter Blocks

Blocks that return values (reporter blocks in Scratch):

```snap
let value = category::BlockName(args);
```

Examples:

```snap
let x = motion::XPosition;
let touching = sensing::TouchingSprite("Enemy");
let random = operators::Random(1, 10);
```

### Boolean Blocks

Blocks that return true/false (boolean reporters in Scratch):

```snap
if sensing::KeyPressed("space") {
    // ...
}

if sensing::TouchingEdge && motion::XPosition > 0 {
    // ...
}
```

### C-Blocks

Blocks that contain other blocks (C-shaped blocks in Scratch):

```snap
control::Forever {
    // Blocks inside
}

control::Repeat(10) {
    // Blocks inside
}
```

## Block Categories

| Category    | Description           | Example                                |
| ----------- | --------------------- | -------------------------------------- |
| `motion`    | Movement and position | `motion::Move(10)`                     |
| `looks`     | Appearance and speech | `looks::Say("Hi")`                     |
| `sound`     | Sound playback        | `sound::Play("meow")`                  |
| `events`    | Broadcasts            | `events::Broadcast("msg")`             |
| `control`   | Flow control          | `control::Wait(units::Sec(1))`         |
| `sensing`   | Input and detection   | `sensing::KeyPressed("space")`         |
| `operators` | Math and logic        | `operators::Random(1, 10)`             |
| `pen`       | Drawing on stage      | `pen::PenDown()` (requires `use pen;`) |

## Quick Reference

### Motion

```snap
motion::Move(steps);
motion::TurnRight(degrees);
motion::TurnLeft(degrees);
motion::GoToXY(x, y);
motion::GlideToXY(units::Sec(time), x, y);
motion::PointInDirection(degrees);
motion::ChangeX(amount);
motion::ChangeY(amount);
motion::SetX(x);
motion::SetY(y);
motion::IfOnEdgeBounce();

// Reporters
motion::XPosition
motion::YPosition
motion::Direction
```

### Looks

```snap
looks::Say(message);
looks::SayTimed(message, units::Sec(time));
looks::Think(message);
looks::ThinkTimed(message, units::Sec(time));
looks::Show();
looks::Hide();
looks::SwitchCostume(name);
looks::NextCostume();
looks::SwitchBackdrop(name);  // Stage only
looks::NextBackdrop();        // Stage only
looks::ChangeSize(amount);
looks::SetSize(percent);

// Reporters
looks::CostumeNumber
looks::Size
```

### Sound

```snap
sound::Play(name);
sound::PlayUntilDone(name);
sound::StopAllSounds();
sound::ChangeVolume(amount);
sound::SetVolume(percent);

// Reporters
sound::Volume
```

### Control

```snap
control::Wait(units::Sec(time));
control::Stop(mode);  // "all", "this script", "other scripts in sprite"
control::CreateClone(target);  // "_myself_" or sprite name
control::DeleteClone();

// C-blocks
control::Forever { ... }
control::Repeat(times) { ... }
control::RepeatUntil(condition) { ... }
```

### Sensing

```snap
sensing::AskAndWait(question);
sensing::ResetTimer();

// Reporters
sensing::TouchingSprite(name)  // Boolean
sensing::TouchingEdge          // Boolean
sensing::KeyPressed(key)       // Boolean
sensing::MouseDown             // Boolean
sensing::MouseX
sensing::MouseY
sensing::Timer
sensing::Answer
```

### Events

```snap
events::Broadcast(message);
events::BroadcastAndWait(message);
```

### Operators

```snap
// Math (usually use native operators instead)
operators::Random(from, to)
operators::Round(number)
operators::MathOp(operation, number)  // "abs", "floor", "ceiling", "sqrt", etc.

// Strings
operators::Join(string1, string2)
operators::LetterOf(index, string)
operators::Length(string)
```

## Detailed Documentation

For complete documentation on each category:

- [Motion Blocks](./blocks/motion.md)
- [Looks Blocks](./blocks/looks.md)
- [Sound Blocks](./blocks/sound.md)
- [Control Blocks](./blocks/control.md)
- [Sensing Blocks](./blocks/sensing.md)
- [Operators](./blocks/operators.md)
- [Variables](./blocks/variables.md)

## Using Native Operators

For basic math and comparisons, prefer native operators over block calls:

```snap
// Preferred
let sum = a + b;
let product = x * y;
let is_greater = score > 100;

// Instead of
let sum = operators::Add(a, b);
let product = operators::Mul(x, y);
let is_greater = operators::Gt(score, 100);
```

Native operators compile to the same Scratch blocks but are more readable.
