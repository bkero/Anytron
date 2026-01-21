//! Subtitle parsing module
//!
//! Supports SRT, ASS/SSA, and WebVTT subtitle formats.

pub mod ass;
pub mod srt;
pub mod types;
pub mod vtt;

pub use types::{SubtitleEntry, SubtitleFormat, Timestamp};

use std::path::Path;

use crate::error::{AnytronError, Result};

/// Parse a subtitle file, auto-detecting the format from the file extension
pub fn parse_file(path: &Path) -> Result<Vec<SubtitleEntry>> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| AnytronError::UnsupportedSubtitleFormat("no extension".to_string()))?;

    let format = SubtitleFormat::from_extension(extension)
        .ok_or_else(|| AnytronError::UnsupportedSubtitleFormat(extension.to_string()))?;

    match format {
        SubtitleFormat::Srt => srt::parse_file(path),
        SubtitleFormat::Ass => ass::parse_file(path),
        SubtitleFormat::Vtt => vtt::parse_file(path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        assert_eq!(
            SubtitleFormat::from_extension("srt"),
            Some(SubtitleFormat::Srt)
        );
        assert_eq!(
            SubtitleFormat::from_extension("SRT"),
            Some(SubtitleFormat::Srt)
        );
        assert_eq!(
            SubtitleFormat::from_extension("ass"),
            Some(SubtitleFormat::Ass)
        );
        assert_eq!(
            SubtitleFormat::from_extension("ssa"),
            Some(SubtitleFormat::Ass)
        );
        assert_eq!(
            SubtitleFormat::from_extension("vtt"),
            Some(SubtitleFormat::Vtt)
        );
        assert_eq!(SubtitleFormat::from_extension("txt"), None);
    }
}
