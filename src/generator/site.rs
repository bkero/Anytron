//! Site generation orchestration

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::discovery::Episode;
use crate::error::{AnytronError, Result};
use crate::generator::assets::AssetBundler;
use crate::generator::html::HtmlGenerator;
use crate::indexer::{SearchEntry, SearchIndex};
use crate::subtitle::SubtitleEntry;

/// Site generator - orchestrates all generation tasks
pub struct SiteGenerator<'a> {
    config: &'a Config,
    output_dir: PathBuf,
}

impl<'a> SiteGenerator<'a> {
    /// Create a new site generator
    pub fn new(config: &'a Config, output_dir: &Path) -> Self {
        Self {
            config,
            output_dir: output_dir.to_path_buf(),
        }
    }

    /// Generate the complete site
    pub fn generate(
        &self,
        episodes: &[(Episode, Vec<SubtitleEntry>)],
        index: &SearchIndex,
    ) -> Result<()> {
        // Create output directories
        self.create_directories()?;

        // Generate HTML pages
        self.generate_html(episodes, index)?;

        // Write search index
        self.write_search_index(index)?;

        // Bundle and copy assets
        self.bundle_assets()?;

        Ok(())
    }

    /// Create required output directories
    fn create_directories(&self) -> Result<()> {
        let dirs = ["css", "js", "search", "caption", "img/frames", "img/thumbs"];

        for dir in dirs {
            let path = self.output_dir.join(dir);
            std::fs::create_dir_all(&path)
                .map_err(|e| AnytronError::OutputDir { path, source: e })?;
        }

        Ok(())
    }

    /// Generate all HTML pages
    fn generate_html(
        &self,
        episodes: &[(Episode, Vec<SubtitleEntry>)],
        index: &SearchIndex,
    ) -> Result<()> {
        let html_gen = HtmlGenerator::new(self.config);

        // Generate index page
        html_gen.generate_index(&self.output_dir.join("index.html"))?;

        // Build lookup maps for navigation
        let entry_map: HashMap<&str, (&SearchEntry, &SubtitleEntry, &Episode)> = episodes
            .iter()
            .flat_map(|(episode, subs)| {
                subs.iter().filter_map(move |sub| {
                    let id = format!("{}-{}", episode.id, sub.midpoint().0);
                    index
                        .entries
                        .iter()
                        .find(|e| e.id == id)
                        .map(|entry| (entry.id.as_str(), (entry, sub, episode)))
                })
            })
            .collect();

        // Sort entries for navigation
        let mut sorted_entries: Vec<&SearchEntry> = index.entries.iter().collect();
        sorted_entries.sort_by(|a, b| {
            a.episode
                .cmp(&b.episode)
                .then(a.timestamp.cmp(&b.timestamp))
        });

        // Generate caption pages
        for (i, entry) in sorted_entries.iter().enumerate() {
            let prev = if i > 0 {
                Some(sorted_entries[i - 1])
            } else {
                None
            };
            let next = sorted_entries.get(i + 1).copied();

            if let Some((_, subtitle, episode)) = entry_map.get(entry.id.as_str()) {
                let output_path = self
                    .output_dir
                    .join("caption")
                    .join(format!("{}.html", entry.id));

                html_gen.generate_caption(entry, subtitle, episode, prev, next, &output_path)?;
            }
        }

        Ok(())
    }

    /// Write the search index JSON
    fn write_search_index(&self, index: &SearchIndex) -> Result<()> {
        let index_path = self.output_dir.join("search").join("index.json");

        let json = serde_json::to_string_pretty(index)
            .map_err(|e| AnytronError::Output(format!("Failed to serialize index: {}", e)))?;

        std::fs::write(&index_path, json).map_err(|e| AnytronError::FileWrite {
            path: index_path,
            source: e,
        })
    }

    /// Bundle and copy static assets
    fn bundle_assets(&self) -> Result<()> {
        let bundler = AssetBundler::new();

        // Write CSS
        bundler.write_css(&self.output_dir.join("css").join("style.css"))?;

        // Write bundled JS
        bundler.write_js(&self.output_dir.join("js").join("bundle.js"))?;

        Ok(())
    }
}
