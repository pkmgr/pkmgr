use anyhow::{Context, Result, bail};
use std::path::Path;
use std::fs;
use crate::ui::output::Output;
use super::Profile;

pub struct ProfileImporter {
    output: Output,
}

impl ProfileImporter {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Import a profile from a file or URL
    pub async fn import(&self, source: &str, name: Option<String>) -> Result<()> {
        self.output.progress(&format!("Importing profile from {}", source));

        let profile = if source.starts_with("http://") || source.starts_with("https://") {
            self.import_from_url(source).await?
        } else {
            self.import_from_file(Path::new(source))?
        };

        // Override name if specified
        let mut profile = profile;
        if let Some(name) = name {
            profile.name = name;
        }

        // Check if profile already exists
        if Profile::list_all()?.contains(&profile.name) {
            bail!("Profile '{}' already exists", profile.name);
        }

        // Save the profile
        profile.save()?;

        self.output.success(&format!("Profile '{}' imported successfully", profile.name));

        Ok(())
    }

    /// Import from a local file
    fn import_from_file(&self, path: &Path) -> Result<Profile> {
        if !path.exists() {
            bail!("File not found: {}", path.display());
        }

        let content = fs::read_to_string(path)
            .context("Failed to read profile file")?;

        // Try to detect format and parse
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            self.parse_json(&content)
        } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
                  path.extension().and_then(|s| s.to_str()) == Some("yml") {
            self.parse_yaml(&content)
        } else {
            // Default to TOML
            self.parse_toml(&content)
        }
    }

    /// Import from a URL
    async fn import_from_url(&self, url: &str) -> Result<Profile> {
        self.output.progress("Downloading profile...");

        let client = reqwest::Client::new();
        let response = client.get(url)
            .send()
            .await
            .context("Failed to download profile")?;

        if !response.status().is_success() {
            bail!("Failed to download profile: HTTP {}", response.status());
        }

        let content = response.text()
            .await
            .context("Failed to read response")?;

        // Try to detect format from URL or content
        if url.ends_with(".json") {
            self.parse_json(&content)
        } else if url.ends_with(".yaml") || url.ends_with(".yml") {
            self.parse_yaml(&content)
        } else {
            // Try TOML first, then JSON
            self.parse_toml(&content)
                .or_else(|_| self.parse_json(&content))
        }
    }

    /// Parse TOML format
    fn parse_toml(&self, content: &str) -> Result<Profile> {
        toml::from_str(content)
            .context("Failed to parse profile as TOML")
    }

    /// Parse JSON format
    fn parse_json(&self, content: &str) -> Result<Profile> {
        serde_json::from_str(content)
            .context("Failed to parse profile as JSON")
    }

    /// Parse YAML format
    fn parse_yaml(&self, content: &str) -> Result<Profile> {
        // Would need serde_yaml dependency
        self.output.warn("YAML import not yet implemented");
        bail!("YAML format not supported yet")
    }
}