//! Search index generator for lunr.js

use serde::{Deserialize, Serialize};

use crate::discovery::Episode;
use crate::error::Result;
use crate::subtitle::SubtitleEntry;

/// Search index entry for a single subtitle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntry {
    /// Unique ID (episode-timestamp)
    pub id: String,

    /// Searchable text (cleaned subtitle)
    pub text: String,

    /// Episode identifier (S01E01)
    pub episode: String,

    /// Timestamp in milliseconds
    pub timestamp: u64,

    /// Path to the full frame image
    pub frame: String,

    /// Path to the thumbnail image
    pub thumb: String,
}

/// The complete search index structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// All searchable entries
    pub entries: Vec<SearchEntry>,

    /// Metadata about the index
    pub meta: SearchMeta,
}

/// Metadata about the search index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMeta {
    /// Total number of entries
    pub total: usize,

    /// Number of episodes indexed
    pub episodes: usize,

    /// Index generation timestamp
    pub generated_at: String,

    /// Index version for cache busting
    pub version: String,
}

/// Builder for creating search indices
pub struct SearchIndexer {
    /// Fields to include in the index
    fields: Vec<String>,
}

impl Default for SearchIndexer {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchIndexer {
    /// Create a new search indexer
    pub fn new() -> Self {
        Self {
            fields: vec!["text".to_string(), "episode".to_string()],
        }
    }

    /// Set the fields to index
    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    /// Build the search index from episodes and their subtitle entries
    pub fn build_index(&self, episodes: &[(Episode, Vec<SubtitleEntry>)]) -> Result<SearchIndex> {
        let total_entries: usize = episodes.iter().map(|(_, subs)| subs.len()).sum();
        let mut entries = Vec::with_capacity(total_entries);

        for (episode, subs) in episodes {
            let episode_id = episode.id.to_string();

            for entry in subs {
                let timestamp = entry.midpoint().0;
                let id = format!("{}-{}", episode_id, timestamp);

                let frame = format!("img/frames/{}/{}.jpg", episode_id, timestamp);
                let thumb = format!("img/thumbs/{}/{}.jpg", episode_id, timestamp);

                entries.push(SearchEntry {
                    id,
                    text: entry.text_clean.clone(),
                    episode: episode_id.clone(),
                    timestamp,
                    frame,
                    thumb,
                });
            }
        }

        let meta = SearchMeta {
            total: entries.len(),
            episodes: episodes.len(),
            generated_at: chrono_now(),
            version: generate_version(&entries),
        };

        Ok(SearchIndex { entries, meta })
    }

    /// Build a lunr.js-compatible index configuration
    pub fn build_lunr_config(&self) -> serde_json::Value {
        serde_json::json!({
            "fields": self.fields,
            "ref": "id",
            "pipeline": ["trimmer", "stopWordFilter", "stemmer"]
        })
    }
}

/// Get current timestamp as ISO 8601 string
fn chrono_now() -> String {
    // Simple timestamp without external crate
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}

/// Generate a version string for cache busting
fn generate_version(entries: &[SearchEntry]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    entries.len().hash(&mut hasher);
    for entry in entries.iter().take(10) {
        entry.id.hash(&mut hasher);
    }
    format!("{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::{EpisodeId, SubtitleSource};
    use crate::subtitle::Timestamp;
    use std::path::PathBuf;

    fn create_test_episode() -> Episode {
        let subtitle_path = PathBuf::from("test.srt");
        Episode {
            id: EpisodeId::new(1, 1),
            video_path: PathBuf::from("test.mp4"),
            subtitle_path: subtitle_path.clone(),
            subtitle_source: SubtitleSource::External(subtitle_path),
        }
    }

    fn create_test_entries() -> Vec<SubtitleEntry> {
        vec![
            SubtitleEntry::new(
                1,
                Timestamp(1000),
                Timestamp(3000),
                "Hello world".to_string(),
            ),
            SubtitleEntry::new(
                2,
                Timestamp(4000),
                Timestamp(6000),
                "Goodbye world".to_string(),
            ),
        ]
    }

    #[test]
    fn test_build_index() {
        let indexer = SearchIndexer::new();
        let episode = create_test_episode();
        let entries = create_test_entries();

        let index = indexer.build_index(&[(episode, entries)]).unwrap();

        assert_eq!(index.entries.len(), 2);
        assert_eq!(index.meta.total, 2);
        assert_eq!(index.meta.episodes, 1);

        let first = &index.entries[0];
        assert_eq!(first.episode, "S01E01");
        assert_eq!(first.text, "Hello world");
        assert!(first.frame.contains("S01E01"));
    }

    #[test]
    fn test_lunr_config() {
        let indexer = SearchIndexer::new();
        let config = indexer.build_lunr_config();

        assert!(config["fields"].is_array());
        assert_eq!(config["ref"], "id");
    }
}
