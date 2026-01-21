//! Episode ID parsing from filenames
//!
//! Supports various naming conventions:
//! - `S01E01`, `s01e01`
//! - `1x01`
//! - `Season 1 Episode 01`
//! - `[01x01]`

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{AnytronError, Result};

lazy_static! {
    /// Patterns to match episode identifiers in filenames
    static ref EPISODE_PATTERNS: Vec<Regex> = vec![
        // S01E01, s01e01
        Regex::new(r"(?i)[Ss](\d{1,2})[Ee](\d{1,3})").unwrap(),
        // 1x01, 01x01
        Regex::new(r"(\d{1,2})x(\d{1,3})").unwrap(),
        // Season 1 Episode 01
        Regex::new(r"(?i)Season\s*(\d{1,2})\s*Episode\s*(\d{1,3})").unwrap(),
        // [01x01] in brackets
        Regex::new(r"\[(\d{1,2})x(\d{1,3})\]").unwrap(),
    ];
}

/// Episode identifier (season + episode number)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EpisodeId {
    pub season: u32,
    pub episode: u32,
}

impl EpisodeId {
    /// Create a new episode ID
    pub fn new(season: u32, episode: u32) -> Self {
        Self { season, episode }
    }

    /// Parse episode ID from a filename or path
    pub fn from_filename(filename: &str) -> Result<Self> {
        for pattern in EPISODE_PATTERNS.iter() {
            if let Some(captures) = pattern.captures(filename) {
                let season: u32 = captures
                    .get(1)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(0);
                let episode: u32 = captures
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(0);

                if season > 0 && episode > 0 {
                    return Ok(Self { season, episode });
                }
            }
        }

        Err(AnytronError::InvalidEpisodeFormat(filename.to_string()))
    }

    /// Format as SXXEXX string
    pub fn to_string_padded(&self) -> String {
        format!("S{:02}E{:02}", self.season, self.episode)
    }
}

impl fmt::Display for EpisodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S{:02}E{:02}", self.season, self.episode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sxxexx() {
        let id = EpisodeId::from_filename("Show.S01E05.720p.mkv").unwrap();
        assert_eq!(id.season, 1);
        assert_eq!(id.episode, 5);

        let id2 = EpisodeId::from_filename("s02e15.avi").unwrap();
        assert_eq!(id2.season, 2);
        assert_eq!(id2.episode, 15);
    }

    #[test]
    fn test_parse_nxnn() {
        let id = EpisodeId::from_filename("Show.1x05.avi").unwrap();
        assert_eq!(id.season, 1);
        assert_eq!(id.episode, 5);

        let id2 = EpisodeId::from_filename("show.02x15.mkv").unwrap();
        assert_eq!(id2.season, 2);
        assert_eq!(id2.episode, 15);
    }

    #[test]
    fn test_parse_season_episode() {
        let id = EpisodeId::from_filename("Season 1 Episode 05.mp4").unwrap();
        assert_eq!(id.season, 1);
        assert_eq!(id.episode, 5);
    }

    #[test]
    fn test_parse_brackets() {
        let id = EpisodeId::from_filename("[01x05] Show Title.mkv").unwrap();
        assert_eq!(id.season, 1);
        assert_eq!(id.episode, 5);
    }

    #[test]
    fn test_invalid_format() {
        assert!(EpisodeId::from_filename("movie.mkv").is_err());
    }

    #[test]
    fn test_display() {
        let id = EpisodeId::new(1, 5);
        assert_eq!(id.to_string(), "S01E05");
        assert_eq!(id.to_string_padded(), "S01E05");

        let id2 = EpisodeId::new(12, 99);
        assert_eq!(id2.to_string(), "S12E99");
    }
}
