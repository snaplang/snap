# Sound Blocks

Sound blocks control audio playback in your project.

> **Note:** Custom sound files are not yet supported. Currently, sounds must be added manually in the Scratch editor after importing the `.sb3` file.

## Playing Sounds

### Play

Start playing a sound without waiting.

```snap
sound::Play("meow");
```

The script continues immediately while the sound plays in the background.

### Play Until Done

Play a sound and wait for it to finish.

```snap
sound::PlayUntilDone("meow");
looks::Say("Done!");  // Only runs after sound finishes
```

### Stop All Sounds

Stop all sounds that are currently playing.

```snap
sound::StopAllSounds();
```

## Volume

### Set Volume

Set the sprite's volume (0-100).

```snap
sound::SetVolume(100);    // Full volume
sound::SetVolume(50);     // Half volume
sound::SetVolume(0);      // Muted
```

### Change Volume

Change volume by an amount.

```snap
sound::ChangeVolume(10);     // Louder
sound::ChangeVolume(-10);    // Quieter
```

### Volume Reporter

Get the current volume.

```snap
let vol = sound::Volume;
```

## Examples

### Sound on Event

```snap
on Clicked {
    sound::Play("pop");
}
```

### Background Music

```snap
on GreenFlag {
    sound::SetVolume(50);
    control::Forever {
        sound::PlayUntilDone("music");
    }
}
```

### Sound Effects

```snap
// Jump sound
on KeyPressed("space") {
    sound::Play("jump");
}

// Collision sound
if sensing::TouchingSprite("Enemy") {
    sound::Play("ouch");
}
```

### Fade Out Music

```snap
fn fadeOutMusic() {
    control::Repeat(10) {
        sound::ChangeVolume(-10);
        control::Wait(units::Sec(0.2));
    }
    sound::StopAllSounds();
    sound::SetVolume(100);  // Reset for next time
}
```

### Sound with Animation

```snap
on Broadcast("explosion") {
    sound::Play("boom");
    looks::SetEffect("brightness", 100);
    control::Wait(units::Sec(0.1));
    looks::ClearEffects();
}
```

## Notes

- Each sprite has its own volume setting
- The Stage can also play sounds
- Sounds continue playing even if the sprite is hidden
- Use `PlayUntilDone` for sound sequences that need to be in order
- Use `Play` for sound effects that shouldn't block execution
