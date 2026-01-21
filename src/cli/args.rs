//! CLI argument definitions using clap derive macros

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Anytron - Generate static websites for TV show quote search and meme generation
#[derive(Parser, Debug)]
#[command(name = "anytron")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Verbose output (-v, -vv, -vvv for increasing verbosity)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a static website from video and subtitle files
    Generate(GenerateArgs),

    /// Validate input directory structure and subtitle files
    Validate(ValidateArgs),

    /// Serve the generated site locally for preview
    Serve(ServeArgs),
}

/// Arguments for the generate command
#[derive(Parser, Debug)]
pub struct GenerateArgs {
    /// Input directory containing video and subtitle files
    #[arg(value_name = "INPUT_DIR")]
    pub input: PathBuf,

    /// Output directory for the generated site
    #[arg(short, long, default_value = "output")]
    pub output: PathBuf,

    /// Configuration file path (default: INPUT_DIR/anytron.toml)
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Number of parallel workers (default: number of CPU cores)
    #[arg(short = 'j', long)]
    pub jobs: Option<usize>,

    /// Skip frame extraction (use existing frames)
    #[arg(long)]
    pub skip_frames: bool,

    /// Only process specific seasons (e.g., 1,2,3)
    #[arg(long, value_delimiter = ',')]
    pub seasons: Option<Vec<u32>>,

    /// Only process specific episodes (e.g., S01E01,S01E02)
    #[arg(long, value_delimiter = ',')]
    pub episodes: Option<Vec<String>>,

    /// Frame extraction interval in milliseconds
    #[arg(long, default_value = "1000")]
    pub interval: u64,

    /// JPEG quality for frames (1-100)
    #[arg(long, default_value = "85")]
    pub quality: u8,

    /// Thumbnail width in pixels
    #[arg(long, default_value = "320")]
    pub thumb_width: u32,

    /// Clean output directory before generating
    #[arg(long)]
    pub clean: bool,
}

/// Arguments for the validate command
#[derive(Parser, Debug)]
pub struct ValidateArgs {
    /// Input directory to validate
    #[arg(value_name = "INPUT_DIR")]
    pub input: PathBuf,

    /// Configuration file path (default: INPUT_DIR/anytron.toml)
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Show detailed validation results
    #[arg(long)]
    pub detailed: bool,
}

/// Arguments for the serve command
#[derive(Parser, Debug)]
pub struct ServeArgs {
    /// Directory to serve (default: ./output)
    #[arg(value_name = "DIR", default_value = "output")]
    pub directory: PathBuf,

    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Bind address
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind: String,

    /// Open browser automatically
    #[arg(long)]
    pub open: bool,
}
