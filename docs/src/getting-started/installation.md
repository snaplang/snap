# Installation

## Prerequisites

Snap is written in Rust, so you'll need to have Rust installed on your system.

### Installing Rust

If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, make sure `cargo` is in your PATH:

```bash
cargo --version
```

## Installing Snap

### From Source (Recommended)

Clone the repository and build from source:

```bash
# Clone the repository
git clone https://github.com/snaplang/snap.git
cd snap

# Build in release mode
cargo build --release

# The binary will be at ./target/release/snap
```

### Add to PATH

To use `snap` from anywhere, add it to your PATH:

#### macOS/Linux

```bash
# Add to your .bashrc, .zshrc, or equivalent
export PATH="$PATH:/path/to/snap/target/release"
```

Or create a symlink:

```bash
sudo ln -s /path/to/snap/target/release/snap /usr/local/bin/snap
```

#### Windows

Add the `target\release` directory to your system's PATH environment variable.

## Verify Installation

Verify that Snap is installed correctly:

```bash
snap --version
```

You should see output like:

```
snap 0.1.0
```

## Editor Support

While there's no official editor extension yet, Snap's syntax is similar to Rust, so you can use Rust syntax highlighting for a reasonable experience:

- **VS Code**: Use the Rust extension and set `.sp` files to use Rust highlighting
- **Vim/Neovim**: Add `autocmd BufRead,BufNewFile *.sp set filetype=rust`
- **Other editors**: Configure `.sp` files to use Rust or JavaScript highlighting

## Next Steps

Now that you have Snap installed, let's [create your first project](./quick-start.md)!
