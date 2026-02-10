//! Subtitle data types

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{AnytronError, Result};

lazy_static! {
    static ref RE_HTML: Regex = Regex::new(r"<[^>]+>").unwrap();
    static ref RE_ASS: Regex = Regex::new(r"\{[^}]+\}").unwrap();
}

/// Timestamp in milliseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Create a new timestamp from milliseconds
    pub fn from_millis(ms: u64) -> Self {
        Self(ms)
    }

    /// Create a timestamp from hours, minutes, seconds, and milliseconds
    pub fn from_hms_ms(hours: u64, minutes: u64, seconds: u64, millis: u64) -> Self {
        let total_ms = hours * 3600 * 1000 + minutes * 60 * 1000 + seconds * 1000 + millis;
        Self(total_ms)
    }

    /// Get the timestamp in milliseconds
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    /// Get the timestamp in seconds (floating point)
    pub fn as_secs_f64(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    /// Parse SRT timestamp format: HH:MM:SS,mmm
    pub fn parse_srt(s: &str) -> Result<Self> {
        let s = s.trim();
        let parts: Vec<&str> = s.split([':', ',']).collect();
        if parts.len() != 4 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid SRT timestamp: {}",
                s
            )));
        }

        let hours: u64 = parts[0]
            .parse()
            .map_err(|_| AnytronError::InvalidTimestamp(format!("Invalid hours: {}", parts[0])))?;
        let minutes: u64 = parts[1].parse().map_err(|_| {
            AnytronError::InvalidTimestamp(format!("Invalid minutes: {}", parts[1]))
        })?;
        let seconds: u64 = parts[2].parse().map_err(|_| {
            AnytronError::InvalidTimestamp(format!("Invalid seconds: {}", parts[2]))
        })?;
        let millis: u64 = parts[3]
            .parse()
            .map_err(|_| AnytronError::InvalidTimestamp(format!("Invalid millis: {}", parts[3])))?;

        if hours > 23 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid hours (must be 0-23): {}",
                hours
            )));
        }
        if minutes > 59 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid minutes (must be 0-59): {}",
                minutes
            )));
        }
        if seconds > 60 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid seconds (must be 0-60): {}",
                seconds
            )));
        }
        if millis > 999 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid milliseconds (must be 0-999): {}",
                millis
            )));
        }

        Ok(Self::from_hms_ms(hours, minutes, seconds, millis))
    }

    /// Parse ASS timestamp format: H:MM:SS.cc (centiseconds)
    pub fn parse_ass(s: &str) -> Result<Self> {
        let s = s.trim();
        let parts: Vec<&str> = s.split([':', '.']).collect();
        if parts.len() != 4 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid ASS timestamp: {}",
                s
            )));
        }

        let hours: u64 = parts[0]
            .parse()
            .map_err(|_| AnytronError::InvalidTimestamp(format!("Invalid hours: {}", parts[0])))?;
        let minutes: u64 = parts[1].parse().map_err(|_| {
            AnytronError::InvalidTimestamp(format!("Invalid minutes: {}", parts[1]))
        })?;
        let seconds: u64 = parts[2].parse().map_err(|_| {
            AnytronError::InvalidTimestamp(format!("Invalid seconds: {}", parts[2]))
        })?;
        let centis: u64 = parts[3].parse().map_err(|_| {
            AnytronError::InvalidTimestamp(format!("Invalid centiseconds: {}", parts[3]))
        })?;

        if hours > 23 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid hours (must be 0-23): {}",
                hours
            )));
        }
        if minutes > 59 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid minutes (must be 0-59): {}",
                minutes
            )));
        }
        if seconds > 60 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid seconds (must be 0-60): {}",
                seconds
            )));
        }
        if centis > 99 {
            return Err(AnytronError::InvalidTimestamp(format!(
                "Invalid centiseconds (must be 0-99): {}",
                centis
            )));
        }

        Ok(Self::from_hms_ms(hours, minutes, seconds, centis * 10))
    }

    /// Parse VTT timestamp format: HH:MM:SS.mmm or MM:SS.mmm
    pub fn parse_vtt(s: &str) -> Result<Self> {
        let s = s.trim();
        let parts: Vec<&str> = s.split([':', '.']).collect();

        match parts.len() {
            3 => {
                // MM:SS.mmm
                let minutes: u64 = parts[0].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid minutes: {}", parts[0]))
                })?;
                let seconds: u64 = parts[1].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid seconds: {}", parts[1]))
                })?;
                let millis: u64 = parts[2].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid millis: {}", parts[2]))
                })?;
                Ok(Self::from_hms_ms(0, minutes, seconds, millis))
            }
            4 => {
                // HH:MM:SS.mmm
                let hours: u64 = parts[0].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid hours: {}", parts[0]))
                })?;
                let minutes: u64 = parts[1].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid minutes: {}", parts[1]))
                })?;
                let seconds: u64 = parts[2].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid seconds: {}", parts[2]))
                })?;
                let millis: u64 = parts[3].parse().map_err(|_| {
                    AnytronError::InvalidTimestamp(format!("Invalid millis: {}", parts[3]))
                })?;
                Ok(Self::from_hms_ms(hours, minutes, seconds, millis))
            }
            _ => Err(AnytronError::InvalidTimestamp(format!(
                "Invalid VTT timestamp: {}",
                s
            ))),
        }
    }

    /// Format as FFmpeg seek time: HH:MM:SS.mmm
    pub fn to_ffmpeg(&self) -> String {
        let total_secs = self.0 / 1000;
        let millis = self.0 % 1000;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_secs = self.0 / 1000;
        let millis = self.0 % 1000;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, millis
        )
    }
}

/// A single subtitle entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleEntry {
    /// Sequential index (1-based from source file)
    pub index: usize,

    /// Start timestamp
    pub start: Timestamp,

    /// End timestamp
    pub end: Timestamp,

    /// Original text (may contain formatting)
    pub text: String,

    /// Cleaned text (no formatting tags)
    pub text_clean: String,
}

impl SubtitleEntry {
    /// Create a new subtitle entry
    pub fn new(index: usize, start: Timestamp, end: Timestamp, text: String) -> Self {
        let text_clean = Self::clean_text(&text);
        Self {
            index,
            start,
            end,
            text,
            text_clean,
        }
    }

    /// Remove formatting tags and normalize whitespace
    fn clean_text(text: &str) -> String {
        let text = RE_HTML.replace_all(text, "");
        let text = RE_ASS.replace_all(&text, "");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        text.trim().to_string()
    }

    /// Get the midpoint timestamp (useful for frame extraction)
    pub fn midpoint(&self) -> Timestamp {
        Timestamp((self.start.0 + self.end.0) / 2)
    }

    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> u64 {
        self.end.0.saturating_sub(self.start.0)
    }

    /// Generate a unique ID for this entry (e.g., "S01E01-12345")
    pub fn generate_id(&self, episode_id: &str) -> String {
        format!("{}-{}", episode_id, self.start.0)
    }
}

/// Subtitle format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    Srt,
    Ass,
    Vtt,
}

impl SubtitleFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "srt" => Some(Self::Srt),
            "ass" | "ssa" => Some(Self::Ass),
            "vtt" => Some(Self::Vtt),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_srt() {
        let ts = Timestamp::parse_srt("01:23:45,678").unwrap();
        // 1h 23m 45s 678ms = 3600000 + 1380000 + 45000 + 678 = 5025678
        assert_eq!(ts.0, 5025678);
    }

    #[test]
    fn test_timestamp_ass() {
        let ts = Timestamp::parse_ass("1:23:45.67").unwrap();
        // 1h 23m 45s 670ms = 3600000 + 1380000 + 45000 + 670 = 5025670
        assert_eq!(ts.0, 5025670);
    }

    #[test]
    fn test_timestamp_vtt() {
        let ts = Timestamp::parse_vtt("01:23:45.678").unwrap();
        // 1h 23m 45s 678ms = 3600000 + 1380000 + 45000 + 678 = 5025678
        assert_eq!(ts.0, 5025678);

        let ts2 = Timestamp::parse_vtt("23:45.678").unwrap();
        assert_eq!(ts2.0, 23 * 60 * 1000 + 45 * 1000 + 678);
    }

    #[test]
    fn test_clean_text() {
        let entry = SubtitleEntry::new(
            1,
            Timestamp(0),
            Timestamp(1000),
            "<i>Hello</i> <b>World</b>".to_string(),
        );
        assert_eq!(entry.text_clean, "Hello World");

        let entry2 = SubtitleEntry::new(
            1,
            Timestamp(0),
            Timestamp(1000),
            "{\\an8}Some text{\\pos(100,50)}".to_string(),
        );
        assert_eq!(entry2.text_clean, "Some text");
    }

    #[test]
    fn test_ffmpeg_format() {
        let ts = Timestamp(5025678); // 1h 23m 45s 678ms
        assert_eq!(ts.to_ffmpeg(), "01:23:45.678");
    }
}
