//! Discovery module for finding video and subtitle files

pub mod episode;
pub mod scanner;

pub use episode::EpisodeId;
pub use scanner::{Episode, Scanner, SubtitleSource};
