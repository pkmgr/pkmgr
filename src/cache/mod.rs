pub mod manager;
pub mod cleaner;
pub mod stats;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Cache entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub path: PathBuf,
    pub size: u64,
    pub created: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u32,
    pub cache_type: CacheType,
    pub ttl_seconds: Option<i64>,
}

impl CacheEntry {
    /// Check if cache entry is expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_seconds {
            let age = Utc::now() - self.created;
            age.num_seconds() > ttl
        } else {
            false
        }
    }

    /// Check if cache entry is stale (hasn't been accessed recently)
    pub fn is_stale(&self, days: i64) -> bool {
        let threshold = Utc::now() - Duration::days(days);
        self.last_accessed < threshold
    }

    /// Update access time and count
    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
        self.access_count += 1;
    }
}

/// Types of cached data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheType {
    PackageMetadata,    // Package information from repositories
    PackageDownload,    // Downloaded package files
    RepositoryIndex,    // Repository package lists
    BinaryDownload,     // Downloaded binary releases
    IsoDownload,        // Downloaded ISO files
    LanguageVersion,    // Downloaded language versions
    BuildArtifact,      // Compilation artifacts
    Temporary,          // Temporary files
}

impl CacheType {
    /// Get default TTL for cache type
    pub fn default_ttl(&self) -> Option<i64> {
        match self {
            CacheType::PackageMetadata => Some(86400),     // 1 day
            CacheType::RepositoryIndex => Some(3600),      // 1 hour
            CacheType::PackageDownload => Some(7776000),   // 90 days
            CacheType::BinaryDownload => Some(7776000),    // 90 days
            CacheType::IsoDownload => None,                // Never expire
            CacheType::LanguageVersion => None,            // Never expire
            CacheType::BuildArtifact => Some(604800),      // 7 days
            CacheType::Temporary => Some(86400),           // 1 day
        }
    }

    /// Get cleanup priority (higher = clean first)
    pub fn cleanup_priority(&self) -> u8 {
        match self {
            CacheType::Temporary => 10,
            CacheType::BuildArtifact => 8,
            CacheType::RepositoryIndex => 7,
            CacheType::PackageMetadata => 6,
            CacheType::PackageDownload => 4,
            CacheType::BinaryDownload => 3,
            CacheType::LanguageVersion => 2,
            CacheType::IsoDownload => 1,
        }
    }

    /// Get human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            CacheType::PackageMetadata => "Package Metadata",
            CacheType::PackageDownload => "Package Downloads",
            CacheType::RepositoryIndex => "Repository Indexes",
            CacheType::BinaryDownload => "Binary Downloads",
            CacheType::IsoDownload => "ISO Downloads",
            CacheType::LanguageVersion => "Language Versions",
            CacheType::BuildArtifact => "Build Artifacts",
            CacheType::Temporary => "Temporary Files",
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub base_dir: PathBuf,
    pub max_size: u64,              // Maximum cache size in bytes
    pub cleanup_threshold: f32,     // Start cleanup when cache is this % full (0.8 = 80%)
    pub min_free_space: u64,        // Minimum free disk space to maintain
    pub auto_cleanup: bool,         // Automatically clean when threshold reached
    pub stale_days: i64,            // Consider entries stale after this many days
}

impl Default for CacheConfig {
    fn default() -> Self {
        let base_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("pkmgr");

        Self {
            base_dir,
            max_size: 5 * 1024 * 1024 * 1024,  // 5 GB
            cleanup_threshold: 0.8,             // 80%
            min_free_space: 1024 * 1024 * 1024, // 1 GB
            auto_cleanup: true,
            stale_days: 30,
        }
    }
}

impl CacheConfig {
    /// Load from configuration file
    pub fn load() -> Result<Self> {
        let config_path = dirs::config_dir()
            .context("Failed to determine config directory")?
            .join("pkmgr")
            .join("cache.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content).context("Failed to parse cache config")
        } else {
            Ok(Self::default())
        }
    }

    /// Save to configuration file
    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?
            .join("pkmgr");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("cache.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }

    /// Get cache directory for specific type
    pub fn get_cache_dir(&self, cache_type: &CacheType) -> PathBuf {
        let subdir = match cache_type {
            CacheType::PackageMetadata => "metadata",
            CacheType::PackageDownload => "packages",
            CacheType::RepositoryIndex => "repos",
            CacheType::BinaryDownload => "binaries",
            CacheType::IsoDownload => "isos",
            CacheType::LanguageVersion => "languages",
            CacheType::BuildArtifact => "build",
            CacheType::Temporary => "tmp",
        };

        self.base_dir.join(subdir)
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub total_size: u64,
    pub total_entries: usize,
    pub expired_entries: usize,
    pub stale_entries: usize,
    pub by_type: Vec<(CacheType, u64, usize)>,  // (type, size, count)
    pub disk_free: u64,
    pub cache_usage_percent: f32,
}

impl CacheStats {
    /// Check if cleanup is needed
    pub fn needs_cleanup(&self, config: &CacheConfig) -> bool {
        self.cache_usage_percent > (config.cleanup_threshold * 100.0) ||
        self.disk_free < config.min_free_space ||
        self.expired_entries > 0
    }

    /// Get estimated space that can be freed
    pub fn estimated_cleanup_size(&self) -> u64 {
        let mut size = 0u64;

        // Add expired and stale entries
        for (cache_type, type_size, _) in &self.by_type {
            if cache_type == &CacheType::Temporary {
                size += type_size;
            }
        }

        // Estimate 30% of package downloads can be cleaned
        for (cache_type, type_size, _) in &self.by_type {
            if cache_type == &CacheType::PackageDownload {
                size += type_size / 3;
            }
        }

        size
    }
}

/// Get human-readable size
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}