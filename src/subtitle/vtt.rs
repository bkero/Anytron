//! WebVTT subtitle parser
//!
//! VTT format:
//! ```text
//! WEBVTT
//!
//! 00:00:01.000 --> 00:00:04.000
//! First subtitle text
//!
//! NOTE This is a comment
//!
//! 00:00:05.000 --> 00:00:08.000
//! Second subtitle text
//! ```

use std::path::Path;

use crate::error::{AnytronError, Result};
use crate::subtitle::types::{SubtitleEntry, Timestamp};

/// Parse a WebVTT file into subtitle entries
pub fn parse_file(path: &Path) -> Result<Vec<SubtitleEntry>> {
    let content = std::fs::read_to_string(path).map_err(|e| AnytronError::SubtitleParse {
        path: path.to_path_buf(),
        line: 0,
        message: format!("Failed to read file: {}", e),
    })?;

    parse_str(&content, path)
}

/// Parse VTT content string into subtitle entries
pub fn parse_str(content: &str, path: &Path) -> Result<Vec<SubtitleEntry>> {
    let mut entries = Vec::new();

    // Handle BOM if present
    let content = content.trim_start_matches('\u{feff}');

    // Split into blocks
    let blocks: Vec<&str> = content.split("\n\n").collect();
    let mut index = 0;

    for block in blocks {
        let block = block.trim();

        // Skip empty blocks
        if block.is_empty() {
            continue;
        }

        // Skip WEBVTT header
        if block.starts_with("WEBVTT") {
            continue;
        }

        // Skip NOTE blocks
        if block.starts_with("NOTE") {
            continue;
        }

        // Skip STYLE blocks
        if block.starts_with("STYLE") {
            continue;
        }

        // Skip REGION blocks
        if block.starts_with("REGION") {
            continue;
        }

        // Try to parse as a cue
        if let Some(entry) = parse_cue(block, &mut index, path)? {
            entries.push(entry);
        }
    }

    Ok(entries)
}

/// Parse a single VTT cue block
fn parse_cue(block: &str, index: &mut usize, path: &Path) -> Result<Option<SubtitleEntry>> {
    let lines: Vec<&str> = block.lines().collect();

    if lines.is_empty() {
        return Ok(None);
    }

    let mut line_idx = 0;

    // Optional cue identifier (skip it)
    if !lines[0].contains("-->") {
        line_idx = 1;
        if lines.len() < 2 {
            return Ok(None);
        }
    }

    // Parse timestamp line
    let timestamp_line = lines.get(line_idx).copied().unwrap_or("");
    if !timestamp_line.contains("-->") {
        return Ok(None); // Not a valid cue
    }

    let (start, end) =
        parse_timestamp_line(timestamp_line).map_err(|e| AnytronError::SubtitleParse {
            path: path.to_path_buf(),
            line: *index + 1,
            message: e,
        })?;

    // Parse text (remaining lines)
    let text = lines[(line_idx + 1)..].join("\n");

    if text.is_empty() {
        return Ok(None);
    }

    *index += 1;

    Ok(Some(SubtitleEntry::new(*index, start, end, text)))
}

/// Parse a timestamp line like "00:00:01.000 --> 00:00:04.000"
/// May include cue settings after the end timestamp
fn parse_timestamp_line(line: &str) -> std::result::Result<(Timestamp, Timestamp), String> {
    let parts: Vec<&str> = line.split("-->").collect();
    if parts.len() != 2 {
        return Err(format!("Invalid timestamp line: {}", line));
    }

    let start = Timestamp::parse_vtt(parts[0].trim())
        .map_err(|e| format!("Invalid start timestamp: {}", e))?;

    // End timestamp might have cue settings after it (separated by space)
    let end_part = parts[1].trim();
    let end_time = end_part.split_whitespace().next().unwrap_or(end_part);

    let end =
        Timestamp::parse_vtt(end_time).map_err(|e| format!("Invalid end timestamp: {}", e))?;

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_vtt() {
        let content = r#"WEBVTT

00:00:01.000 --> 00:00:04.000
Hello world

00:00:05.000 --> 00:00:08.000
Second line
with continuation
"#;

        let entries = parse_str(content, &PathBuf::from("test.vtt")).unwrap();
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].index, 1);
        assert_eq!(entries[0].start.0, 1000);
        assert_eq!(entries[0].end.0, 4000);
        assert_eq!(entries[0].text, "Hello world");

        assert_eq!(entries[1].index, 2);
        assert_eq!(entries[1].text, "Second line\nwith continuation");
    }

    #[test]
    fn test_parse_with_identifiers() {
        let content = r#"WEBVTT

1
00:00:01.000 --> 00:00:04.000
First cue

cue-2
00:00:05.000 --> 00:00:08.000
Second cue
"#;

        let entries = parse_str(content, &PathBuf::from("test.vtt")).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_parse_with_settings() {
        let content = r#"WEBVTT

00:00:01.000 --> 00:00:04.000 line:0 position:50%
Hello world
"#;

        let entries = parse_str(content, &PathBuf::from("test.vtt")).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].text, "Hello world");
    }

    #[test]
    fn test_skip_notes() {
        let content = r#"WEBVTT

NOTE This is a comment

00:00:01.000 --> 00:00:04.000
Hello world
"#;

        let entries = parse_str(content, &PathBuf::from("test.vtt")).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_short_timestamps() {
        let content = r#"WEBVTT

01:30.000 --> 02:00.000
Short timestamp format
"#;

        let entries = parse_str(content, &PathBuf::from("test.vtt")).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].start.0, 90000); // 1:30 = 90 seconds
    }
}
