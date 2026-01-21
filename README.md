# Anytron

[![CI](https://github.com/anytron/anytron/actions/workflows/ci.yml/badge.svg)](https://github.com/anytron/anytron/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/anytron.svg)](https://crates.io/crates/anytron)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Generate static websites for TV show quote search and meme generation. Similar to [Frinkiac](https://frinkiac.com/) and [Morbotron](https://morbotron.com/), but self-hosted and for any show.

## Features

- **Full-text search** - Search through all subtitles with instant results using lunr.js
- **Frame extraction** - Automatically extract frames at subtitle timestamps
- **Meme generator** - Create memes with custom text overlays
- **Multiple subtitle formats** - Supports SRT, ASS/SSA, and WebVTT
- **Embedded subtitles** - Extract subtitles from MKV/MP4 containers
- **Static output** - No server required, host anywhere (GitHub Pages, S3, etc.)
- **Fast & parallel** - Uses Rayon for parallel frame extraction

## Installation

### From crates.io

```bash
cargo install anytron
```

### From source

```bash
git clone https://github.com/anytron/anytron.git
cd anytron
cargo install --path .
```

### Prerequisites

- [FFmpeg](https://ffmpeg.org/) must be installed and available in your PATH
- Rust 1.70+ (for building from source)

## Quick Start

1. Organize your media files:

```
my_show/
├── anytron.toml          # Configuration file
├── Show.S01E01.mkv       # Video files with embedded subtitles
├── Show.S01E02.mkv       # ... or external subtitle files
├── Show.S01E02.srt
└── ...
```

2. Create a configuration file (`anytron.toml`):

```toml
[show]
name = "My Favorite Show"
description = "Quotes from My Favorite Show"

[site]
title = "My Show Quote Search"
base_url = "/"
enable_memes = true
```

3. Generate the site:

```bash
anytron generate ./my_show -o ./output
```

4. Preview locally:

```bash
anytron serve ./output
# Open http://localhost:8080 in your browser
```

5. Deploy to any static hosting (GitHub Pages, Netlify, S3, etc.)

## Usage

### Commands

```bash
# Generate a static site
anytron generate <INPUT_DIR> [OPTIONS]

# Validate input directory structure
anytron validate <INPUT_DIR>

# Serve generated site locally
anytron serve <DIRECTORY> [--port PORT]
```

### Generate Options

| Option | Description |
|--------|-------------|
| `-o, --output <DIR>` | Output directory (default: `output`) |
| `-j, --jobs <N>` | Number of parallel workers |
| `--quality <N>` | JPEG quality 1-100 (default: 85) |
| `--thumb-width <N>` | Thumbnail width in pixels (default: 320) |
| `--skip-frames` | Skip frame extraction (use existing) |
| `--clean` | Clean output directory before generating |
| `--seasons <LIST>` | Only process specific seasons (e.g., `1,2,3`) |
| `--episodes <LIST>` | Only process specific episodes (e.g., `S01E01,S02E05`) |
| `-v, --verbose` | Increase verbosity (-v, -vv, -vvv) |

### Configuration

Create an `anytron.toml` file in your input directory:

```toml
[show]
name = "Show Name"
description = "A description of the show"

[site]
title = "Quote Search"      # Page title
base_url = "/"              # Base URL for links (use "/" for root)
theme_color = "#1a1a2e"     # Theme color for mobile browsers
enable_memes = true         # Enable meme generator on caption pages
```

## File Naming Convention

Anytron uses filename patterns to identify episodes:

- `Show.S01E01.mkv` - Season 1, Episode 1
- `Show.S01E01.Episode.Title.mp4` - With episode title
- `show.s1e5.avi` - Lowercase, single-digit season
- `Show.1x05.mkv` - Alternative format
- `Show [1x05].mkv` - Bracketed format

Subtitle files should match video files:
- `Show.S01E01.srt` for `Show.S01E01.mkv`
- `Show.S01E01.en.srt` - Language-tagged subtitle

## Output Structure

```
output/
├── index.html              # Search page
├── caption/
│   └── S01E01-12345.html   # Caption detail pages
├── img/
│   ├── frames/
│   │   └── S01E01/
│   │       └── 12345.jpg   # Full-size frames
│   └── thumbs/
│       └── S01E01/
│           └── 12345.jpg   # Thumbnails
├── search/
│   └── index.json          # Search index
├── css/
│   └── style.css
└── js/
    └── bundle.js           # Search + meme generator
```

## Subtitle Priority

When multiple subtitle tracks are available, Anytron selects the best one:

1. **English** subtitles preferred over other languages
2. **Default** track preferred if marked
3. **Regular** subtitles preferred over SDH/CC (hearing impaired)
4. **Full** subtitles preferred over forced-only tracks
5. **Text-based** formats (SRT, ASS) preferred over bitmap

## Library Usage

Anytron can also be used as a library:

```rust
use anytron::config::Config;
use anytron::discovery::Scanner;
use anytron::subtitle::parse_file;
use std::path::Path;

// Scan for episodes
let scanner = Scanner::new(Path::new("./my_show"));
let episodes = scanner.scan()?;

// Parse subtitles
for episode in &episodes {
    if let Some(sub_path) = &episode.subtitle_path {
        let entries = parse_file(sub_path)?;
        println!("Found {} subtitle entries", entries.len());
    }
}
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Frinkiac](https://frinkiac.com/) and [Morbotron](https://morbotron.com/)
- Uses [lunr.js](https://lunrjs.com/) for client-side search
- Built with [Rust](https://www.rust-lang.org/) and love
