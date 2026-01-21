//! ASS/SSA (Advanced SubStation Alpha) subtitle parser
//!
//! ASS format has sections like `[Script Info]`, `[Styles]`, `[Events]`.
//! We focus on the `[Events]` section for dialogue lines:
//! ```text
//! [Events]
//! Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
//! Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello world
//! ```

use std::path::Path;

use crate::error::{AnytronError, Result};
use crate::subtitle::types::{SubtitleEntry, Timestamp};

/// Parse an ASS/SSA file into subtitle entries
pub fn parse_file(path: &Path) -> Result<Vec<SubtitleEntry>> {
    let content = std::fs::read_to_string(path).map_err(|e| AnytronError::SubtitleParse {
        path: path.to_path_buf(),
        line: 0,
        message: format!("Failed to read file: {}", e),
    })?;

    parse_str(&content, path)
}

/// Parse ASS content string into subtitle entries
pub fn parse_str(content: &str, path: &Path) -> Result<Vec<SubtitleEntry>> {
    let mut entries = Vec::new();
    let mut in_events_section = false;
    let mut format_indices: Option<FormatIndices> = None;
    let mut index = 0;

    // Handle BOM if present
    let content = content.trim_start_matches('\u{feff}');

    // Normalize line endings (CRLF -> LF) for cross-platform compatibility
    let content = content.replace("\r\n", "\n");

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();

        // Track sections
        if line.starts_with('[') && line.ends_with(']') {
            in_events_section = line.eq_ignore_ascii_case("[events]");
            continue;
        }

        if !in_events_section {
            continue;
        }

        // Parse format line to get column indices
        if line.to_lowercase().starts_with("format:") {
            format_indices = Some(parse_format_line(line));
            continue;
        }

        // Parse dialogue lines
        if line.to_lowercase().starts_with("dialogue:") {
            let Some(ref fmt) = format_indices else {
                return Err(AnytronError::SubtitleParse {
                    path: path.to_path_buf(),
                    line: line_num + 1,
                    message: "Dialogue line before Format line".to_string(),
                });
            };

            if let Some(entry) = parse_dialogue_line(line, fmt, &mut index, path, line_num)? {
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

/// Indices for the Format fields we care about
#[derive(Debug)]
struct FormatIndices {
    start: usize,
    end: usize,
    text: usize,
    total_fields: usize,
}

/// Parse the Format line to extract column indices
fn parse_format_line(line: &str) -> FormatIndices {
    let fields_str = line.split_once(':').map(|x| x.1).unwrap_or("");
    let fields: Vec<&str> = fields_str.split(',').map(|s| s.trim()).collect();

    let mut start = 1;
    let mut end = 2;
    let mut text = fields.len().saturating_sub(1);

    for (i, field) in fields.iter().enumerate() {
        match field.to_lowercase().as_str() {
            "start" => start = i,
            "end" => end = i,
            "text" => text = i,
            _ => {}
        }
    }

    FormatIndices {
        start,
        end,
        text,
        total_fields: fields.len(),
    }
}

/// Parse a single Dialogue line
fn parse_dialogue_line(
    line: &str,
    fmt: &FormatIndices,
    index: &mut usize,
    path: &Path,
    line_num: usize,
) -> Result<Option<SubtitleEntry>> {
    // Remove "Dialogue:" prefix
    let content = line.split_once(':').map(|x| x.1).unwrap_or("").trim();

    // Split into fields, but the last field (Text) can contain commas
    let mut fields: Vec<&str> = Vec::with_capacity(fmt.total_fields);
    let mut remaining = content;

    for _ in 0..(fmt.total_fields - 1) {
        if let Some(comma_pos) = remaining.find(',') {
            fields.push(&remaining[..comma_pos]);
            remaining = &remaining[comma_pos + 1..];
        } else {
            fields.push(remaining);
            remaining = "";
        }
    }
    // The rest is the Text field
    fields.push(remaining);

    if fields.len() < fmt.total_fields {
        return Ok(None); // Skip malformed lines
    }

    let start =
        Timestamp::parse_ass(fields[fmt.start]).map_err(|e| AnytronError::SubtitleParse {
            path: path.to_path_buf(),
            line: line_num + 1,
            message: format!("Invalid start timestamp: {}", e),
        })?;

    let end = Timestamp::parse_ass(fields[fmt.end]).map_err(|e| AnytronError::SubtitleParse {
        path: path.to_path_buf(),
        line: line_num + 1,
        message: format!("Invalid end timestamp: {}", e),
    })?;

    let text = fields[fmt.text].to_string();

    // Convert ASS line breaks (\N) to actual newlines
    let text = text.replace("\\N", "\n").replace("\\n", "\n");

    *index += 1;

    Ok(Some(SubtitleEntry::new(*index, start, end, text)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_ass() {
        let content = r#"[Script Info]
Title: Test

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello world
Dialogue: 0,0:00:05.00,0:00:08.00,Default,,0,0,0,,Second line
"#;

        let entries = parse_str(content, &PathBuf::from("test.ass")).unwrap();
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].index, 1);
        assert_eq!(entries[0].start.0, 1000);
        assert_eq!(entries[0].end.0, 4000);
        assert_eq!(entries[0].text_clean, "Hello world");

        assert_eq!(entries[1].index, 2);
        assert_eq!(entries[1].start.0, 5000);
    }

    #[test]
    fn test_parse_with_formatting() {
        let content = r#"[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,{\an8}Top text
Dialogue: 0,0:00:05.00,0:00:08.00,Default,,0,0,0,,Line one\NLine two
"#;

        let entries = parse_str(content, &PathBuf::from("test.ass")).unwrap();
        assert_eq!(entries[0].text_clean, "Top text");
        assert!(entries[1].text.contains('\n'));
    }

    #[test]
    fn test_parse_text_with_commas() {
        let content = r#"[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello, world, how are you?
"#;

        let entries = parse_str(content, &PathBuf::from("test.ass")).unwrap();
        assert_eq!(entries[0].text_clean, "Hello, world, how are you?");
    }
}
