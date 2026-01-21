//! Error types for Anytron
//!
//! Uses `thiserror` for library errors and `anyhow` for CLI-level error handling.

use std::path::PathBuf;
use thiserror::Error;

/// Library-level errors with specific context
#[derive(Error, Debug)]
pub enum AnytronError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Failed to read configuration file '{path}': {source}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse configuration file '{path}': {message}")]
    ConfigParse { path: PathBuf, message: String },

    #[error("Discovery error: {0}")]
    Discovery(String),

    #[error("No video files found in '{0}'")]
    NoVideosFound(PathBuf),

    #[error("No subtitle files found for '{video}'")]
    NoSubtitlesFound { video: PathBuf },

    #[error("Subtitle parse error in '{path}' at line {line}: {message}")]
    SubtitleParse {
        path: PathBuf,
        line: usize,
        message: String,
    },

    #[error("Unsupported subtitle format: {0}")]
    UnsupportedSubtitleFormat(String),

    #[error("FFmpeg error: {0}")]
    Ffmpeg(String),

    #[error("FFmpeg not found. Please install FFmpeg and ensure it's in your PATH")]
    FfmpegNotFound,

    #[error("Failed to extract frame at {timestamp}ms from '{video}': {message}")]
    FrameExtraction {
        video: PathBuf,
        timestamp: u64,
        message: String,
    },

    #[error("Output error: {0}")]
    Output(String),

    #[error("Failed to create output directory '{path}': {source}")]
    OutputDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Template error: {0}")]
    Template(String),

    #[error("Invalid episode format in filename '{0}'. Expected SXXEXX pattern")]
    InvalidEpisodeFormat(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias using AnytronError
pub type Result<T> = std::result::Result<T, AnytronError>;
