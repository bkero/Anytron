//! SRT (SubRip) subtitle parser
//!
//! SRT format:
//! ```text
//! 1
//! 00:00:01,000 --> 00:00:04,000
//! First subtitle text
//!
//! 2
//! 00:00:05,000 --> 00:00:08,000
//! Second subtitle text
//! with multiple lines
//! ```

use std::path::Path;

use crate::error::{AnytronError, Result};
use crate::subtitle::types::{SubtitleEntry, Timestamp};

/// Parse an SRT file into subtitle entries
pub fn parse_file(path: &Path) -> Result<Vec<SubtitleEntry>> {
    let content = std::fs::read_to_string(path).map_err(|e| AnytronError::SubtitleParse {
        path: path.to_path_buf(),
        line: 0,
        message: format!("Failed to read file: {}", e),
    })?;

    parse_str(&content, path)
}

/// Parse SRT content string into subtitle entries
pub fn parse_str(content: &str, path: &Path) -> Result<Vec<SubtitleEntry>> {
    let mut entries = Vec::new();

    // Handle BOM if present
    let content = content.trim_start_matches('\u{feff}');

    // Split into blocks (separated by blank lines)
    let blocks: Vec<&str> = content.split("\n\n").collect();

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        let lines: Vec<&str> = block.lines().collect();
        if lines.len() < 3 {
            // Skip malformed blocks
            continue;
        }

        // Parse index (first line)
        let index: usize = match lines[0].trim().parse() {
            Ok(i) => i,
            Err(_) => continue, // Skip malformed entries
        };

        // Parse timestamps (second line)
        let timestamp_line = lines[1].trim();
        let (start, end) =
            parse_timestamp_line(timestamp_line).map_err(|e| AnytronError::SubtitleParse {
                path: path.to_path_buf(),
                line: index,
                message: e,
            })?;

        // Parse text (remaining lines)
        let text = lines[2..].join("\n");

        entries.push(SubtitleEntry::new(index, start, end, text));
    }

    Ok(entries)
}

/// Parse a timestamp line like "00:00:01,000 --> 00:00:04,000"
fn parse_timestamp_line(line: &str) -> std::result::Result<(Timestamp, Timestamp), String> {
    let parts: Vec<&str> = line.split("-->").collect();
    if parts.len() != 2 {
        return Err(format!("Invalid timestamp line: {}", line));
    }

    let start = Timestamp::parse_srt(parts[0].trim())
        .map_err(|e| format!("Invalid start timestamp: {}", e))?;
    let end = Timestamp::parse_srt(parts[1].trim())
        .map_err(|e| format!("Invalid end timestamp: {}", e))?;

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_srt() {
        let content = r#"1
00:00:01,000 --> 00:00:04,000
Hello world

2
00:00:05,000 --> 00:00:08,000
Second line
with continuation
"#;

        let entries = parse_str(content, &PathBuf::from("test.srt")).unwrap();
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].index, 1);
        assert_eq!(entries[0].start.0, 1000);
        assert_eq!(entries[0].end.0, 4000);
        assert_eq!(entries[0].text, "Hello world");

        assert_eq!(entries[1].index, 2);
        assert_eq!(entries[1].text, "Second line\nwith continuation");
    }

    #[test]
    fn test_parse_with_formatting() {
        let content = r#"1
00:00:01,000 --> 00:00:04,000
<i>Italic text</i> and <b>bold</b>
"#;

        let entries = parse_str(content, &PathBuf::from("test.srt")).unwrap();
        assert_eq!(entries[0].text_clean, "Italic text and bold");
    }

    #[test]
    fn test_parse_with_bom() {
        let content = "\u{feff}1\n00:00:01,000 --> 00:00:04,000\nText";

        let entries = parse_str(content, &PathBuf::from("test.srt")).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].text, "Text");
    }
}
