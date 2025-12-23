use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::cache::{CacheConfig, CacheEntry, CacheType, CacheStats, format_size};
use crate::cache::manager::CacheManager;
use crate::ui::output::Output;
use crate::ui::prompt::Prompt;

pub struct CacheCleaner {
    pub manager: CacheManager,
    pub config: CacheConfig,
    pub output: Output,
    pub prompt: Prompt,
    pub dry_run: bool,
}

impl CacheCleaner {
    pub fn new(output: Output, dry_run: bool) -> Result<Self> {
        let config = CacheConfig::load()?;
        let manager = CacheManager::new(output.clone())?;
        let emoji_enabled = output.emoji_enabled;

        Ok(Self {
            manager,
            config,
            output,
            prompt: Prompt::new(emoji_enabled),
            dry_run,
        })
    }

    /// Clean all caches
    pub async fn clean_all(&mut self, force: bool) -> Result<()> {
        self.output.section("Cache Cleanup");

        let stats = self.manager.get_stats()?;

        if stats.total_entries == 0 {
            self.output.success("✅ Cache is already empty");
            return Ok(());
        }

        self.output.info(&format!("📊 Current cache size: {} ({} entries)",
            format_size(stats.total_size),
            stats.total_entries
        ));

        let estimated_cleanup = stats.estimated_cleanup_size();
        self.output.info(&format!("🧹 Estimated cleanup: {}",
            format_size(estimated_cleanup)
        ));

        if !force && !self.dry_run {
            if !self.prompt.confirm("Clean all cache entries?")? {
                self.output.info("Cleanup cancelled");
                return Ok(());
            }
        }

        // Clean by priority
        let mut total_cleaned = 0u64;
        let mut entries_cleaned = 0usize;

        // Group entries by type
        let mut by_type: Vec<(CacheType, Vec<CacheEntry>)> = Vec::new();
        for cache_type in &[
            CacheType::Temporary,
            CacheType::BuildArtifact,
            CacheType::RepositoryIndex,
            CacheType::PackageMetadata,
            CacheType::PackageDownload,
            CacheType::BinaryDownload,
            CacheType::LanguageVersion,
            CacheType::IsoDownload,
        ] {
            let entries: Vec<CacheEntry> = self.manager
                .index
                .values()
                .filter(|e| e.cache_type == *cache_type)
                .cloned()
                .collect();

            if !entries.is_empty() {
                by_type.push((cache_type.clone(), entries));
            }
        }

        // Sort by cleanup priority
        by_type.sort_by_key(|(t, _)| std::cmp::Reverse(t.cleanup_priority()));

        for (cache_type, entries) in by_type {
            let (cleaned_size, cleaned_count) = self.clean_type(&cache_type, entries).await?;
            total_cleaned += cleaned_size;
            entries_cleaned += cleaned_count;
        }

        if self.dry_run {
            self.output.info(&format!("🔍 Dry run - would clean: {} ({} entries)",
                format_size(total_cleaned),
                entries_cleaned
            ));
        } else {
            self.output.success(&format!("✅ Cleaned: {} ({} entries)",
                format_size(total_cleaned),
                entries_cleaned
            ));
        }

        Ok(())
    }

    /// Clean specific cache type
    pub async fn clean_type(&mut self, cache_type: &CacheType, entries: Vec<CacheEntry>) -> Result<(u64, usize)> {
        if entries.is_empty() {
            return Ok((0, 0));
        }

        self.output.progress(&format!("🧹 Cleaning {}...", cache_type.display_name()));

        let mut total_size = 0u64;
        let mut count = 0usize;

        for entry in entries {
            if self.should_clean(&entry) {
                if self.dry_run {
                    self.output.info(&format!("  Would remove: {}",
                        entry.path.display()
                    ));
                } else {
                    if self.remove_entry(&entry).await? {
                        total_size += entry.size;
                        count += 1;
                    }
                }
            }
        }

        Ok((total_size, count))
    }

    /// Clean expired entries only
    pub async fn clean_expired(&mut self) -> Result<()> {
        self.output.section("Cleaning Expired Cache Entries");

        let expired: Vec<CacheEntry> = self.manager
            .index
            .values()
            .filter(|e| e.is_expired())
            .cloned()
            .collect();

        if expired.is_empty() {
            self.output.success("✅ No expired entries found");
            return Ok(());
        }

        self.output.info(&format!("🗑️  Found {} expired entries", expired.len()));

        let mut total_size = 0u64;
        let mut count = 0usize;

        for entry in expired {
            if self.dry_run {
                self.output.info(&format!("  Would remove: {}",
                    entry.path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                ));
                total_size += entry.size;
                count += 1;
            } else {
                if self.remove_entry(&entry).await? {
                    total_size += entry.size;
                    count += 1;
                }
            }
        }

        if self.dry_run {
            self.output.info(&format!("🔍 Dry run - would clean: {} ({} entries)",
                format_size(total_size),
                count
            ));
        } else {
            self.output.success(&format!("✅ Cleaned expired: {} ({} entries)",
                format_size(total_size),
                count
            ));
        }

        Ok(())
    }

    /// Clean stale entries (not accessed recently)
    pub async fn clean_stale(&mut self) -> Result<()> {
        self.output.section("Cleaning Stale Cache Entries");

        let stale: Vec<CacheEntry> = self.manager
            .index
            .values()
            .filter(|e| e.is_stale(self.config.stale_days))
            .cloned()
            .collect();

        if stale.is_empty() {
            self.output.success(&format!("✅ No stale entries (>{} days)",
                self.config.stale_days
            ));
            return Ok(());
        }

        self.output.info(&format!("📅 Found {} stale entries", stale.len()));

        let mut total_size = 0u64;
        let mut count = 0usize;

        for entry in stale {
            if self.dry_run {
                self.output.info(&format!("  Would remove: {}",
                    entry.path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                ));
                total_size += entry.size;
                count += 1;
            } else {
                if self.remove_entry(&entry).await? {
                    total_size += entry.size;
                    count += 1;
                }
            }
        }

        if self.dry_run {
            self.output.info(&format!("🔍 Dry run - would clean: {} ({} entries)",
                format_size(total_size),
                count
            ));
        } else {
            self.output.success(&format!("✅ Cleaned stale: {} ({} entries)",
                format_size(total_size),
                count
            ));
        }

        Ok(())
    }

    /// Smart cleanup to free specific amount of space
    pub async fn clean_to_free(&mut self, required_bytes: u64) -> Result<bool> {
        self.output.progress(&format!("🎯 Freeing up {}...", format_size(required_bytes)));

        let mut freed = 0u64;
        let mut cleaned_count = 0usize;

        // First, clean expired entries
        let expired: Vec<CacheEntry> = self.manager
            .index
            .values()
            .filter(|e| e.is_expired())
            .cloned()
            .collect();

        for entry in expired {
            if freed >= required_bytes {
                break;
            }
            if self.remove_entry(&entry).await? {
                freed += entry.size;
                cleaned_count += 1;
            }
        }

        // If not enough, clean temporary files
        if freed < required_bytes {
            let temp: Vec<CacheEntry> = self.manager
                .index
                .values()
                .filter(|e| e.cache_type == CacheType::Temporary)
                .cloned()
                .collect();

            for entry in temp {
                if freed >= required_bytes {
                    break;
                }
                if self.remove_entry(&entry).await? {
                    freed += entry.size;
                    cleaned_count += 1;
                }
            }
        }

        // If still not enough, clean stale entries
        if freed < required_bytes {
            let mut stale: Vec<CacheEntry> = self.manager
                .index
                .values()
                .filter(|e| e.is_stale(self.config.stale_days))
                .cloned()
                .collect();

            // Sort by last accessed (oldest first)
            stale.sort_by_key(|e| e.last_accessed);

            for entry in stale {
                if freed >= required_bytes {
                    break;
                }
                if self.remove_entry(&entry).await? {
                    freed += entry.size;
                    cleaned_count += 1;
                }
            }
        }

        // If still not enough, clean by priority
        if freed < required_bytes {
            let mut all_entries: Vec<CacheEntry> = self.manager
                .index
                .values()
                .cloned()
                .collect();

            // Sort by cleanup priority and age
            all_entries.sort_by(|a, b| {
                let priority_cmp = b.cache_type.cleanup_priority()
                    .cmp(&a.cache_type.cleanup_priority());
                if priority_cmp == std::cmp::Ordering::Equal {
                    a.last_accessed.cmp(&b.last_accessed)
                } else {
                    priority_cmp
                }
            });

            for entry in all_entries {
                if freed >= required_bytes {
                    break;
                }

                // Skip ISOs and language versions unless desperate
                if entry.cache_type == CacheType::IsoDownload ||
                   entry.cache_type == CacheType::LanguageVersion {
                    continue;
                }

                if self.remove_entry(&entry).await? {
                    freed += entry.size;
                    cleaned_count += 1;
                }
            }
        }

        if freed >= required_bytes {
            self.output.success(&format!("✅ Freed {} by cleaning {} entries",
                format_size(freed),
                cleaned_count
            ));
            Ok(true)
        } else {
            self.output.warn(&format!("⚠️  Could only free {} of {} requested",
                format_size(freed),
                format_size(required_bytes)
            ));
            Ok(false)
        }
    }

    /// Determine if an entry should be cleaned
    fn should_clean(&self, entry: &CacheEntry) -> bool {
        // Always clean expired entries
        if entry.is_expired() {
            return true;
        }

        // Clean based on type priority
        match entry.cache_type {
            CacheType::Temporary => true,  // Always clean temp files
            CacheType::BuildArtifact => entry.is_stale(7),  // Clean if > 7 days
            CacheType::RepositoryIndex => entry.is_expired(),  // Only if expired
            CacheType::PackageMetadata => entry.is_stale(30),  // Clean if > 30 days
            CacheType::PackageDownload => entry.is_stale(90),  // Clean if > 90 days
            CacheType::BinaryDownload => entry.is_stale(90),
            CacheType::LanguageVersion => false,  // Never auto-clean
            CacheType::IsoDownload => false,  // Never auto-clean
        }
    }

    /// Remove a cache entry
    async fn remove_entry(&mut self, entry: &CacheEntry) -> Result<bool> {
        if entry.path.exists() {
            // Remove file or directory
            if entry.path.is_dir() {
                fs::remove_dir_all(&entry.path)?;
            } else {
                fs::remove_file(&entry.path)?;
            }

            // Remove from index
            self.manager.remove_entry(&entry.key)?;

            return Ok(true);
        }

        // Entry doesn't exist on disk, just remove from index
        self.manager.remove_entry(&entry.key)?;
        Ok(false)
    }

    /// Clean orphaned files (exist on disk but not in index)
    pub async fn clean_orphaned(&mut self) -> Result<()> {
        self.output.section("Cleaning Orphaned Files");

        let mut orphaned_size = 0u64;
        let mut orphaned_count = 0usize;

        // Scan cache directories
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
                orphaned_size += self.clean_orphaned_in_dir(&cache_dir, &mut orphaned_count).await?;
            }
        }

        if orphaned_count > 0 {
            if self.dry_run {
                self.output.info(&format!("🔍 Dry run - would clean {} orphaned files ({})",
                    orphaned_count,
                    format_size(orphaned_size)
                ));
            } else {
                self.output.success(&format!("✅ Cleaned {} orphaned files ({})",
                    orphaned_count,
                    format_size(orphaned_size)
                ));
            }
        } else {
            self.output.success("✅ No orphaned files found");
        }

        Ok(())
    }

    /// Clean orphaned files in a directory
    async fn clean_orphaned_in_dir(&self, dir: &Path, count: &mut usize) -> Result<u64> {
        let mut total_size = 0u64;

        for entry in walkdir::WalkDir::new(dir)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let relative = path.strip_prefix(&self.config.base_dir)
                .unwrap_or(path);
            let key = relative.to_string_lossy().to_string();

            // Check if in index
            if !self.manager.index.contains_key(&key) {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();

                    if self.dry_run {
                        self.output.info(&format!("  Would remove orphaned: {}",
                            path.display()
                        ));
                    } else {
                        if path.is_dir() {
                            fs::remove_dir_all(path)?;
                        } else {
                            fs::remove_file(path)?;
                        }
                    }

                    total_size += size;
                    *count += 1;
                }
            }
        }

        Ok(total_size)
    }
}