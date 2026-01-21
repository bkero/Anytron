//! Subtitle extraction from video container files (MKV, MP4, etc.)

use serde::Deserialize;
use std::path::Path;
use std::process::Command;

use crate::error::{AnytronError, Result};

/// Information about a subtitle stream in a video file
#[derive(Debug, Clone)]
pub struct SubtitleStream {
    /// Stream index in the container
    pub index: u32,

    /// Codec name (subrip, ass, webvtt, etc.)
    pub codec: String,

    /// Language code (eng, spa, fra, etc.)
    pub language: Option<String>,

    /// Track title
    pub title: Option<String>,

    /// Whether this is the default track
    pub is_default: bool,

    /// Whether this is a forced subtitle track
    pub is_forced: bool,

    /// Whether this is for hearing impaired (SDH/CC)
    pub is_hearing_impaired: bool,
}

impl SubtitleStream {
    /// Check if this stream is English
    pub fn is_english(&self) -> bool {
        if let Some(ref lang) = self.language {
            let lang_lower = lang.to_lowercase();
            lang_lower == "eng" || lang_lower == "en" || lang_lower == "english"
        } else {
            // Check title for English indication
            if let Some(ref title) = self.title {
                let title_lower = title.to_lowercase();
                title_lower.contains("english") || title_lower == "en" || title_lower == "eng"
            } else {
                false
            }
        }
    }

    /// Check if this stream appears to be SDH/CC (for hearing impaired)
    /// Checks both the disposition flag and title patterns
    pub fn appears_to_be_sdh(&self) -> bool {
        if self.is_hearing_impaired {
            return true;
        }

        // Check title for SDH/CC/HI indicators
        if let Some(ref title) = self.title {
            let title_lower = title.to_lowercase();
            return title_lower.contains("sdh")
                || title_lower.contains("[cc]")
                || title_lower.contains("(cc)")
                || title_lower.contains("hearing impaired")
                || title_lower.contains("closed caption");
        }

        false
    }

    /// Calculate priority score for track selection (higher = better)
    /// Prefers: English > default > non-SDH > non-forced
    pub fn priority_score(&self) -> i32 {
        let mut score = 0;

        // English is strongly preferred
        if self.is_english() {
            score += 1000;
        }

        // Default track gets a boost
        if self.is_default {
            score += 100;
        }

        // Non-SDH preferred over SDH (regular subs over hearing impaired)
        // Check both disposition flag AND title for SDH indicators
        if !self.appears_to_be_sdh() {
            score += 50;
        }

        // Non-forced preferred (forced usually only contain essential text)
        if !self.is_forced {
            score += 25;
        }

        // Prefer text-based formats over bitmap
        match self.codec.as_str() {
            "subrip" | "srt" => score += 10,
            "ass" | "ssa" => score += 9,
            "webvtt" | "vtt" => score += 8,
            "mov_text" => score += 7,
            _ => {}
        }

        score
    }
}

/// FFprobe JSON output structures
#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    streams: Vec<FFprobeStream>,
}

#[derive(Debug, Deserialize)]
struct FFprobeStream {
    index: u32,
    codec_name: Option<String>,
    codec_type: Option<String>,
    disposition: Option<FFprobeDisposition>,
    tags: Option<FFprobeTags>,
}

#[derive(Debug, Deserialize)]
struct FFprobeDisposition {
    default: Option<i32>,
    forced: Option<i32>,
    hearing_impaired: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct FFprobeTags {
    language: Option<String>,
    title: Option<String>,
    #[serde(rename = "LANGUAGE")]
    language_upper: Option<String>,
    #[serde(rename = "Title")]
    title_cap: Option<String>,
}

/// Subtitle extractor for embedded tracks
pub struct SubtitleExtractor;

impl SubtitleExtractor {
    /// Probe a video file for subtitle streams
    pub fn probe_streams(video_path: &Path) -> Result<Vec<SubtitleStream>> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_streams",
                "-select_streams",
                "s", // subtitle streams only
            ])
            .arg(video_path)
            .output()
            .map_err(|e| AnytronError::Ffmpeg(format!("Failed to run ffprobe: {}", e)))?;

        if !output.status.success() {
            // No subtitle streams is not an error, just return empty
            return Ok(Vec::new());
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let probe: FFprobeOutput = serde_json::from_str(&json_str)
            .map_err(|e| AnytronError::Ffmpeg(format!("Failed to parse ffprobe output: {}", e)))?;

        let streams = probe
            .streams
            .into_iter()
            .filter(|s| s.codec_type.as_deref() == Some("subtitle"))
            .map(|s| {
                let tags = s.tags.as_ref();
                let disposition = s.disposition.as_ref();

                SubtitleStream {
                    index: s.index,
                    codec: s.codec_name.unwrap_or_default(),
                    language: tags
                        .and_then(|t| t.language.clone().or_else(|| t.language_upper.clone())),
                    title: tags.and_then(|t| t.title.clone().or_else(|| t.title_cap.clone())),
                    is_default: disposition.and_then(|d| d.default).unwrap_or(0) == 1,
                    is_forced: disposition.and_then(|d| d.forced).unwrap_or(0) == 1,
                    is_hearing_impaired: disposition.and_then(|d| d.hearing_impaired).unwrap_or(0)
                        == 1,
                }
            })
            .collect();

        Ok(streams)
    }

    /// Select the best subtitle stream (prefers English, non-SDH)
    pub fn select_best_stream(streams: &[SubtitleStream]) -> Option<&SubtitleStream> {
        if streams.is_empty() {
            return None;
        }

        streams.iter().max_by_key(|s| s.priority_score())
    }

    /// Extract a subtitle stream to a file
    pub fn extract_stream(
        video_path: &Path,
        stream: &SubtitleStream,
        output_path: &Path,
    ) -> Result<()> {
        // Determine output format based on codec
        let output_format = match stream.codec.as_str() {
            "ass" | "ssa" => "ass",
            "webvtt" | "vtt" => "webvtt",
            _ => "srt", // Default to SRT for subrip and others
        };

        let output = Command::new("ffmpeg")
            .args(["-hide_banner", "-loglevel", "error", "-i"])
            .arg(video_path)
            .args([
                "-map",
                &format!("0:{}", stream.index),
                "-c:s",
                output_format,
                "-y",
            ])
            .arg(output_path)
            .output()
            .map_err(|e| AnytronError::Ffmpeg(format!("Failed to run ffmpeg: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AnytronError::Ffmpeg(format!(
                "Failed to extract subtitle stream {}: {}",
                stream.index, stderr
            )));
        }

        Ok(())
    }

    /// Extract the best subtitle stream from a video to a file
    /// Returns the path to the extracted subtitle, or None if no subtitles found
    pub fn extract_best_subtitle(
        video_path: &Path,
        output_dir: &Path,
    ) -> Result<Option<std::path::PathBuf>> {
        let streams = Self::probe_streams(video_path)?;

        if streams.is_empty() {
            log::debug!("No subtitle streams found in {:?}", video_path);
            return Ok(None);
        }

        log::debug!(
            "Found {} subtitle streams in {:?}",
            streams.len(),
            video_path
        );

        for stream in &streams {
            log::debug!(
                "  Stream {}: codec={}, lang={:?}, title={:?}, default={}, forced={}, sdh={}, score={}",
                stream.index,
                stream.codec,
                stream.language,
                stream.title,
                stream.is_default,
                stream.is_forced,
                stream.appears_to_be_sdh(),
                stream.priority_score()
            );
        }

        let best = Self::select_best_stream(&streams)
            .ok_or_else(|| AnytronError::Ffmpeg("No suitable subtitle stream found".to_string()))?;

        log::info!(
            "Selected subtitle stream {} ({:?}) from {:?}",
            best.index,
            best.language,
            video_path
        );

        // Determine output extension
        let ext = match best.codec.as_str() {
            "ass" | "ssa" => "ass",
            "webvtt" | "vtt" => "vtt",
            _ => "srt",
        };

        // Create output filename based on video filename
        let video_stem = video_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("subtitle");

        let output_path = output_dir.join(format!("{}.{}", video_stem, ext));

        // Create output directory if needed
        std::fs::create_dir_all(output_dir).map_err(|e| AnytronError::OutputDir {
            path: output_dir.to_path_buf(),
            source: e,
        })?;

        Self::extract_stream(video_path, best, &output_path)?;

        Ok(Some(output_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_english() {
        let stream = SubtitleStream {
            index: 0,
            codec: "subrip".to_string(),
            language: Some("eng".to_string()),
            title: None,
            is_default: false,
            is_forced: false,
            is_hearing_impaired: false,
        };
        assert!(stream.is_english());

        let stream2 = SubtitleStream {
            language: Some("spa".to_string()),
            ..stream.clone()
        };
        assert!(!stream2.is_english());

        let stream3 = SubtitleStream {
            language: None,
            title: Some("English".to_string()),
            ..stream.clone()
        };
        assert!(stream3.is_english());
    }

    #[test]
    fn test_priority_score() {
        let english_regular = SubtitleStream {
            index: 0,
            codec: "subrip".to_string(),
            language: Some("eng".to_string()),
            title: None,
            is_default: false,
            is_forced: false,
            is_hearing_impaired: false,
        };

        let english_sdh = SubtitleStream {
            is_hearing_impaired: true,
            ..english_regular.clone()
        };

        let spanish = SubtitleStream {
            language: Some("spa".to_string()),
            ..english_regular.clone()
        };

        // English regular should score highest
        assert!(english_regular.priority_score() > english_sdh.priority_score());
        assert!(english_regular.priority_score() > spanish.priority_score());
        assert!(english_sdh.priority_score() > spanish.priority_score());
    }

    #[test]
    fn test_select_best_stream() {
        let streams = vec![
            SubtitleStream {
                index: 0,
                codec: "subrip".to_string(),
                language: Some("spa".to_string()),
                title: Some("Spanish".to_string()),
                is_default: true,
                is_forced: false,
                is_hearing_impaired: false,
            },
            SubtitleStream {
                index: 1,
                codec: "subrip".to_string(),
                language: Some("eng".to_string()),
                title: Some("English".to_string()),
                is_default: false,
                is_forced: false,
                is_hearing_impaired: false,
            },
            SubtitleStream {
                index: 2,
                codec: "subrip".to_string(),
                language: Some("eng".to_string()),
                title: Some("English [SDH]".to_string()),
                is_default: false,
                is_forced: false,
                is_hearing_impaired: true,
            },
        ];

        let best = SubtitleExtractor::select_best_stream(&streams).unwrap();
        assert_eq!(best.index, 1); // English non-SDH
        assert!(best.is_english());
        assert!(!best.is_hearing_impaired);
    }
}
