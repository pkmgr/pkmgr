use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Repository configuration stored by pkmgr
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub repositories: HashMap<String, RepositoryEntry>,
    pub preferences: RepositoryPreferences,
    pub mirrors: HashMap<String, MirrorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryEntry {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
    pub added_date: chrono::DateTime<chrono::Utc>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub auto_added: bool,
    pub package_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryPreferences {
    pub auto_add_repos: bool,
    pub auto_update_keys: bool,
    pub verify_signatures: bool,
    pub prefer_mirrors: bool,
    pub keyserver_timeout: u64,
    pub cache_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorConfig {
    pub original_url: String,
    pub mirror_url: String,
    pub location: String,
    pub priority: u32,
    pub enabled: bool,
}

impl Default for RepositoryConfig {
    fn default() -> Self {
        Self {
            repositories: HashMap::new(),
            preferences: RepositoryPreferences::default(),
            mirrors: HashMap::new(),
        }
    }
}

impl Default for RepositoryPreferences {
    fn default() -> Self {
        Self {
            auto_add_repos: true,
            auto_update_keys: true,
            verify_signatures: true,
            prefer_mirrors: false,
            keyserver_timeout: 30,
            cache_timeout: 3600,
        }
    }
}

impl RepositoryConfig {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read repository config")?;
            toml::from_str(&content)
                .context("Failed to parse repository config")
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize repository config")?;

        fs::write(&config_path, content)
            .context("Failed to write repository config")?;

        Ok(())
    }

    /// Get configuration file path
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        Ok(config_dir.join("pkmgr").join("repos.toml"))
    }

    /// Add a repository entry
    pub fn add_repository(&mut self, name: String, url: String, auto_added: bool) {
        let entry = RepositoryEntry {
            name: name.clone(),
            url,
            enabled: true,
            priority: 100,
            added_date: chrono::Utc::now(),
            last_updated: None,
            auto_added,
            package_count: None,
        };

        self.repositories.insert(name, entry);
    }

    /// Remove a repository entry
    pub fn remove_repository(&mut self, name: &str) -> bool {
        self.repositories.remove(name).is_some()
    }

    /// Get a repository entry
    pub fn get_repository(&self, name: &str) -> Option<&RepositoryEntry> {
        self.repositories.get(name)
    }

    /// Update repository metadata
    pub fn update_repository(&mut self, name: &str, package_count: usize) {
        if let Some(entry) = self.repositories.get_mut(name) {
            entry.package_count = Some(package_count);
            entry.last_updated = Some(chrono::Utc::now());
        }
    }

    /// Add a mirror configuration
    pub fn add_mirror(&mut self, name: String, original: String, mirror: String, location: String) {
        let config = MirrorConfig {
            original_url: original,
            mirror_url: mirror,
            location,
            priority: 100,
            enabled: true,
        };

        self.mirrors.insert(name, config);
    }

    /// Get the best mirror for a URL
    pub fn get_best_mirror(&self, url: &str) -> Option<&str> {
        if !self.preferences.prefer_mirrors {
            return None;
        }

        // Find matching mirror
        for mirror in self.mirrors.values() {
            if mirror.enabled && url.contains(&mirror.original_url) {
                return Some(&mirror.mirror_url);
            }
        }

        None
    }

    /// Check if a repository was auto-added
    pub fn is_auto_added(&self, name: &str) -> bool {
        self.repositories.get(name)
            .map(|e| e.auto_added)
            .unwrap_or(false)
    }
}