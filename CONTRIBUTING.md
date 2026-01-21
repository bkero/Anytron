# Contributing to Anytron

Thank you for your interest in contributing to Anytron! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions. We want this to be a welcoming project for everyone.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- FFmpeg installed and in your PATH
- Git

### Setting Up the Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/anytron.git
   cd anytron
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/anytron/anytron.git
   ```
4. Build the project:
   ```bash
   cargo build
   ```
5. Run tests:
   ```bash
   cargo test
   ```

## Making Changes

### Branching

Create a new branch for your changes:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### Code Style

- Run `cargo fmt` before committing to ensure consistent formatting
- Run `cargo clippy` to catch common issues
- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Write documentation comments for public APIs

### Testing

- Add tests for new functionality
- Ensure all existing tests pass: `cargo test`
- For integration tests, place them in the `tests/` directory
- Test fixtures go in `tests/fixtures/`

### Commit Messages

Write clear, descriptive commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Keep the first line under 72 characters
- Reference issues when applicable: "Fix #123: Correct subtitle parsing"

Example:
```
Add support for WebVTT subtitle format

- Parse WEBVTT headers and cue identifiers
- Handle timestamp settings (line, position, align)
- Add comprehensive tests for edge cases

Closes #42
```

## Pull Requests

1. Update your fork with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Open a pull request on GitHub

4. In your PR description:
   - Describe what the changes do
   - Link to any related issues
   - Include screenshots for UI changes
   - Note any breaking changes

5. Wait for CI to pass and address any review feedback

## Project Structure

```
anytron/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library root
│   ├── cli/              # CLI argument parsing
│   ├── config/           # Configuration parsing
│   ├── discovery/        # File scanning and episode detection
│   ├── subtitle/         # Subtitle format parsers
│   ├── extractor/        # FFmpeg integration
│   ├── indexer/          # Search index generation
│   ├── generator/        # HTML/asset generation
│   └── error.rs          # Error types
├── tests/
│   ├── integration_test.rs
│   └── fixtures/         # Test data files
└── static/               # Embedded CSS/JS assets
```

## Areas for Contribution

### Good First Issues

Look for issues labeled `good first issue` for beginner-friendly tasks.

### Feature Ideas

- Additional subtitle format support (SSA v4, PGS bitmap subtitles)
- Theme customization options
- Multiple language support
- Image optimization (WebP output)
- Progressive Web App features
- API mode for dynamic serving

### Bug Reports

When reporting bugs, please include:

- Anytron version (`anytron --version`)
- Operating system and version
- FFmpeg version (`ffmpeg -version`)
- Steps to reproduce
- Expected vs. actual behavior
- Sample files if applicable (or describe their format)

## Release Process

Releases are automated via GitHub Actions when a version tag is pushed:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Commit: `git commit -am "Release v0.x.y"`
4. Tag: `git tag v0.x.y`
5. Push: `git push origin main --tags`

## Questions?

- Open an issue for questions about contributing
- Check existing issues and discussions first

Thank you for contributing to Anytron!
