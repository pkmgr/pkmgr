pub mod checker;
pub mod diagnostics;
pub mod report;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health check severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Ok,       // Everything is fine
    Info,     // Informational, no action needed
    Warning,  // Potential issue, should investigate
    Error,    // Problem that needs fixing
    Critical, // Severe problem affecting functionality
}

impl Severity {
    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Ok => "✅",
            Severity::Info => "ℹ️",
            Severity::Warning => "⚠️",
            Severity::Error => "❌",
            Severity::Critical => "🔴",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Severity::Ok => "green",
            Severity::Info => "blue",
            Severity::Warning => "yellow",
            Severity::Error => "red",
            Severity::Critical => "red bold",
        }
    }
}

/// Health check finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub category: String,
    pub name: String,
    pub severity: Severity,
    pub message: String,
    pub details: Option<String>,
    pub fix_hint: Option<String>,
    pub auto_fixable: bool,
}

impl Finding {
    pub fn new(category: impl Into<String>, name: impl Into<String>, severity: Severity, message: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            name: name.into(),
            severity,
            message: message.into(),
            details: None,
            fix_hint: None,
            auto_fixable: false,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn with_fix(mut self, hint: impl Into<String>, auto_fixable: bool) -> Self {
        self.fix_hint = Some(hint.into());
        self.auto_fixable = auto_fixable;
        self
    }
}

/// Health check category
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckCategory {
    System,
    Packages,
    Languages,
    Network,
    Storage,
    Security,
    Configuration,
    Cache,
    Repository,
    USB,
    Binary,
    Shell,
}

impl CheckCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            CheckCategory::System => "System",
            CheckCategory::Packages => "Package Management",
            CheckCategory::Languages => "Language Versions",
            CheckCategory::Network => "Network",
            CheckCategory::Storage => "Storage",
            CheckCategory::Security => "Security",
            CheckCategory::Configuration => "Configuration",
            CheckCategory::Cache => "Cache",
            CheckCategory::Repository => "Repositories",
            CheckCategory::USB => "USB Devices",
            CheckCategory::Binary => "Binary Tools",
            CheckCategory::Shell => "Shell Integration",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            CheckCategory::System => "🖥️",
            CheckCategory::Packages => "📦",
            CheckCategory::Languages => "🔤",
            CheckCategory::Network => "🌐",
            CheckCategory::Storage => "💾",
            CheckCategory::Security => "🔐",
            CheckCategory::Configuration => "⚙️",
            CheckCategory::Cache => "💿",
            CheckCategory::Repository => "🗄️",
            CheckCategory::USB => "💾",
            CheckCategory::Binary => "⚙️",
            CheckCategory::Shell => "🐚",
        }
    }
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system_info: SystemInfo,
    pub findings: Vec<Finding>,
    pub stats: HealthStats,
    pub recommendations: Vec<String>,
}

impl HealthReport {
    pub fn new(system_info: SystemInfo) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            system_info,
            findings: Vec::new(),
            stats: HealthStats::default(),
            recommendations: Vec::new(),
        }
    }

    pub fn add_finding(&mut self, finding: Finding) {
        // Update stats
        match finding.severity {
            Severity::Ok => self.stats.ok_count += 1,
            Severity::Info => self.stats.info_count += 1,
            Severity::Warning => self.stats.warning_count += 1,
            Severity::Error => self.stats.error_count += 1,
            Severity::Critical => self.stats.critical_count += 1,
        }

        if finding.auto_fixable {
            self.stats.fixable_count += 1;
        }

        self.findings.push(finding);
    }

    pub fn overall_status(&self) -> Severity {
        if self.stats.critical_count > 0 {
            Severity::Critical
        } else if self.stats.error_count > 0 {
            Severity::Error
        } else if self.stats.warning_count > 0 {
            Severity::Warning
        } else if self.stats.info_count > 0 {
            Severity::Info
        } else {
            Severity::Ok
        }
    }

    pub fn generate_recommendations(&mut self) {
        let mut recommendations = Vec::new();

        // Check for critical issues
        if self.stats.critical_count > 0 {
            recommendations.push(format!(
                "🔴 {} critical issues found - immediate attention required",
                self.stats.critical_count
            ));
        }

        // Check for errors
        if self.stats.error_count > 0 {
            recommendations.push(format!(
                "❌ {} errors found - should be fixed soon",
                self.stats.error_count
            ));
        }

        // Check for fixable issues
        if self.stats.fixable_count > 0 {
            recommendations.push(format!(
                "🔧 {} issues can be auto-fixed with 'pkmgr doctor --fix'",
                self.stats.fixable_count
            ));
        }

        // Storage recommendations
        let storage_issues: Vec<_> = self.findings.iter()
            .filter(|f| f.category == "Storage" && f.severity >= Severity::Warning)
            .collect();

        if !storage_issues.is_empty() {
            recommendations.push("💾 Consider cleaning cache with 'pkmgr cache clean'".to_string());
        }

        // Security recommendations
        let security_issues: Vec<_> = self.findings.iter()
            .filter(|f| f.category == "Security" && f.severity >= Severity::Warning)
            .collect();

        if !security_issues.is_empty() {
            recommendations.push("🔐 Security issues detected - review and update keys/certificates".to_string());
        }

        // Update recommendations
        let package_updates: Vec<_> = self.findings.iter()
            .filter(|f| f.name.contains("updates available"))
            .collect();

        if !package_updates.is_empty() {
            recommendations.push("📦 Package updates available - run 'pkmgr update all'".to_string());
        }

        self.recommendations = recommendations;
    }
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub distribution: String,
    pub version: String,
    pub kernel: String,
    pub architecture: String,
    pub hostname: String,
    pub uptime: String,
    pub cpu_count: usize,
    pub memory_total: u64,
    pub memory_available: u64,
    pub disk_total: u64,
    pub disk_available: u64,
    pub pkmgr_version: String,
}

impl SystemInfo {
    pub fn gather() -> Result<Self> {
        let platform = crate::core::platform::Platform::detect()?;

        // Get system info
        let hostname = hostname::get()?
            .to_string_lossy()
            .to_string();

        // Get uptime
        #[cfg(unix)]
        let uptime = {
            let info = sys_info::loadavg()?;
            format_uptime(sys_info::boottime()?.tv_sec as u64)
        };
        #[cfg(not(unix))]
        let uptime = "unknown".to_string();

        // Get CPU count
        let cpu_count = num_cpus::get();

        // Get memory info
        let (memory_total, memory_available) = get_memory_info()?;

        // Get disk info
        let (disk_total, disk_available) = get_disk_info("/")?;

        Ok(Self {
            os: platform.platform.to_string(),
            distribution: platform.distribution.unwrap_or_else(|| "Unknown".to_string()),
            version: platform.version.unwrap_or_else(|| "Unknown".to_string()),
            kernel: "Unknown".to_string(), // TODO: Add kernel detection
            architecture: platform.architecture.to_string(),
            hostname,
            uptime,
            cpu_count,
            memory_total,
            memory_available,
            disk_total,
            disk_available,
            pkmgr_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

/// Health check statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthStats {
    pub ok_count: usize,
    pub info_count: usize,
    pub warning_count: usize,
    pub error_count: usize,
    pub critical_count: usize,
    pub fixable_count: usize,
}

impl HealthStats {
    pub fn total(&self) -> usize {
        self.ok_count + self.info_count + self.warning_count +
        self.error_count + self.critical_count
    }

    pub fn has_issues(&self) -> bool {
        self.warning_count > 0 || self.error_count > 0 || self.critical_count > 0
    }
}

/// Format uptime from seconds
fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours", days, hours)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    }
}

/// Get memory information
fn get_memory_info() -> Result<(u64, u64)> {
    #[cfg(unix)]
    {
        let mem_info = sys_info::mem_info()?;
        Ok((mem_info.total * 1024, mem_info.avail * 1024))
    }
    #[cfg(not(unix))]
    {
        Ok((0, 0))
    }
}

/// Get disk information
fn get_disk_info(path: &str) -> Result<(u64, u64)> {
    #[cfg(unix)]
    {
        let stat = fs2::statvfs(path)?;
        Ok((stat.total_space(), stat.available_space()))
    }
    #[cfg(not(unix))]
    {
        Ok((0, 0))
    }
}