//! Directory scanner for video and subtitle files

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::discovery::episode::EpisodeId;
use crate::error::{AnytronError, Result};
use crate::extractor::SubtitleExtractor;
use crate::subtitle::{self, SubtitleEntry};

/// Video file extensions to look for
const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "wmv", "webm", "m4v"];

/// Subtitle file extensions to look for
const SUBTITLE_EXTENSIONS: &[&str] = &["srt", "ass", "ssa", "vtt"];

/// Patterns indicating English language in filenames (case-insensitive)
const ENGLISH_PATTERNS: &[&str] = &[
    ".en.",
    ".eng.",
    ".english.",
    "_en.",
    "_eng.",
    "_english.",
    "-en.",
    "-eng.",
    "-english.",
    ".en-us.",
    ".en-gb.",
    ".en_us.",
    ".en_gb.",
];

/// Patterns indicating non-English language (to deprioritize)
const NON_ENGLISH_PATTERNS: &[&str] = &[
    ".es.",
    ".spa.",
    ".spanish.",
    ".fr.",
    ".fra.",
    ".french.",
    ".de.",
    ".deu.",
    ".ger.",
    ".german.",
    ".it.",
    ".ita.",
    ".italian.",
    ".pt.",
    ".por.",
    ".portuguese.",
    ".ru.",
    ".rus.",
    ".russian.",
    ".ja.",
    ".jpn.",
    ".japanese.",
    ".ko.",
    ".kor.",
    ".korean.",
    ".zh.",
    ".chi.",
    ".chinese.",
];

/// Source of subtitle data
#[derive(Debug, Clone)]
pub enum SubtitleSource {
    /// External subtitle file
    External(PathBuf),
    /// Embedded in video container (extracted to temp path)
    Embedded {
        video_path: PathBuf,
        extracted_path: PathBuf,
    },
}

/// A discovered episode with video and subtitle files
#[derive(Debug, Clone)]
pub struct Episode {
    /// Episode identifier (SXXEXX)
    pub id: EpisodeId,

    /// Path to the video file
    pub video_path: PathBuf,

    /// Path to the subtitle file (external or extracted)
    pub subtitle_path: PathBuf,

    /// Source of the subtitle
    pub subtitle_source: SubtitleSource,
}

impl Episode {
    /// Parse the subtitle file and return entries
    pub fn parse_subtitles(&self) -> Result<Vec<SubtitleEntry>> {
        subtitle::parse_file(&self.subtitle_path)
    }
}

/// Scanner for discovering episodes in a directory
pub struct Scanner {
    /// Root directory to scan
    root: PathBuf,

    /// Filter by specific seasons
    seasons_filter: Option<Vec<u32>>,

    /// Filter by specific episodes (e.g., "S01E01")
    episodes_filter: Option<Vec<String>>,

    /// Directory for extracted subtitles cache
    cache_dir: Option<PathBuf>,
}

impl Scanner {
    /// Create a new scanner for the given directory
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            seasons_filter: None,
            episodes_filter: None,
            cache_dir: None,
        }
    }

    /// Filter to only include specific seasons
    pub fn with_seasons(mut self, seasons: Option<Vec<u32>>) -> Self {
        self.seasons_filter = seasons;
        self
    }

    /// Filter to only include specific episodes
    pub fn with_episodes(mut self, episodes: Option<Vec<String>>) -> Self {
        self.episodes_filter = episodes;
        self
    }

    /// Set cache directory for extracted subtitles
    pub fn with_cache_dir(mut self, cache_dir: Option<PathBuf>) -> Self {
        self.cache_dir = cache_dir;
        self
    }

    /// Scan the directory and return discovered episodes
    pub fn scan(&self) -> Result<Vec<Episode>> {
        if !self.root.exists() {
            return Err(AnytronError::Discovery(format!(
                "Directory does not exist: {:?}",
                self.root
            )));
        }

        // Collect all video and subtitle files
        let mut video_files: HashMap<EpisodeId, PathBuf> = HashMap::new();
        let mut subtitle_files: HashMap<EpisodeId, Vec<PathBuf>> = HashMap::new();

        for entry in WalkDir::new(&self.root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .unwrap_or_default();

            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            // Try to extract episode ID from filename
            let episode_id = match EpisodeId::from_filename(filename) {
                Ok(id) => id,
                Err(_) => continue,
            };

            // Apply filters
            if let Some(ref seasons) = self.seasons_filter {
                if !seasons.contains(&episode_id.season) {
                    continue;
                }
            }

            if let Some(ref episodes) = self.episodes_filter {
                let ep_str = episode_id.to_string();
                if !episodes.iter().any(|e| e.eq_ignore_ascii_case(&ep_str)) {
                    continue;
                }
            }

            // Categorize by file type
            if VIDEO_EXTENSIONS.contains(&extension.as_str()) {
                video_files
                    .entry(episode_id)
                    .or_insert_with(|| path.to_path_buf());
            } else if SUBTITLE_EXTENSIONS.contains(&extension.as_str()) {
                subtitle_files
                    .entry(episode_id)
                    .or_default()
                    .push(path.to_path_buf());
            }
        }

        // Determine cache directory for extracted subtitles
        let cache_dir = self
            .cache_dir
            .clone()
            .unwrap_or_else(|| self.root.join(".anytron_cache").join("subtitles"));

        // Match videos with subtitles
        let mut episodes: Vec<Episode> = Vec::new();

        for (id, video_path) in video_files {
            // First, try to find external subtitle file
            if let Some(subs) = subtitle_files.get(&id) {
                if let Some(subtitle_path) = Self::select_best_external_subtitle(subs) {
                    log::debug!("Using external subtitle for {}: {:?}", id, subtitle_path);
                    episodes.push(Episode {
                        id,
                        video_path,
                        subtitle_path: subtitle_path.clone(),
                        subtitle_source: SubtitleSource::External(subtitle_path),
                    });
                    continue;
                }
            }

            // No external subtitle found, try to extract from video container
            match SubtitleExtractor::extract_best_subtitle(&video_path, &cache_dir) {
                Ok(Some(extracted_path)) => {
                    log::info!(
                        "Extracted embedded subtitle for {}: {:?}",
                        id,
                        extracted_path
                    );
                    episodes.push(Episode {
                        id,
                        video_path: video_path.clone(),
                        subtitle_path: extracted_path.clone(),
                        subtitle_source: SubtitleSource::Embedded {
                            video_path,
                            extracted_path,
                        },
                    });
                }
                Ok(None) => {
                    log::warn!(
                        "No subtitle found for video: {:?} ({}) - no external file or embedded track",
                        video_path,
                        id
                    );
                }
                Err(e) => {
                    log::warn!("Failed to extract subtitle from {:?}: {}", video_path, e);
                }
            }
        }

        // Sort by episode ID
        episodes.sort_by(|a, b| a.id.cmp(&b.id));

        if episodes.is_empty() {
            return Err(AnytronError::NoVideosFound(self.root.clone()));
        }

        Ok(episodes)
    }

    /// Select the best external subtitle file from a list
    /// Prefers English, non-SDH tracks
    fn select_best_external_subtitle(paths: &[PathBuf]) -> Option<PathBuf> {
        if paths.is_empty() {
            return None;
        }

        if paths.len() == 1 {
            return Some(paths[0].clone());
        }

        // Score each subtitle file
        let mut scored: Vec<(i32, &PathBuf)> = paths
            .iter()
            .map(|p| (Self::score_external_subtitle(p), p))
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.0.cmp(&a.0));

        scored.first().map(|(_, p)| (*p).clone())
    }

    /// Score an external subtitle file (higher = better)
    fn score_external_subtitle(path: &Path) -> i32 {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Also check parent directory name
        let parent_name = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mut score = 0;

        // Check for English patterns
        for pattern in ENGLISH_PATTERNS {
            if filename.contains(pattern) {
                score += 1000;
                break;
            }
        }

        // Check parent directory for English
        if parent_name == "english" || parent_name == "eng" || parent_name == "en" {
            score += 500;
        }

        // Penalize non-English patterns
        for pattern in NON_ENGLISH_PATTERNS {
            if filename.contains(pattern) {
                score -= 500;
                break;
            }
        }

        // Penalize SDH/CC/HI subtitles
        if filename.contains(".sdh.")
            || filename.contains("_sdh.")
            || filename.contains("-sdh.")
            || filename.contains(".cc.")
            || filename.contains("_cc.")
            || filename.contains("-cc.")
            || filename.contains(".hi.")
            || filename.contains("_hi.")
            || filename.contains("-hi.")
            || filename.contains("[sdh]")
            || filename.contains("[cc]")
            || filename.contains("[hi]")
        {
            score -= 100;
        }

        // Prefer SRT format slightly
        if filename.ends_with(".srt") {
            score += 10;
        }

        score
    }

    /// Find subtitle file for a video (legacy method for compatibility)
    pub fn find_subtitle_for_video(&self, video_path: &Path) -> Option<PathBuf> {
        let video_stem = video_path.file_stem()?.to_str()?;
        let video_dir = video_path.parent()?;

        // Collect all matching subtitles
        let mut matches = Vec::new();

        // Check for subtitle with same name
        for ext in SUBTITLE_EXTENSIONS {
            let subtitle_path = video_dir.join(format!("{}.{}", video_stem, ext));
            if subtitle_path.exists() {
                matches.push(subtitle_path);
            }

            // Check for language-tagged variants
            for lang in &["en", "eng", "english"] {
                let subtitle_path = video_dir.join(format!("{}.{}.{}", video_stem, lang, ext));
                if subtitle_path.exists() {
                    matches.push(subtitle_path);
                }
            }
        }

        // Check for subtitle in a "subs" subdirectory
        let subs_dir = video_dir.join("subs");
        if subs_dir.exists() {
            for ext in SUBTITLE_EXTENSIONS {
                let subtitle_path = subs_dir.join(format!("{}.{}", video_stem, ext));
                if subtitle_path.exists() {
                    matches.push(subtitle_path);
                }
            }
        }

        // Check English subdirectory
        let eng_dir = video_dir.join("English");
        if eng_dir.exists() {
            for ext in SUBTITLE_EXTENSIONS {
                let subtitle_path = eng_dir.join(format!("{}.{}", video_stem, ext));
                if subtitle_path.exists() {
                    matches.push(subtitle_path);
                }
            }
        }

        Self::select_best_external_subtitle(&matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_extensions() {
        assert!(VIDEO_EXTENSIONS.contains(&"mp4"));
        assert!(VIDEO_EXTENSIONS.contains(&"mkv"));
        assert!(!VIDEO_EXTENSIONS.contains(&"srt"));
    }

    #[test]
    fn test_subtitle_extensions() {
        assert!(SUBTITLE_EXTENSIONS.contains(&"srt"));
        assert!(SUBTITLE_EXTENSIONS.contains(&"ass"));
        assert!(!SUBTITLE_EXTENSIONS.contains(&"mp4"));
    }

    #[test]
    fn test_score_external_subtitle() {
        let english = PathBuf::from("Show.S01E01.en.srt");
        let spanish = PathBuf::from("Show.S01E01.es.srt");
        let english_sdh = PathBuf::from("Show.S01E01.en.sdh.srt");
        let plain = PathBuf::from("Show.S01E01.srt");

        let score_en = Scanner::score_external_subtitle(&english);
        let score_es = Scanner::score_external_subtitle(&spanish);
        let score_en_sdh = Scanner::score_external_subtitle(&english_sdh);
        let score_plain = Scanner::score_external_subtitle(&plain);

        // English should score highest
        assert!(score_en > score_es);
        assert!(score_en > score_en_sdh);
        // English SDH should score higher than Spanish
        assert!(score_en_sdh > score_es);
        // Plain should score higher than explicit non-English
        assert!(score_plain > score_es);
    }

    #[test]
    fn test_select_best_external_subtitle() {
        let paths = vec![
            PathBuf::from("Show.S01E01.es.srt"),
            PathBuf::from("Show.S01E01.en.srt"),
            PathBuf::from("Show.S01E01.en.sdh.srt"),
        ];

        let best = Scanner::select_best_external_subtitle(&paths).unwrap();
        assert!(best.to_string_lossy().contains(".en."));
        assert!(!best.to_string_lossy().contains(".sdh."));
    }
}
