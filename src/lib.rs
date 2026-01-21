//! Anytron - Generate static websites for TV show quote search and meme generation
//!
//! This library provides functionality to:
//! - Scan directories for video and subtitle files
//! - Parse SRT, ASS/SSA, and WebVTT subtitle formats
//! - Extract frames from videos using FFmpeg
//! - Generate search indices compatible with lunr.js
//! - Create static HTML pages for search and caption viewing
//!
//! # Example
//!
//! ```no_run
//! use anytron::config::Config;
//! use anytron::discovery::Scanner;
//! use anytron::extractor::FrameExtractor;
//! use anytron::generator::SiteGenerator;
//! use anytron::indexer::SearchIndexer;
//! use std::path::Path;
//!
//! // Scan for episodes
//! let scanner = Scanner::new(Path::new("./my_show"));
//! let episodes = scanner.scan().expect("Failed to scan");
//!
//! // Parse subtitles
//! let mut all_entries = Vec::new();
//! for episode in &episodes {
//!     let entries = episode.parse_subtitles().expect("Failed to parse");
//!     all_entries.push((episode.clone(), entries));
//! }
//!
//! // Build search index
//! let indexer = SearchIndexer::new();
//! let index = indexer.build_index(&all_entries).expect("Failed to build index");
//!
//! // Generate site
//! let config = Config::default();
//! let generator = SiteGenerator::new(&config, Path::new("./output"));
//! generator.generate(&all_entries, &index).expect("Failed to generate");
//! ```

pub mod cli;
pub mod config;
pub mod discovery;
pub mod error;
pub mod extractor;
pub mod generator;
pub mod indexer;
pub mod subtitle;

pub use error::{AnytronError, Result};
