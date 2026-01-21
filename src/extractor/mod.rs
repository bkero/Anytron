//! Frame and subtitle extraction module

mod ffmpeg;
mod subtitle;

pub use ffmpeg::FrameExtractor;
pub use subtitle::{SubtitleExtractor, SubtitleStream};
