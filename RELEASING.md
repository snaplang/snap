# Releasing Snap

This document describes how to create a new release of Snap.

## Automated Releases (Recommended)

Releases are automated via GitHub Actions. There are two ways to trigger a release:

### Option 1: Git Tag (Recommended)

1. Update the version in `Cargo.toml`:

   ```toml
   [package]
   version = "0.2.0"
   ```

2. Commit the change:

   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.0"
   ```

3. Create and push a tag:

   ```bash
   git tag v0.2.0
   git push origin main
   git push origin v0.2.0
   ```

4. The GitHub Action will automatically:
   - Build binaries for all platforms (Linux, macOS, Windows)
   - Create a GitHub Release with all binaries
   - Generate release notes and checksums
   - Create a Homebrew formula artifact

### Option 2: Manual Workflow Dispatch

1. Go to the GitHub repository
2. Click on "Actions" tab
3. Select "Release" workflow
4. Click "Run workflow"
5. Enter the version (e.g., `0.2.0`)
6. Click "Run workflow"

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

### Pre-releases

For pre-releases, use suffixes:

- `0.2.0-alpha.1`
- `0.2.0-beta.1`
- `0.2.0-rc.1`

Pre-releases are automatically marked as such on GitHub.

## Release Checklist

Before releasing:

- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings (`cargo build --release`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version in Cargo.toml is updated

## Platform Support

Builds are created for:

| Platform | Architecture  | File                                    |
| -------- | ------------- | --------------------------------------- |
| Linux    | x86_64        | `snap-x86_64-unknown-linux-gnu.tar.gz`  |
| Linux    | ARM64         | `snap-aarch64-unknown-linux-gnu.tar.gz` |
| macOS    | Intel         | `snap-x86_64-apple-darwin.tar.gz`       |
| macOS    | Apple Silicon | `snap-aarch64-apple-darwin.tar.gz`      |
| Windows  | x86_64        | `snap-x86_64-pc-windows-msvc.zip`       |
| Windows  | ARM64         | `snap-aarch64-pc-windows-msvc.zip`      |

## Troubleshooting

### Build Failures

If a build fails:

1. Check the GitHub Actions logs
2. Ensure the code compiles locally for that target:
   ```bash
   rustup target add x86_64-unknown-linux-gnu
   cargo build --release --target x86_64-unknown-linux-gnu
   ```

### Re-running a Release

If a release partially failed:

1. Delete the failed release from GitHub Releases
2. Delete the tag locally and remotely:
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```
3. Re-create the tag and push

## Local Testing

To test the release build locally:

```bash
# Build release binary
cargo build --release

# Test it works
./target/release/snap --version
./target/release/snap new test_release
./target/release/snap build test_release
```
