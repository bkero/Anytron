# Anytron - TV Show Quote Search & Meme Generator

## Purpose

Anytron is a command-line tool that generates static websites for searching TV show quotes and creating memes, similar to Frinkiac (The Simpsons) and Morbotron (Futurama). A user provides a directory containing video files and subtitle files for a TV show, and Anytron processes these to generate a fully functional, self-contained website.

**Goals:**
- Generate a complete, static website from any TV show's video and subtitle files
- Enable full-text search across all dialogue/quotes
- Display screenshot grids for search results
- Provide meme generation with text overlay on screenshots
- Work entirely offline after generation (no external API dependencies at runtime)

**Data Flow:**
1. User organizes TV show files in a structured directory (videos + subtitles)
2. User runs `anytron generate /path/to/show`
3. Anytron extracts frames at subtitle timestamps from video files
4. Anytron parses subtitle files (SRT, ASS, VTT) to build a searchable index
5. Anytron generates static HTML/CSS/JS and a JSON search index
6. User can serve the generated `output/` directory with any static file server

## Requirements

### Requirement: CLI Interface

The system SHALL provide a command-line interface that accepts a TV show directory as input and generates a complete website.

#### Scenario: Basic generation

- **WHEN** user runs `anytron generate /path/to/show`
- **THEN** the system processes all video and subtitle files
- **AND** generates a complete website in `./output/` directory

#### Scenario: Custom output directory

- **WHEN** user runs `anytron generate /path/to/show --output /custom/path`
- **THEN** the system generates the website in the specified output directory

#### Scenario: Show metadata configuration

- **WHEN** user provides an `anytron.toml` configuration file in the show directory
- **THEN** the system reads show title, season/episode naming patterns, and other metadata from this file

### Requirement: Input Directory Structure

The system SHALL support a standardized directory structure for TV show content.

#### Scenario: Standard directory layout

- **WHEN** input directory contains `Season XX/` subdirectories
- **AND** each season directory contains video files (`.mkv`, `.mp4`, `.avi`, `.webm`)
- **AND** subtitle files (`.srt`, `.ass`, `.vtt`) with matching names
- **THEN** the system correctly associates subtitles with their video files

#### Scenario: Flat directory structure

- **WHEN** input directory contains video and subtitle files at the root level
- **AND** files follow naming pattern `ShowName.SXXEXX.extension`
- **THEN** the system correctly identifies season and episode numbers

### Requirement: Frame Extraction

The system SHALL extract screenshot frames from video files at subtitle timestamps.

#### Scenario: Frame extraction at dialogue timestamps

- **WHEN** processing a video file with associated subtitles
- **THEN** the system extracts a frame image at the start time of each subtitle entry
- **AND** stores frames as JPEG or WebP images with optimized file sizes

#### Scenario: Thumbnail generation

- **WHEN** extracting frames
- **THEN** the system also generates thumbnail versions for grid display
- **AND** thumbnails are appropriately sized for web performance (e.g., 320px width)

### Requirement: Subtitle Parsing

The system SHALL parse multiple subtitle formats to extract dialogue text and timing.

#### Scenario: SRT format support

- **WHEN** subtitle file is in SRT format
- **THEN** the system extracts text, start time, and end time for each entry

#### Scenario: ASS/SSA format support

- **WHEN** subtitle file is in ASS or SSA format
- **THEN** the system extracts dialogue text and timing
- **AND** strips formatting tags from the text

#### Scenario: VTT format support

- **WHEN** subtitle file is in WebVTT format
- **THEN** the system extracts text and timing information

### Requirement: Search Index Generation

The system SHALL generate a searchable index of all dialogue.

#### Scenario: Full-text search index

- **WHEN** processing subtitles
- **THEN** the system creates a JSON search index containing:
  - Subtitle text (normalized/searchable)
  - Episode identifier (SXXEXX format)
  - Timestamp (milliseconds)
  - Frame image filename

#### Scenario: Search index optimization

- **WHEN** generating the search index
- **THEN** the index is optimized for client-side search (e.g., using lunr.js or similar)
- **AND** the index is split into chunks if the show is large to improve load time

### Requirement: Generated Website - Search Interface

The generated website SHALL provide a search interface for finding quotes.

#### Scenario: Quote search

- **WHEN** user enters text in the search box
- **THEN** the system displays matching quotes with their screenshots in a grid
- **AND** results are ordered by relevance

#### Scenario: Empty search

- **WHEN** search box is empty or cleared
- **THEN** the system displays a random selection of screenshots
- **OR** displays a welcome message with instructions

### Requirement: Generated Website - Results Grid

The generated website SHALL display search results as a grid of screenshots.

#### Scenario: Grid display

- **WHEN** search returns results
- **THEN** results appear as a grid of thumbnail images
- **AND** each thumbnail shows the episode identifier (SXXEXX)
- **AND** hovering displays the quote text as a tooltip

#### Scenario: Pagination

- **WHEN** search returns more than 48 results
- **THEN** results are paginated or infinite-scroll loaded
- **AND** user can navigate through all results

### Requirement: Generated Website - Caption Page

The generated website SHALL provide a detail page for each frame.

#### Scenario: Caption page content

- **WHEN** user clicks on a search result
- **THEN** a caption page opens showing:
  - Full-size screenshot
  - The subtitle text for that frame
  - Surrounding context (previous and next 2-3 subtitles)
  - Episode identifier and timestamp

#### Scenario: Navigation between frames

- **WHEN** viewing a caption page
- **THEN** user can navigate to previous/next frame in the episode
- **AND** navigation updates the URL for shareability

### Requirement: Generated Website - Meme Generator

The generated website SHALL allow users to create memes from screenshots.

#### Scenario: Generate meme button

- **WHEN** user clicks "Generate Meme" on a caption page
- **THEN** a meme is created with the subtitle text overlaid on the screenshot
- **AND** text appears at the bottom of the image in a readable font (white text with black outline)

#### Scenario: Custom meme text

- **WHEN** user is on the meme generation view
- **THEN** user can edit the text before generating
- **AND** text can span multiple lines
- **AND** changes preview in real-time

#### Scenario: Meme download

- **WHEN** user generates a meme
- **THEN** user can download the meme as an image file
- **AND** meme generation happens client-side (no server required)

### Requirement: Generated Website - Static Deployment

The generated website SHALL be fully static and self-contained.

#### Scenario: No runtime dependencies

- **WHEN** website is generated
- **THEN** all assets (HTML, CSS, JS, images, search index) are included
- **AND** no external API calls are required at runtime
- **AND** website can be served by any static file server

#### Scenario: Local file serving

- **WHEN** user opens `index.html` directly in a browser (file:// protocol)
- **THEN** basic functionality works (with possible limitations on search due to CORS)
- **OR** the system provides clear instructions to use a local server

## Technology Stack

### CLI Tool

- **Language**: Rust (for performance and easy distribution)
- **Video Processing**: ffmpeg (via subprocess or rust bindings)
- **Subtitle Parsing**: Custom parsers or existing crates for SRT/ASS/VTT
- **Search Index**: Generate JSON compatible with lunr.js or flexsearch

### Generated Website

- **Framework**: Vanilla JS or minimal framework (preact, alpine.js)
- **Search Library**: lunr.js or flexsearch (bundled)
- **Meme Generation**: Canvas API for client-side image composition
- **Styling**: Minimal CSS, responsive design

## Project Structure

```
anytron/
├── src/
│   ├── main.rs                    # CLI entry point
│   ├── config.rs                  # Configuration parsing
│   ├── extractor/
│   │   ├── mod.rs
│   │   ├── video.rs               # Frame extraction via ffmpeg
│   │   └── subtitles.rs           # SRT/ASS/VTT parsing
│   ├── indexer/
│   │   ├── mod.rs
│   │   └── search.rs              # Search index generation
│   └── generator/
│       ├── mod.rs
│       ├── html.rs                # HTML page generation
│       ├── assets.rs              # Static asset copying
│       └── thumbnails.rs          # Image optimization
├── templates/
│   ├── index.html                 # Search page template
│   ├── caption.html               # Caption page template
│   └── partials/
│       ├── header.html
│       └── footer.html
├── static/
│   ├── css/
│   │   └── style.css
│   └── js/
│       ├── search.js              # Search functionality
│       ├── meme.js                # Meme generator
│       └── vendor/
│           └── lunr.min.js
├── Cargo.toml
└── README.md
```

## Output Structure

```
output/
├── index.html                     # Search page
├── caption/
│   └── S01E01/
│       └── 12345.html             # Caption pages by episode/timestamp
├── img/
│   ├── frames/
│   │   └── S01E01/
│   │       └── 12345.jpg          # Full-size frames
│   └── thumbs/
│       └── S01E01/
│           └── 12345.jpg          # Thumbnails
├── search/
│   └── index.json                 # Search index
├── css/
│   └── style.css
└── js/
    └── bundle.js                  # Bundled JavaScript
```

## Configuration

### anytron.toml (in show directory)

```toml
[show]
title = "My TV Show"
seasons = 5

[naming]
# Regex pattern to extract season/episode from filenames
pattern = "S(\\d+)E(\\d+)"

[output]
frame_quality = 85           # JPEG quality (1-100)
thumb_width = 320            # Thumbnail width in pixels
max_text_length = 140        # Max characters for meme text

[search]
min_query_length = 2         # Minimum search query length
results_per_page = 48        # Results per page/load
```

## CLI Commands

```bash
# Basic usage
anytron generate /path/to/show

# With options
anytron generate /path/to/show \
  --output /var/www/myshow \
  --title "My Show" \
  --quality 90

# Development server (optional)
anytron serve /path/to/output --port 8080

# Validate input directory
anytron validate /path/to/show
```

## Verification

1. **Frame extraction**: Run on sample video, verify frames extracted at correct timestamps
2. **Subtitle parsing**: Test with SRT, ASS, and VTT files, verify text and timing accuracy
3. **Search functionality**: Search for known quotes, verify results match
4. **Caption page**: Click result, verify correct screenshot and surrounding context displayed
5. **Meme generation**: Generate meme, verify text overlay and download functionality
6. **Static deployment**: Serve with nginx/caddy/python http.server, verify all features work

## Known Limitations

- **Large shows**: Shows with many episodes may have large search indexes; consider chunking
- **Video formats**: Depends on ffmpeg; some obscure formats may not be supported
- **Subtitle accuracy**: Quality depends on subtitle file accuracy
- **Client-side search**: Very large indexes may be slow on low-powered devices
- **No GIF support**: Initial version focuses on static memes; animated GIFs are future enhancement
