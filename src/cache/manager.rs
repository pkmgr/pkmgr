use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::cache::{CacheConfig, CacheEntry, CacheType, CacheStats, format_size};
use crate::ui::output::Output;

pub struct CacheManager {
    pub config: CacheConfig,
    pub output: Output,
    pub index: HashMap<String, CacheEntry>,
}

impl CacheManager {
    pub fn new(output: Output) -> Result<Self> {
        let config = CacheConfig::load()?;
        let mut manager = Self {
            config,
            output,
            index: HashMap::new(),
        };
        manager.load_index()?;
        Ok(manager)
    }

    /// Load cache index from disk
    fn load_index(&mut self) -> Result<()> {
        let index_path = self.config.base_dir.join("cache_index.json");

        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            self.index = serde_json::from_str(&content)
                .unwrap_or_else(|_| HashMap::new());
        }

        // Scan cache directories to update index
        self.scan_cache_directories()?;

        Ok(())
    }

    /// Save cache index to disk
    fn save_index(&self) -> Result<()> {
        let index_path = self.config.base_dir.join("cache_index.json");
        let content = serde_json::to_string_pretty(&self.index)?;
        fs::write(index_path, content)?;
        Ok(())
    }

    /// Scan cache directories and update index
    fn scan_cache_directories(&mut self) -> Result<()> {
        for cache_type in &[
            CacheType::PackageMetadata,
            CacheType::PackageDownload,
            CacheType::RepositoryIndex,
            CacheType::BinaryDownload,
            CacheType::IsoDownload,
            CacheType::LanguageVersion,
            CacheType::BuildArtifact,
            CacheType::Temporary,
        ] {
            let cache_dir = self.config.get_cache_dir(cache_type);
            if cache_dir.exists() {
                self.scan_directory(&cache_dir, cache_type.clone())?;
            }
        }
        Ok(())
    }

    /// Scan a specific directory
    fn scan_directory(&mut self, dir: &Path, cache_type: CacheType) -> Result<()> {
        for entry in WalkDir::new(dir)
            .min_depth(1)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                let key = path.strip_prefix(&self.config.base_dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();

                // Check if we already have this in index
                if !self.index.contains_key(&key) {
                    let metadata = entry.metadata()?;
                    let size = metadata.len();

                    let created = metadata.created()
                        .map(|t| DateTime::<Utc>::from(t))
                        .unwrap_or_else(|_| Utc::now());

                    let cache_entry = CacheEntry {
                        key: key.clone(),
                        path: path.to_path_buf(),
                        size,
                        created,
                        last_accessed: created,
                        access_count: 0,
                        cache_type: cache_type.clone(),
                        ttl_seconds: cache_type.default_ttl(),
                    };

                    self.index.insert(key, cache_entry);
                }
            }
        }
        Ok(())
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> Result<CacheStats> {
        let mut stats = CacheStats::default();
        let mut type_stats: HashMap<CacheType, (u64, usize)> = HashMap::new();

        for entry in self.index.values() {
            stats.total_size += entry.size;
            stats.total_entries += 1;

            if entry.is_expired() {
                stats.expired_entries += 1;
            }

            if entry.is_stale(self.config.stale_days) {
                stats.stale_entries += 1;
            }

            let type_stat = type_stats.entry(entry.cache_type.clone()).or_default();
            type_stat.0 += entry.size;
            type_stat.1 += 1;
        }

        // Convert type stats to vector
        stats.by_type = type_stats
            .into_iter()
            .map(|(k, (size, count))| (k, size, count))
            .collect();

        // Sort by size descending
        stats.by_type.sort_by(|a, b| b.1.cmp(&a.1));

        // Get disk free space
        #[cfg(unix)]
        {
            if let Ok(stat) = fs2::statvfs(&self.config.base_dir) {
                stats.disk_free = stat.available_space();
            }
        }

        // Calculate cache usage percentage
        if self.config.max_size > 0 {
            stats.cache_usage_percent = (stats.total_size as f32 / self.config.max_size as f32) * 100.0;
        }

        Ok(stats)
    }

    /// List cache contents
    pub fn list(&self) -> Result<()> {
        self.output.section("Cache Contents");

        let stats = self.get_stats()?;

        // Show summary
        self.output.info(&format!("📊 Total size: {} ({} entries)",
            format_size(stats.total_size),
            stats.total_entries
        ));

        if stats.expired_entries > 0 {
            self.output.warn(&format!("⏰ Expired entries: {} (can be cleaned)",
                stats.expired_entries
            ));
        }

        if stats.stale_entries > 0 {
            self.output.info(&format!("📅 Stale entries: {} (not accessed in {} days)",
                stats.stale_entries,
                self.config.stale_days
            ));
        }

        // Show by type
        self.output.section("Cache by Type");
        for (cache_type, size, count) in &stats.by_type {
            if *count > 0 {
                self.output.info(&format!("  {} {}: {} ({} items)",
                    self.get_type_emoji(&cache_type),
                    cache_type.display_name(),
                    format_size(*size),
                    count
                ));
            }
        }

        // Show disk usage
        self.output.section("Disk Usage");
        self.output.info(&format!("💾 Cache limit: {}",
            format_size(self.config.max_size)
        ));
        self.output.info(&format!("📊 Cache usage: {:.1}%",
            stats.cache_usage_percent
        ));

        if stats.disk_free > 0 {
            self.output.info(&format!("💿 Disk free: {}",
                format_size(stats.disk_free)
            ));
        }

        // Show largest files
        self.show_largest_files(10)?;

        Ok(())
    }

    /// Show information about cache
    pub fn info(&self) -> Result<()> {
        self.output.section("Cache Information");

        self.output.info(&format!("📁 Cache directory: {}",
            self.config.base_dir.display()
        ));

        self.output.info(&format!("⚙️  Configuration:"));
        self.output.info(&format!("   • Max size: {}",
            format_size(self.config.max_size)
        ));
        self.output.info(&format!("   • Cleanup threshold: {:.0}%",
            self.config.cleanup_threshold * 100.0
        ));
        self.output.info(&format!("   • Min free space: {}",
            format_size(self.config.min_free_space)
        ));
        self.output.info(&format!("   • Auto cleanup: {}",
            if self.config.auto_cleanup { "enabled" } else { "disabled" }
        ));
        self.output.info(&format!("   • Stale after: {} days",
            self.config.stale_days
        ));

        // Show TTL settings
        self.output.section("Cache TTL Settings");
        for cache_type in &[
            CacheType::PackageMetadata,
            CacheType::RepositoryIndex,
            CacheType::PackageDownload,
            CacheType::BinaryDownload,
            CacheType::IsoDownload,
            CacheType::LanguageVersion,
            CacheType::BuildArtifact,
            CacheType::Temporary,
        ] {
            let ttl = cache_type.default_ttl()
                .map(|s| format!("{} hours", s / 3600))
                .unwrap_or_else(|| "Never expires".to_string());

            self.output.info(&format!("   • {}: {}",
                cache_type.display_name(),
                ttl
            ));
        }

        // Show cleanup priority
        self.output.section("Cleanup Priority");
        self.output.info("Items are cleaned in this order when space is needed:");

        let mut types: Vec<_> = vec![
            CacheType::Temporary,
            CacheType::BuildArtifact,
            CacheType::RepositoryIndex,
            CacheType::PackageMetadata,
            CacheType::PackageDownload,
            CacheType::BinaryDownload,
            CacheType::LanguageVersion,
            CacheType::IsoDownload,
        ];
        types.sort_by_key(|t| std::cmp::Reverse(t.cleanup_priority()));

        for (i, cache_type) in types.iter().enumerate() {
            self.output.info(&format!("   {}. {}",
                i + 1,
                cache_type.display_name()
            ));
        }

        Ok(())
    }

    /// Refresh cache metadata
    pub fn refresh(&mut self) -> Result<()> {
        self.output.progress("🔄 Refreshing cache metadata...");

        // Clear and rebuild index
        self.index.clear();
        self.scan_cache_directories()?;
        self.save_index()?;

        let stats = self.get_stats()?;
        self.output.success(&format!("✅ Cache refreshed: {} entries, {}",
            stats.total_entries,
            format_size(stats.total_size)
        ));

        Ok(())
    }

    /// Add entry to cache
    pub fn add_entry(&mut self, key: String, path: PathBuf, cache_type: CacheType) -> Result<()> {
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();

        let entry = CacheEntry {
            key: key.clone(),
            path,
            size,
            created: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 1,
            cache_type: cache_type.clone(),
            ttl_seconds: cache_type.default_ttl(),
        };

        self.index.insert(key, entry);
        self.save_index()?;

        // Check if auto-cleanup needed
        if self.config.auto_cleanup {
            let stats = self.get_stats()?;
            if stats.needs_cleanup(&self.config) {
                self.output.info("🧹 Cache threshold reached, starting automatic cleanup...");
                // Trigger cleanup (would call cleaner module)
            }
        }

        Ok(())
    }

    /// Get entry from cache
    pub fn get_entry(&mut self, key: &str) -> Option<&mut CacheEntry> {
        if let Some(entry) = self.index.get_mut(key) {
            entry.touch();
            // Save index after the borrow ends
            drop(entry);
            let _ = self.save_index();
            self.index.get_mut(key)
        } else {
            None
        }
    }

    /// Remove entry from cache
    pub fn remove_entry(&mut self, key: &str) -> Result<bool> {
        if let Some(entry) = self.index.remove(key) {
            if entry.path.exists() {
                fs::remove_file(&entry.path)?;
            }
            self.save_index()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Show largest files in cache
    fn show_largest_files(&self, limit: usize) -> Result<()> {
        let mut entries: Vec<_> = self.index.values().collect();
        entries.sort_by(|a, b| b.size.cmp(&a.size));

        if !entries.is_empty() {
            self.output.section("Largest Cache Files");
            for (i, entry) in entries.iter().take(limit).enumerate() {
                let age = Utc::now() - entry.last_accessed;
                let age_str = if age.num_days() > 0 {
                    format!("{} days ago", age.num_days())
                } else if age.num_hours() > 0 {
                    format!("{} hours ago", age.num_hours())
                } else {
                    "recently".to_string()
                };

                self.output.info(&format!("  {}. {} - {} (accessed {})",
                    i + 1,
                    format_size(entry.size),
                    entry.path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown"),
                    age_str
                ));
            }
        }

        Ok(())
    }

    /// Get emoji for cache type
    fn get_type_emoji(&self, cache_type: &CacheType) -> &'static str {
        match cache_type {
            CacheType::PackageMetadata => "📋",
            CacheType::PackageDownload => "📦",
            CacheType::RepositoryIndex => "📚",
            CacheType::BinaryDownload => "⚙️",
            CacheType::IsoDownload => "💿",
            CacheType::LanguageVersion => "🔤",
            CacheType::BuildArtifact => "🏗️",
            CacheType::Temporary => "⏱️",
        }
    }
}