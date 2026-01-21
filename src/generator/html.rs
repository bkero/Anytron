//! HTML page generation using minijinja templates

use minijinja::{context, Environment};
use std::path::Path;

use crate::config::Config;
use crate::discovery::Episode;
use crate::error::{AnytronError, Result};
use crate::indexer::SearchEntry;
use crate::subtitle::SubtitleEntry;

/// HTML generator with template support
pub struct HtmlGenerator<'a> {
    config: &'a Config,
    env: Environment<'a>,
}

impl<'a> HtmlGenerator<'a> {
    /// Create a new HTML generator
    pub fn new(config: &'a Config) -> Self {
        let mut env = Environment::new();

        // Add index template
        env.add_template("index.html", INDEX_TEMPLATE)
            .expect("Failed to add index template");

        // Add caption page template
        env.add_template("caption.html", CAPTION_TEMPLATE)
            .expect("Failed to add caption template");

        Self { config, env }
    }

    /// Generate the main index/search page
    pub fn generate_index(&self, output_path: &Path) -> Result<()> {
        let template = self
            .env
            .get_template("index.html")
            .map_err(|e| AnytronError::Template(e.to_string()))?;

        let html = template
            .render(context! {
                title => &self.config.site.title,
                show_name => &self.config.show.name,
                description => &self.config.show.description,
                base_url => &self.config.site.base_url,
                theme_color => &self.config.site.theme_color,
                enable_memes => self.config.site.enable_memes,
            })
            .map_err(|e| AnytronError::Template(e.to_string()))?;

        std::fs::write(output_path, html).map_err(|e| AnytronError::FileWrite {
            path: output_path.to_path_buf(),
            source: e,
        })
    }

    /// Generate a caption detail page
    pub fn generate_caption(
        &self,
        entry: &SearchEntry,
        subtitle: &SubtitleEntry,
        _episode: &Episode,
        prev: Option<&SearchEntry>,
        next: Option<&SearchEntry>,
        output_path: &Path,
    ) -> Result<()> {
        let template = self
            .env
            .get_template("caption.html")
            .map_err(|e| AnytronError::Template(e.to_string()))?;

        let html = template
            .render(context! {
                title => &self.config.site.title,
                show_name => &self.config.show.name,
                base_url => &self.config.site.base_url,
                theme_color => &self.config.site.theme_color,
                enable_memes => self.config.site.enable_memes,

                // Entry data
                id => &entry.id,
                text => &subtitle.text,
                text_clean => &subtitle.text_clean,
                episode => &entry.episode,
                timestamp => entry.timestamp,
                timestamp_formatted => format_timestamp(entry.timestamp),
                frame => &entry.frame,
                thumb => &entry.thumb,

                // Navigation
                prev_id => prev.map(|p| &p.id),
                prev_thumb => prev.map(|p| &p.thumb),
                next_id => next.map(|n| &n.id),
                next_thumb => next.map(|n| &n.thumb),
            })
            .map_err(|e| AnytronError::Template(e.to_string()))?;

        // Create parent directories
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| AnytronError::OutputDir {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        std::fs::write(output_path, html).map_err(|e| AnytronError::FileWrite {
            path: output_path.to_path_buf(),
            source: e,
        })
    }
}

/// Format timestamp as HH:MM:SS
fn format_timestamp(ms: u64) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// Index page template
const INDEX_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }}</title>
    <meta name="description" content="{{ description }}">
    <meta name="theme-color" content="{{ theme_color }}">
    <link rel="stylesheet" href="{{ base_url }}css/style.css">
</head>
<body>
    <header class="header">
        <h1 class="header__title">{{ show_name }}</h1>
        <p class="header__subtitle">Quote Search & Meme Generator</p>
    </header>

    <main class="main">
        <section class="search-section">
            <form class="search-form" id="search-form">
                <input
                    type="search"
                    class="search-input"
                    id="search-input"
                    placeholder="Search for a quote..."
                    autocomplete="off"
                    autofocus
                >
                <button type="submit" class="search-button">Search</button>
            </form>
        </section>

        <section class="results-section" id="results">
            <div class="results-info" id="results-info"></div>
            <div class="results-grid" id="results-grid"></div>
        </section>
    </main>

    <footer class="footer">
        <p>Powered by <a href="https://github.com/anytron/anytron">Anytron</a></p>
    </footer>

    <script src="{{ base_url }}js/bundle.js"></script>
</body>
</html>
"#;

/// Caption page template
const CAPTION_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ text_clean }} - {{ show_name }}</title>
    <meta name="theme-color" content="{{ theme_color }}">

    <!-- Open Graph -->
    <meta property="og:title" content="{{ text_clean }}">
    <meta property="og:image" content="{{ base_url }}{{ frame }}">
    <meta property="og:type" content="website">

    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="{{ text_clean }}">
    <meta name="twitter:image" content="{{ base_url }}{{ frame }}">

    <link rel="stylesheet" href="{{ base_url }}css/style.css">
</head>
<body>
    <header class="header">
        <a href="{{ base_url }}" class="header__back">&larr; Back to Search</a>
        <h1 class="header__title">{{ show_name }}</h1>
    </header>

    <main class="main caption-page">
        <section class="caption-section">
            <div class="caption-image-container">
                <img
                    src="{{ base_url }}{{ frame }}"
                    alt="{{ text_clean }}"
                    class="caption-image"
                    id="caption-image"
                >
                {% if enable_memes %}
                <div class="caption-overlay" id="caption-overlay">
                    <span class="caption-text" id="caption-text">{{ text_clean }}</span>
                </div>
                {% endif %}
            </div>

            <div class="caption-info">
                <p class="caption-quote">"{{ text_clean }}"</p>
                <p class="caption-meta">
                    <span class="caption-episode">{{ episode }}</span>
                    <span class="caption-timestamp">{{ timestamp_formatted }}</span>
                </p>
            </div>

            {% if enable_memes %}
            <div class="meme-controls">
                <h3>Meme Generator</h3>
                <div class="meme-form">
                    <textarea
                        id="meme-text"
                        class="meme-textarea"
                        placeholder="Enter custom text (or leave empty for original caption)"
                    >{{ text_clean }}</textarea>
                    <div class="meme-options">
                        <label>
                            <input type="checkbox" id="meme-outline" checked>
                            Text outline
                        </label>
                        <label>
                            Font size:
                            <input type="range" id="meme-fontsize" min="12" max="48" value="24">
                        </label>
                    </div>
                    <button id="meme-download" class="meme-button">Download Meme</button>
                </div>
            </div>
            {% endif %}
        </section>

        <nav class="caption-nav">
            {% if prev_id %}
            <a href="{{ base_url }}caption/{{ prev_id }}.html" class="caption-nav__link caption-nav__prev">
                <img src="{{ base_url }}{{ prev_thumb }}" alt="Previous" class="caption-nav__thumb">
                <span>&larr; Previous</span>
            </a>
            {% else %}
            <div class="caption-nav__link caption-nav__prev caption-nav__disabled"></div>
            {% endif %}

            {% if next_id %}
            <a href="{{ base_url }}caption/{{ next_id }}.html" class="caption-nav__link caption-nav__next">
                <span>Next &rarr;</span>
                <img src="{{ base_url }}{{ next_thumb }}" alt="Next" class="caption-nav__thumb">
            </a>
            {% else %}
            <div class="caption-nav__link caption-nav__next caption-nav__disabled"></div>
            {% endif %}
        </nav>
    </main>

    <footer class="footer">
        <p>Powered by <a href="https://github.com/anytron/anytron">Anytron</a></p>
    </footer>

    <script src="{{ base_url }}js/bundle.js"></script>
    {% if enable_memes %}
    <script>
        // Initialize meme generator for this page
        if (typeof initMemeGenerator === 'function') {
            initMemeGenerator('{{ base_url | safe }}{{ frame | safe }}');
        }
    </script>
    {% endif %}
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(0), "00:00:00");
        assert_eq!(format_timestamp(1000), "00:00:01");
        assert_eq!(format_timestamp(61000), "00:01:01");
        assert_eq!(format_timestamp(3661000), "01:01:01");
    }
}
