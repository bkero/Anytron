# Anytron Project Conventions

## Overview

Anytron generates static websites for TV show quote searching and meme generation, similar to Frinkiac/Morbotron but for any show.

## Technology Decisions

- **CLI Language**: Rust (performance, single binary distribution)
- **Video Processing**: ffmpeg (subprocess calls)
- **Generated Website**: Vanilla JS with minimal dependencies
- **Search**: Client-side with lunr.js or flexsearch

## Code Conventions

### Rust

- Use `clap` for CLI argument parsing
- Use `serde` for configuration and JSON serialization
- Use `rayon` for parallel frame extraction
- Error handling with `anyhow` for CLI, `thiserror` for library code
- Format with `rustfmt`, lint with `clippy`

### JavaScript (Generated Site)

- Vanilla JS or Alpine.js for interactivity
- No build step required for generated site
- ES modules where browser support allows
- CSS custom properties for theming

## Directory Layout

```
/                           # Project root
├── src/                    # Rust source code
├── templates/              # HTML templates for generation
├── static/                 # Static assets bundled into output
├── tests/                  # Integration tests
├── openspec/               # Specifications and changes
└── examples/               # Example show directories
```

## Testing

- Unit tests in Rust with `#[test]`
- Integration tests with sample video/subtitle files
- Generated site tested with Playwright or similar

## Issue Tracking

Use `bd` (Beads) for all issue tracking. Every issue must include a detailed description.

## Spec References

- Main capability: `specs/anytron/spec.md`
