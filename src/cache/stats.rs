use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

use crate::cache::{CacheConfig, CacheEntry, CacheType, CacheStats, format_size};
use crate::ui::output::Output;

pub struct CacheStatistics {
    output: Output,
}

impl CacheStatistics {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Display detailed cache statistics
    pub fn display_stats(&self, stats: &CacheStats, entries: &HashMap<String, CacheEntry>) {
        self.output.section("Cache Statistics");

        // Overall stats
        self.display_overall_stats(stats);

        // Type breakdown
        self.display_type_breakdown(stats);

        // Age distribution
        self.display_age_distribution(entries);

        // Access patterns
        self.display_access_patterns(entries);

        // Recommendations
        self.display_recommendations(stats);
    }

    /// Display overall statistics
    fn display_overall_stats(&self, stats: &CacheStats) {
        self.output.info(&format!("📊 Total Cache Size: {}",
            format_size(stats.total_size)
        ));

        self.output.info(&format!("📁 Total Entries: {}",
            stats.total_entries
        ));

        if stats.expired_entries > 0 {
            self.output.warn(&format!("⏰ Expired Entries: {} ({:.1}%)",
                stats.expired_entries,
                (stats.expired_entries as f32 / stats.total_entries as f32) * 100.0
            ));
        }

        if stats.stale_entries > 0 {
            self.output.info(&format!("📅 Stale Entries: {} ({:.1}%)",
                stats.stale_entries,
                (stats.stale_entries as f32 / stats.total_entries as f32) * 100.0
            ));
        }

        self.output.info(&format!("💾 Cache Usage: {:.1}%",
            stats.cache_usage_percent
        ));

        if stats.disk_free > 0 {
            let disk_emoji = if stats.disk_free < 5_000_000_000 { "⚠️" } else { "✅" };
            self.output.info(&format!("{} Disk Free: {}",
                disk_emoji,
                format_size(stats.disk_free)
            ));
        }
    }

    /// Display breakdown by cache type
    fn display_type_breakdown(&self, stats: &CacheStats) {
        self.output.section("Cache by Type");

        // Create a bar chart
        let max_size = stats.by_type
            .iter()
            .map(|(_, size, _)| *size)
            .max()
            .unwrap_or(0);

        for (cache_type, size, count) in &stats.by_type {
            if *count > 0 {
                let bar_width = 30;
                let filled = if max_size > 0 {
                    ((size * bar_width as u64) / max_size) as usize
                } else {
                    0
                };

                let bar = format!("{}{}",
                    "█".repeat(filled),
                    "░".repeat(bar_width - filled)
                );

                self.output.info(&format!("{} {} {} ({} items)",
                    self.get_type_emoji(cache_type),
                    bar,
                    format_size(*size),
                    count
                ));
            }
        }
    }

    /// Display age distribution
    fn display_age_distribution(&self, entries: &HashMap<String, CacheEntry>) {
        self.output.section("Age Distribution");

        let now = Utc::now();
        let mut age_buckets = vec![
            ("< 1 hour", 0usize, 0u64),
            ("1-24 hours", 0usize, 0u64),
            ("1-7 days", 0usize, 0u64),
            ("1-4 weeks", 0usize, 0u64),
            ("1-3 months", 0usize, 0u64),
            ("> 3 months", 0usize, 0u64),
        ];

        for entry in entries.values() {
            let age = now - entry.last_accessed;
            let bucket_idx = if age < Duration::hours(1) {
                0
            } else if age < Duration::days(1) {
                1
            } else if age < Duration::days(7) {
                2
            } else if age < Duration::weeks(4) {
                3
            } else if age < Duration::days(90) {
                4
            } else {
                5
            };

            age_buckets[bucket_idx].1 += 1;
            age_buckets[bucket_idx].2 += entry.size;
        }

        for (label, count, size) in age_buckets {
            if count > 0 {
                self.output.info(&format!("  {}: {} entries ({})",
                    label,
                    count,
                    format_size(size)
                ));
            }
        }
    }

    /// Display access patterns
    fn display_access_patterns(&self, entries: &HashMap<String, CacheEntry>) {
        self.output.section("Access Patterns");

        // Find most and least accessed
        let most_accessed = entries.values()
            .max_by_key(|e| e.access_count);

        let least_accessed = entries.values()
            .filter(|e| e.access_count > 0)
            .min_by_key(|e| e.access_count);

        if let Some(entry) = most_accessed {
            self.output.info(&format!("🔥 Most accessed: {} ({} times)",
                entry.path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown"),
                entry.access_count
            ));
        }

        if let Some(entry) = least_accessed {
            self.output.info(&format!("❄️  Least accessed: {} ({} times)",
                entry.path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown"),
                entry.access_count
            ));
        }

        // Average access count
        let total_accesses: u32 = entries.values()
            .map(|e| e.access_count)
            .sum();

        if !entries.is_empty() {
            let avg_accesses = total_accesses as f32 / entries.len() as f32;
            self.output.info(&format!("📊 Average accesses: {:.1}", avg_accesses));
        }
    }

    /// Display recommendations
    fn display_recommendations(&self, stats: &CacheStats) {
        self.output.section("Recommendations");

        let mut recommendations = Vec::new();

        // Check if cleanup needed
        if stats.cache_usage_percent > 80.0 {
            recommendations.push("⚠️  Cache usage is high. Consider running 'pkmgr cache clean'");
        }

        if stats.expired_entries > 10 {
            recommendations.push("⏰ Many expired entries. Run 'pkmgr cache clean --expired'");
        }

        if stats.stale_entries > stats.total_entries / 2 {
            recommendations.push("📅 Many stale entries. Run 'pkmgr cache clean --stale'");
        }

        if stats.disk_free < 5_000_000_000 {
            recommendations.push("💾 Low disk space. Clean cache to free space");
        }

        // Check for inefficient cache usage
        for (cache_type, size, count) in &stats.by_type {
            if *count > 0 {
                let avg_size = size / *count as u64;
                if avg_size < 1024 && cache_type != &CacheType::PackageMetadata {
                    recommendations.push("🔍 Many small files in cache. Consider consolidation");
                    break;
                }
            }
        }

        if recommendations.is_empty() {
            self.output.success("✅ Cache is healthy and well-maintained");
        } else {
            for recommendation in recommendations {
                self.output.info(recommendation);
            }
        }
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

    /// Generate cache report
    pub fn generate_report(&self, stats: &CacheStats, entries: &HashMap<String, CacheEntry>) -> String {
        let mut report = String::new();

        report.push_str("# pkmgr Cache Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total Size: {}\n", format_size(stats.total_size)));
        report.push_str(&format!("- Total Entries: {}\n", stats.total_entries));
        report.push_str(&format!("- Cache Usage: {:.1}%\n", stats.cache_usage_percent));
        report.push_str(&format!("- Disk Free: {}\n", format_size(stats.disk_free)));

        report.push_str("\n## Cache by Type\n\n");
        report.push_str("| Type | Size | Count | Avg Size |\n");
        report.push_str("|------|------|-------|----------|\n");

        for (cache_type, size, count) in &stats.by_type {
            if *count > 0 {
                let avg_size = size / *count as u64;
                report.push_str(&format!("| {} | {} | {} | {} |\n",
                    cache_type.display_name(),
                    format_size(*size),
                    count,
                    format_size(avg_size)
                ));
            }
        }

        report.push_str("\n## Largest Files\n\n");
        let mut entries_vec: Vec<_> = entries.values().collect();
        entries_vec.sort_by(|a, b| b.size.cmp(&a.size));

        for (i, entry) in entries_vec.iter().take(20).enumerate() {
            report.push_str(&format!("{}. {} - {}\n",
                i + 1,
                format_size(entry.size),
                entry.path.display()
            ));
        }

        report
    }
}