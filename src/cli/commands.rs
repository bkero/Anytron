//! Command implementations for the CLI

use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::Path;

use crate::cli::args::{GenerateArgs, ServeArgs, ValidateArgs};
use crate::config::Config;
use crate::discovery::Scanner;
use crate::extractor::FrameExtractor;
use crate::generator::SiteGenerator;
use crate::indexer::SearchIndexer;

/// Execute the generate command
pub fn generate(args: GenerateArgs, verbose: u8) -> Result<()> {
    let config = load_config(&args.input, args.config.as_deref())?;

    if verbose > 0 {
        log::info!("Configuration loaded: {:?}", config);
    }

    // Create output directory
    if args.clean && args.output.exists() {
        log::info!("Cleaning output directory: {:?}", args.output);
        std::fs::remove_dir_all(&args.output)
            .with_context(|| format!("Failed to clean output directory: {:?}", args.output))?;
    }

    std::fs::create_dir_all(&args.output)
        .with_context(|| format!("Failed to create output directory: {:?}", args.output))?;

    // Set up progress display
    let multi_progress = MultiProgress::new();
    let spinner_style = ProgressStyle::with_template("{spinner:.green} {prefix:.bold} {msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");

    // Phase 1: Discovery
    let discover_pb = multi_progress.add(ProgressBar::new_spinner());
    discover_pb.set_style(spinner_style.clone());
    discover_pb.set_prefix("[1/4]");
    discover_pb.set_message("Scanning for video and subtitle files...");
    discover_pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let scanner = Scanner::new(&args.input)
        .with_seasons(args.seasons.clone())
        .with_episodes(args.episodes.clone());

    let episodes = scanner
        .scan()
        .with_context(|| format!("Failed to scan directory: {:?}", args.input))?;

    discover_pb.finish_with_message(format!("Found {} episodes", episodes.len()));

    if episodes.is_empty() {
        anyhow::bail!("No episodes found in {:?}", args.input);
    }

    // Phase 2: Parse subtitles
    let subtitle_pb = multi_progress.add(ProgressBar::new(episodes.len() as u64));
    subtitle_pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {prefix:.bold} [{bar:40.cyan/blue}] {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("█▓▒░"),
    );
    subtitle_pb.set_prefix("[2/4]");
    subtitle_pb.set_message("Parsing subtitles...");

    let mut all_entries = Vec::new();
    for episode in &episodes {
        let entries = episode
            .parse_subtitles()
            .with_context(|| format!("Failed to parse subtitles for {:?}", episode.video_path))?;
        all_entries.push((episode.clone(), entries));
        subtitle_pb.inc(1);
    }
    subtitle_pb.finish_with_message("Subtitles parsed");

    let total_entries: usize = all_entries.iter().map(|(_, e)| e.len()).sum();
    log::info!("Total subtitle entries: {}", total_entries);

    // Phase 3: Frame extraction
    if !args.skip_frames {
        let frame_pb = multi_progress.add(ProgressBar::new(total_entries as u64));
        frame_pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} {prefix:.bold} [{bar:40.cyan/blue}] {pos}/{len} {msg} ({eta})",
            )
            .unwrap()
            .progress_chars("█▓▒░"),
        );
        frame_pb.set_prefix("[3/4]");
        frame_pb.set_message("Extracting frames...");

        let extractor = FrameExtractor::new()
            .with_quality(args.quality)
            .with_thumb_width(args.thumb_width)
            .with_jobs(args.jobs);

        for (episode, entries) in &all_entries {
            extractor
                .extract_frames(episode, entries, &args.output, &frame_pb)
                .with_context(|| {
                    format!("Failed to extract frames for {:?}", episode.video_path)
                })?;
        }
        frame_pb.finish_with_message("Frames extracted");
    } else {
        log::info!("Skipping frame extraction (--skip-frames)");
    }

    // Phase 4: Generate site
    let gen_pb = multi_progress.add(ProgressBar::new_spinner());
    gen_pb.set_style(spinner_style);
    gen_pb.set_prefix("[4/4]");
    gen_pb.set_message("Generating site...");
    gen_pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // Build search index
    let indexer = SearchIndexer::new();
    let index = indexer.build_index(&all_entries)?;

    // Generate HTML and assets
    let generator = SiteGenerator::new(&config, &args.output);
    generator.generate(&all_entries, &index)?;

    gen_pb.finish_with_message("Site generated");

    println!();
    println!("✓ Site generated successfully at {:?}", args.output);
    println!("  Run `anytron serve {:?}` to preview", args.output);

    Ok(())
}

/// Execute the validate command
pub fn validate(args: ValidateArgs, verbose: u8) -> Result<()> {
    let config = load_config(&args.input, args.config.as_deref())?;

    if verbose > 0 {
        log::info!("Configuration loaded: {:?}", config);
    }

    println!("Validating directory: {:?}", args.input);
    println!();

    let scanner = Scanner::new(&args.input);
    let episodes = scanner.scan()?;

    if episodes.is_empty() {
        println!("✗ No episodes found");
        return Ok(());
    }

    println!("✓ Found {} episodes", episodes.len());

    let mut total_errors = 0;
    let total_warnings = 0;

    for episode in &episodes {
        if args.detailed {
            println!();
            println!("  Episode: {}", episode.id);
            println!("    Video: {:?}", episode.video_path);
            println!("    Subtitle: {:?}", episode.subtitle_path);
        }

        match episode.parse_subtitles() {
            Ok(entries) => {
                if args.detailed {
                    println!("    ✓ {} subtitle entries", entries.len());
                }
            }
            Err(e) => {
                if args.detailed {
                    println!("    ✗ Parse error: {}", e);
                }
                total_errors += 1;
            }
        }
    }

    println!();
    if total_errors == 0 && total_warnings == 0 {
        println!("✓ Validation passed with no issues");
    } else {
        println!(
            "Validation complete: {} errors, {} warnings",
            total_errors, total_warnings
        );
    }

    Ok(())
}

/// Execute the serve command
pub fn serve(args: ServeArgs) -> Result<()> {
    if !args.directory.exists() {
        anyhow::bail!("Directory does not exist: {:?}", args.directory);
    }

    let addr = format!("{}:{}", args.bind, args.port);
    println!("Serving {:?} at http://{}", args.directory, addr);
    println!("Press Ctrl+C to stop");

    let server = tiny_http::Server::http(&addr)
        .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?;

    if args.open {
        if let Err(e) = open_browser(&format!("http://{}", addr)) {
            log::warn!("Failed to open browser: {}", e);
        }
    }

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        let path = url.trim_start_matches('/');
        let path = if path.is_empty() { "index.html" } else { path };
        let path = urlencoding::decode(path).unwrap_or_else(|_| path.into());

        let file_path = args.directory.join(path.as_ref());

        let response = if file_path.is_file() {
            let content = std::fs::read(&file_path)?;
            let content_type = guess_content_type(&file_path);
            tiny_http::Response::from_data(content).with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
                    .unwrap(),
            )
        } else {
            tiny_http::Response::from_string("404 Not Found")
                .with_status_code(404)
                .with_header(
                    tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..])
                        .unwrap(),
                )
        };

        let _ = request.respond(response);
    }

    Ok(())
}

/// Load configuration from file or use defaults
fn load_config(input_dir: &Path, config_path: Option<&Path>) -> Result<Config> {
    let config_file = config_path
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| input_dir.join("anytron.toml"));

    if config_file.exists() {
        Config::from_file(&config_file)
            .with_context(|| format!("Failed to load config from {:?}", config_file))
    } else {
        Ok(Config::default())
    }
}

/// Guess content type from file extension
fn guess_content_type(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
    .to_string()
}

/// Open URL in default browser
fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()?;
    }
    Ok(())
}
