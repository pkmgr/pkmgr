use anyhow::Result;
use std::collections::HashMap;
use async_trait::async_trait;

/// Package information structure
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub size: Option<u64>,
    pub installed: bool,
    pub source: String,
}

/// Search result structure
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub packages: Vec<PackageInfo>,
    pub total_count: usize,
}

/// Installation result
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub success: bool,
    pub message: String,
    pub packages_installed: Vec<String>,
}

/// Core trait for package managers
#[async_trait]
pub trait PackageManager: Send + Sync {
    /// Get the name of this package manager
    fn name(&self) -> &str;

    /// Check if this package manager is available on the system
    async fn is_available(&self) -> bool;

    /// Search for packages
    async fn search(&self, query: &str) -> Result<SearchResult>;

    /// Install packages
    async fn install(&self, packages: &[String]) -> Result<InstallResult>;

    /// Remove packages
    async fn remove(&self, packages: &[String]) -> Result<InstallResult>;

    /// Update package lists
    async fn update(&self) -> Result<()>;

    /// Upgrade packages
    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult>;

    /// List installed packages
    async fn list_installed(&self) -> Result<Vec<PackageInfo>>;

    /// Get package info
    async fn info(&self, package: &str) -> Result<Option<PackageInfo>>;

    /// Check if packages are installed
    async fn is_installed(&self, packages: &[String]) -> Result<HashMap<String, bool>>;
}

/// Trait for language version managers
#[async_trait]
pub trait LanguageManager: Send + Sync {
    /// Get the language name
    fn language(&self) -> &str;

    /// List available versions for installation
    async fn list_available(&self) -> Result<Vec<String>>;

    /// List installed versions
    async fn list_installed(&self) -> Result<Vec<String>>;

    /// Install a specific version
    async fn install_version(&self, version: &str) -> Result<()>;

    /// Remove a version
    async fn remove_version(&self, version: &str) -> Result<()>;

    /// Set the current/default version
    async fn set_current(&self, version: &str) -> Result<()>;

    /// Get the current version
    async fn get_current(&self) -> Result<Option<String>>;

    /// Install packages for the current version
    async fn install_packages(&self, packages: &[String]) -> Result<InstallResult>;

    /// Remove packages for the current version
    async fn remove_packages(&self, packages: &[String]) -> Result<InstallResult>;

    /// Search for packages in the language ecosystem
    async fn search_packages(&self, query: &str) -> Result<SearchResult>;
}

/// Trait for binary managers (GitHub releases, etc.)
#[async_trait]
pub trait BinaryManager: Send + Sync {
    /// Search for binary releases
    async fn search(&self, query: &str) -> Result<SearchResult>;

    /// Install binary from repository
    async fn install_from_repo(&self, repo: &str, version: Option<&str>) -> Result<InstallResult>;

    /// Install binary from URL
    async fn install_from_url(&self, url: &str, name: &str) -> Result<InstallResult>;

    /// List installed binaries
    async fn list_installed(&self) -> Result<Vec<PackageInfo>>;

    /// Remove binary
    async fn remove(&self, name: &str) -> Result<InstallResult>;

    /// Update binaries
    async fn update(&self, names: Option<&[String]>) -> Result<InstallResult>;

    /// Get info about a repository
    async fn repo_info(&self, repo: &str) -> Result<Option<PackageInfo>>;
}