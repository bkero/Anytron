//! FFmpeg wrapper for frame extraction

use indicatif::ProgressBar;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::discovery::Episode;
use crate::error::{AnytronError, Result};
use crate::subtitle::{SubtitleEntry, Timestamp};

/// Frame extractor using FFmpeg
pub struct FrameExtractor {
    /// JPEG quality (1-100)
    quality: u8,

    /// Thumbnail width in pixels
    thumb_width: u32,

    /// Number of parallel jobs (None = use rayon default)
    jobs: Option<usize>,
}

impl Default for FrameExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameExtractor {
    /// Create a new frame extractor
    pub fn new() -> Self {
        Self {
            quality: 85,
            thumb_width: 320,
            jobs: None,
        }
    }

    /// Set JPEG quality
    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }

    /// Set thumbnail width
    pub fn with_thumb_width(mut self, width: u32) -> Self {
        self.thumb_width = width;
        self
    }

    /// Set number of parallel jobs
    pub fn with_jobs(mut self, jobs: Option<usize>) -> Self {
        self.jobs = jobs;
        self
    }

    /// Check if FFmpeg is available
    pub fn check_ffmpeg() -> Result<()> {
        let output = Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map_err(|_| AnytronError::FfmpegNotFound)?;

        if !output.status.success() {
            return Err(AnytronError::FfmpegNotFound);
        }

        Ok(())
    }

    /// Extract frames for all subtitle entries in an episode
    pub fn extract_frames(
        &self,
        episode: &Episode,
        entries: &[SubtitleEntry],
        output_dir: &Path,
        progress: &ProgressBar,
    ) -> Result<()> {
        Self::check_ffmpeg()?;

        let episode_id = episode.id.to_string();

        let frames_dir = output_dir.join("img").join("frames").join(&episode_id);
        let thumbs_dir = output_dir.join("img").join("thumbs").join(&episode_id);

        std::fs::create_dir_all(&frames_dir).map_err(|e| AnytronError::OutputDir {
            path: frames_dir.clone(),
            source: e,
        })?;
        std::fs::create_dir_all(&thumbs_dir).map_err(|e| AnytronError::OutputDir {
            path: thumbs_dir.clone(),
            source: e,
        })?;

        let tasks: Vec<ExtractionTask> = entries
            .iter()
            .map(|entry| {
                let timestamp = entry.midpoint();
                let frame_name = format!("{}.jpg", timestamp.0);
                ExtractionTask {
                    video_path: episode.video_path.clone(),
                    timestamp,
                    frame_path: frames_dir.join(&frame_name),
                    thumb_path: thumbs_dir.join(&frame_name),
                    quality: self.quality,
                    thumb_width: self.thumb_width,
                }
            })
            .collect();

        if let Some(num_jobs) = self.jobs {
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_jobs)
                .build_global()
                .ok();
        }

        let results: Vec<Result<()>> = tasks
            .par_iter()
            .map(|task| {
                let result = task.execute();
                progress.inc(1);
                result
            })
            .collect();

        for result in results {
            result?;
        }

        Ok(())
    }

    /// Extract a single frame at a specific timestamp
    pub fn extract_single_frame(
        &self,
        video_path: &Path,
        timestamp: Timestamp,
        output_path: &Path,
    ) -> Result<()> {
        let task = ExtractionTask {
            video_path: video_path.to_path_buf(),
            timestamp,
            frame_path: output_path.to_path_buf(),
            thumb_path: PathBuf::new(), // No thumbnail
            quality: self.quality,
            thumb_width: 0,
        };

        task.execute_frame_only()
    }
}

/// A single frame extraction task
struct ExtractionTask {
    video_path: PathBuf,
    timestamp: Timestamp,
    frame_path: PathBuf,
    thumb_path: PathBuf,
    quality: u8,
    thumb_width: u32,
}

impl ExtractionTask {
    /// Execute the extraction task (frame + thumbnail)
    fn execute(&self) -> Result<()> {
        // Skip if both already exist
        if self.frame_path.exists() && self.thumb_path.exists() {
            return Ok(());
        }

        let seek_time = self.timestamp.to_ffmpeg();

        // Extract full frame if needed
        if !self.frame_path.exists() {
            let output = Command::new("ffmpeg")
                .args([
                    "-hide_banner",
                    "-loglevel",
                    "error",
                    "-ss",
                    &seek_time,
                    "-i",
                ])
                .arg(&self.video_path)
                .args(["-frames:v", "1", "-q:v"])
                .arg(self.quality_to_qscale().to_string())
                .arg("-y")
                .arg(&self.frame_path)
                .output()
                .map_err(|e| AnytronError::Ffmpeg(e.to_string()))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(AnytronError::FrameExtraction {
                    video: self.video_path.clone(),
                    timestamp: self.timestamp.0,
                    message: stderr.to_string(),
                });
            }
        }

        // Extract thumbnail if needed
        if self.thumb_width > 0
            && !self.thumb_path.as_os_str().is_empty()
            && !self.thumb_path.exists()
        {
            let output = Command::new("ffmpeg")
                .args([
                    "-hide_banner",
                    "-loglevel",
                    "error",
                    "-ss",
                    &seek_time,
                    "-i",
                ])
                .arg(&self.video_path)
                .args(["-frames:v", "1", "-vf"])
                .arg(format!("scale={}:-1", self.thumb_width))
                .arg("-q:v")
                .arg((self.quality_to_qscale() + 2).min(31).to_string())
                .arg("-y")
                .arg(&self.thumb_path)
                .output()
                .map_err(|e| AnytronError::Ffmpeg(e.to_string()))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(AnytronError::FrameExtraction {
                    video: self.video_path.clone(),
                    timestamp: self.timestamp.0,
                    message: format!("thumbnail: {}", stderr),
                });
            }
        }

        Ok(())
    }

    /// Execute extraction for frame only (no thumbnail)
    fn execute_frame_only(&self) -> Result<()> {
        let seek_time = self.timestamp.to_ffmpeg();

        let output = Command::new("ffmpeg")
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("error")
            .arg("-ss")
            .arg(&seek_time)
            .arg("-i")
            .arg(&self.video_path)
            .arg("-frames:v")
            .arg("1")
            .arg("-q:v")
            .arg(self.quality_to_qscale().to_string())
            .arg("-y")
            .arg(&self.frame_path)
            .output()
            .map_err(|e| AnytronError::Ffmpeg(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AnytronError::FrameExtraction {
                video: self.video_path.clone(),
                timestamp: self.timestamp.0,
                message: stderr.to_string(),
            });
        }

        Ok(())
    }

    /// Convert quality (1-100) to FFmpeg qscale (31-1)
    fn quality_to_qscale(&self) -> u8 {
        // FFmpeg qscale: 1 = best, 31 = worst
        // Our quality: 1 = worst, 100 = best
        let normalized = (self.quality as f32 / 100.0).clamp(0.0, 1.0);
        let qscale = 31.0 - (normalized * 30.0);
        qscale.round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_to_qscale() {
        let task = ExtractionTask {
            video_path: PathBuf::new(),
            timestamp: Timestamp(0),
            frame_path: PathBuf::new(),
            thumb_path: PathBuf::new(),
            quality: 100,
            thumb_width: 320,
        };
        assert_eq!(task.quality_to_qscale(), 1);

        let task2 = ExtractionTask { quality: 1, ..task };
        assert_eq!(task2.quality_to_qscale(), 31);

        let task3 = ExtractionTask {
            quality: 85,
            ..task2
        };
        // 85% quality should be roughly qscale 5-6
        assert!(task3.quality_to_qscale() <= 6);
    }
}
