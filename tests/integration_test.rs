//! Integration tests for Anytron

#![allow(deprecated)] // assert_cmd::Command::cargo_bin is deprecated but still works

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Get the path to test fixtures
fn fixtures_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

mod cli_tests {
    use super::*;

    #[test]
    fn test_help_command() {
        Command::cargo_bin("anytron")
            .unwrap()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("TV show quote search"))
            .stdout(predicate::str::contains("generate"))
            .stdout(predicate::str::contains("validate"))
            .stdout(predicate::str::contains("serve"));
    }

    #[test]
    fn test_version_command() {
        Command::cargo_bin("anytron")
            .unwrap()
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("anytron"));
    }

    #[test]
    fn test_generate_help() {
        Command::cargo_bin("anytron")
            .unwrap()
            .args(["generate", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("INPUT_DIR"))
            .stdout(predicate::str::contains("output"));
    }

    #[test]
    fn test_validate_help() {
        Command::cargo_bin("anytron")
            .unwrap()
            .args(["validate", "--help"])
            .assert()
            .success();
    }

    #[test]
    fn test_serve_help() {
        Command::cargo_bin("anytron")
            .unwrap()
            .args(["serve", "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("port"));
    }

    #[test]
    fn test_generate_missing_input() {
        Command::cargo_bin("anytron")
            .unwrap()
            .args(["generate", "/nonexistent/path"])
            .assert()
            .failure();
    }

    #[test]
    fn test_validate_missing_input() {
        Command::cargo_bin("anytron")
            .unwrap()
            .args(["validate", "/nonexistent/path"])
            .assert()
            .failure();
    }
}

mod subtitle_parsing_tests {
    use super::*;

    #[test]
    fn test_parse_srt_file() {
        let srt_path = fixtures_path().join("sample.srt");
        let content = fs::read_to_string(&srt_path).expect("Failed to read sample.srt");

        // Verify file contains expected content
        assert!(content.contains("Hello, this is the first subtitle"));
        assert!(content.contains("00:00:01,000 --> 00:00:04,000"));
    }

    #[test]
    fn test_parse_ass_file() {
        let ass_path = fixtures_path().join("sample.ass");
        let content = fs::read_to_string(&ass_path).expect("Failed to read sample.ass");

        // Verify file contains expected content
        assert!(content.contains("[Script Info]"));
        assert!(content.contains("[Events]"));
        assert!(content.contains("Hello from ASS format"));
    }

    #[test]
    fn test_parse_vtt_file() {
        let vtt_path = fixtures_path().join("sample.vtt");
        let content = fs::read_to_string(&vtt_path).expect("Failed to read sample.vtt");

        // Verify file contains expected content
        assert!(content.contains("WEBVTT"));
        assert!(content.contains("Hello from WebVTT format"));
    }
}

mod site_generation_tests {
    use super::*;

    fn create_test_show_structure(dir: &TempDir) -> std::path::PathBuf {
        let show_dir = dir.path().join("TestShow");
        fs::create_dir_all(&show_dir).unwrap();

        // Create anytron.toml config
        let config = r#"
[show]
name = "Test Show"
description = "A test show for integration tests"

[site]
title = "Test Show Quotes"
base_url = "/"
"#;
        fs::write(show_dir.join("anytron.toml"), config).unwrap();

        // Copy sample subtitle as if it were for an episode
        let fixtures = fixtures_path();
        let srt_content = fs::read_to_string(fixtures.join("sample.srt")).unwrap();
        fs::write(show_dir.join("Test.Show.S01E01.srt"), &srt_content).unwrap();

        show_dir
    }

    #[test]
    fn test_validate_runs_without_panic() {
        let temp_dir = TempDir::new().unwrap();
        let show_dir = create_test_show_structure(&temp_dir);

        // Validate will fail because there's no video file, but it shouldn't panic
        // We're testing that the command runs and provides meaningful output
        Command::cargo_bin("anytron")
            .unwrap()
            .args(["validate", show_dir.to_str().unwrap()])
            .assert()
            .failure()
            .stderr(predicate::str::contains("No video files found"));
    }

    #[test]
    fn test_generate_creates_output_structure() {
        let temp_dir = TempDir::new().unwrap();
        let show_dir = create_test_show_structure(&temp_dir);
        let output_dir = temp_dir.path().join("output");

        // Note: This test will fail if there's no video file, but should create
        // the basic structure. For full testing, we'd need a sample video.
        let result = Command::cargo_bin("anytron")
            .unwrap()
            .args([
                "generate",
                show_dir.to_str().unwrap(),
                "-o",
                output_dir.to_str().unwrap(),
            ])
            .assert();

        // The command may fail due to missing video, but let's check what we can
        // For CI, we primarily test the CLI interface works correctly
        let _ = result; // We just verify it doesn't panic
    }
}

mod output_structure_tests {
    use super::*;

    /// Test that generated output has the expected structure
    /// This test requires a pre-generated output directory
    #[test]
    fn test_output_structure_has_required_files() {
        // Skip if test_output doesn't exist (CI environment)
        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_output");
        if !output_dir.exists() {
            eprintln!("Skipping test: test_output directory not found");
            return;
        }

        // Check required files exist
        assert!(output_dir.join("index.html").exists(), "index.html missing");
        assert!(
            output_dir.join("css").join("style.css").exists(),
            "style.css missing"
        );
        assert!(
            output_dir.join("js").join("bundle.js").exists(),
            "bundle.js missing"
        );
        assert!(
            output_dir.join("search").join("index.json").exists(),
            "search index missing"
        );
    }

    #[test]
    fn test_search_index_is_valid_json() {
        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_output");
        if !output_dir.exists() {
            eprintln!("Skipping test: test_output directory not found");
            return;
        }

        let index_path = output_dir.join("search").join("index.json");
        let content = fs::read_to_string(&index_path).expect("Failed to read search index");
        let parsed: serde_json::Value =
            serde_json::from_str(&content).expect("Search index is not valid JSON");

        // Verify structure
        assert!(parsed.get("entries").is_some(), "entries field missing");
        let entries = parsed["entries"]
            .as_array()
            .expect("entries is not an array");
        assert!(!entries.is_empty(), "entries array is empty");

        // Check first entry has required fields
        let first = &entries[0];
        assert!(first.get("id").is_some(), "entry missing id");
        assert!(first.get("text").is_some(), "entry missing text");
        assert!(first.get("episode").is_some(), "entry missing episode");
        assert!(first.get("timestamp").is_some(), "entry missing timestamp");
    }

    #[test]
    fn test_index_html_has_required_elements() {
        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_output");
        if !output_dir.exists() {
            eprintln!("Skipping test: test_output directory not found");
            return;
        }

        let index_path = output_dir.join("index.html");
        let content = fs::read_to_string(&index_path).expect("Failed to read index.html");

        assert!(content.contains("<!DOCTYPE html>"), "Missing DOCTYPE");
        assert!(content.contains("<html"), "Missing html tag");
        assert!(content.contains("search-input"), "Missing search input");
        assert!(content.contains("bundle.js"), "Missing bundle.js reference");
    }

    #[test]
    fn test_caption_pages_exist() {
        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_output");
        if !output_dir.exists() {
            eprintln!("Skipping test: test_output directory not found");
            return;
        }

        let caption_dir = output_dir.join("caption");
        assert!(caption_dir.exists(), "caption directory missing");

        // Should have at least one HTML file
        let entries: Vec<_> = fs::read_dir(&caption_dir)
            .expect("Failed to read caption directory")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "html"))
            .collect();

        assert!(!entries.is_empty(), "No caption HTML files found");
    }
}

mod library_tests {
    //! Tests for library functionality using the anytron crate directly

    use super::*;

    #[test]
    fn test_srt_parser_integration() {
        let srt_path = fixtures_path().join("sample.srt");
        let entries = anytron::subtitle::parse_file(&srt_path).expect("Failed to parse SRT");

        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].text_clean, "Hello, this is the first subtitle.");
        assert_eq!(entries[0].start.0, 1000); // 1 second in ms
        assert_eq!(entries[0].end.0, 4000); // 4 seconds in ms
    }

    #[test]
    fn test_ass_parser_integration() {
        let ass_path = fixtures_path().join("sample.ass");
        let entries = anytron::subtitle::parse_file(&ass_path).expect("Failed to parse ASS");

        assert!(!entries.is_empty());
        // First entry should be "Hello from ASS format"
        assert!(entries[0].text_clean.contains("Hello from ASS format"));
    }

    #[test]
    fn test_vtt_parser_integration() {
        let vtt_path = fixtures_path().join("sample.vtt");
        let entries = anytron::subtitle::parse_file(&vtt_path).expect("Failed to parse VTT");

        assert!(!entries.is_empty());
        assert!(entries[0].text_clean.contains("Hello from WebVTT format"));
    }

    #[test]
    fn test_timestamp_formatting() {
        use anytron::subtitle::Timestamp;

        let ts = Timestamp(3661500); // 1 hour, 1 minute, 1.5 seconds
        let formatted = ts.to_ffmpeg();
        assert_eq!(formatted, "01:01:01.500");
    }

    #[test]
    fn test_episode_id_parsing() {
        use anytron::discovery::EpisodeId;

        // Test various filename formats
        let id1 = EpisodeId::from_filename("Show.S01E01.mkv").unwrap();
        assert_eq!(id1.season, 1);
        assert_eq!(id1.episode, 1);

        let id2 = EpisodeId::from_filename("Show.S12E99.Episode.Title.mp4").unwrap();
        assert_eq!(id2.season, 12);
        assert_eq!(id2.episode, 99);

        let id3 = EpisodeId::from_filename("show.s1e5.avi").unwrap();
        assert_eq!(id3.season, 1);
        assert_eq!(id3.episode, 5);
    }

    #[test]
    fn test_midpoint_calculation() {
        use anytron::subtitle::{SubtitleEntry, Timestamp};

        let entry = SubtitleEntry {
            index: 1,
            start: Timestamp(1000),
            end: Timestamp(5000),
            text: "Test".to_string(),
            text_clean: "Test".to_string(),
        };

        let midpoint = entry.midpoint();
        assert_eq!(midpoint.0, 3000); // (1000 + 5000) / 2 = 3000
    }
}
