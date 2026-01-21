//! Anytron CLI - Generate static websites for TV show quote search and meme generation

use anyhow::Result;
use clap::Parser;

use anytron::cli::{Cli, Commands};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set up logging based on verbosity
    let log_level = if cli.quiet {
        log::LevelFilter::Error
    } else {
        match cli.verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        }
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(None)
        .init();

    // Execute the appropriate command
    match cli.command {
        Commands::Generate(args) => {
            anytron::cli::commands::generate(args, cli.verbose)?;
        }
        Commands::Validate(args) => {
            anytron::cli::commands::validate(args, cli.verbose)?;
        }
        Commands::Serve(args) => {
            anytron::cli::commands::serve(args)?;
        }
    }

    Ok(())
}
