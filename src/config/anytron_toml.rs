//! Configuration file parsing for anytron.toml

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::{AnytronError, Result};

/// Main configuration structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Show metadata
    pub show: ShowConfig,

    /// Frame extraction settings
    pub frames: FrameConfig,

    /// Site generation settings
    pub site: SiteConfig,

    /// Search settings
    pub search: SearchConfig,
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| AnytronError::ConfigRead {
            path: path.to_path_buf(),
            source: e,
        })?;

        toml::from_str(&content).map_err(|e| AnytronError::ConfigParse {
            path: path.to_path_buf(),
            message: e.to_string(),
        })
    }

    /// Save configuration to a TOML file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let content =
            toml::to_string_pretty(self).map_err(|e| AnytronError::Config(e.to_string()))?;

        std::fs::write(path, content).map_err(|e| AnytronError::FileWrite {
            path: path.to_path_buf(),
            source: e,
        })
    }
}

/// Show metadata configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShowConfig {
    /// Display name of the show
    pub name: String,

    /// Short description
    pub description: String,

    /// URL slug (used in page generation)
    pub slug: String,

    /// Number of seasons (for validation)
    pub seasons: Option<u32>,
}

impl Default for ShowConfig {
    fn default() -> Self {
        Self {
            name: "My Show".to_string(),
            description: "TV show quote search and meme generator".to_string(),
            slug: "myshow".to_string(),
            seasons: None,
        }
    }
}

/// Frame extraction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FrameConfig {
    /// Frame extraction interval in milliseconds
    pub interval_ms: u64,

    /// JPEG quality (1-100)
    pub quality: u8,

    /// Full-size frame width (0 = original)
    pub frame_width: u32,

    /// Thumbnail width in pixels
    pub thumb_width: u32,

    /// Thumbnail quality (1-100)
    pub thumb_quality: u8,
}

impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            interval_ms: 1000,
            quality: 85,
            frame_width: 0,
            thumb_width: 320,
            thumb_quality: 70,
        }
    }
}

/// Site generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SiteConfig {
    /// Site title
    pub title: String,

    /// Base URL for the generated site
    pub base_url: String,

    /// Theme color for meta tags
    pub theme_color: String,

    /// Google Analytics ID (optional)
    pub analytics_id: Option<String>,

    /// Custom CSS (appended to style.css)
    pub custom_css: Option<String>,

    /// Custom JavaScript (appended to bundle.js)
    pub custom_js: Option<String>,

    /// Enable meme generator
    pub enable_memes: bool,

    /// Maximum frames to show in search results
    pub max_results: usize,

    /// Results per page in caption listing
    pub results_per_page: usize,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Quote Search".to_string(),
            base_url: "/".to_string(),
            theme_color: "#1a1a2e".to_string(),
            analytics_id: None,
            custom_css: None,
            custom_js: None,
            enable_memes: true,
            max_results: 100,
            results_per_page: 50,
        }
    }
}

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchConfig {
    /// Minimum query length
    pub min_query_length: usize,

    /// Enable fuzzy matching
    pub fuzzy: bool,

    /// Boost factor for exact matches
    pub exact_boost: f32,

    /// Fields to search (text, episode)
    pub fields: Vec<String>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            min_query_length: 2,
            fuzzy: true,
            exact_boost: 2.0,
            fields: vec!["text".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.show.name, "My Show");
        assert_eq!(config.frames.quality, 85);
        assert!(config.site.enable_memes);
    }

    #[test]
    fn test_parse_toml() {
        let toml_str = r#"
[show]
name = "The Simpsons"
description = "Frinkiac clone"
slug = "simpsons"

[frames]
interval_ms = 500
quality = 90

[site]
title = "Simpsons Search"
enable_memes = true
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.show.name, "The Simpsons");
        assert_eq!(config.frames.interval_ms, 500);
        assert_eq!(config.site.title, "Simpsons Search");
    }
}
