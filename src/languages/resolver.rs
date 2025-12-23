use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json::Value;
use crate::ui::output::Output;

/// Version resolution priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionSource {
    CommandLineOverride,
    CurrentDirectoryFile,
    ParentDirectoryFile,
    ProjectManifest,
    UserDefault,
    SystemDefault,
    SystemInstalled,
}

/// Resolved version information
#[derive(Debug, Clone)]
pub struct ResolvedVersion {
    pub version: String,
    pub source: VersionSource,
    pub path: PathBuf,
    pub description: String,
}

/// Language version resolver
pub struct VersionResolver {
    language: String,
    output: Output,
}

impl VersionResolver {
    pub fn new(language: String, output: Output) -> Self {
        Self { language, output }
    }

    /// Resolve version using priority order from CLAUDE.md specification
    pub async fn resolve_version(&self, override_version: Option<String>) -> Result<ResolvedVersion> {
        // 1. Command line override (--version flag)
        if let Some(version) = override_version {
            if let Some(resolved) = self.find_installed_version(&version).await? {
                return Ok(ResolvedVersion {
                    version: version.clone(),
                    source: VersionSource::CommandLineOverride,
                    path: resolved,
                    description: format!("Command line override: {}", version),
                });
            } else {
                bail!("Specified version {} not found for {}", version, self.language);
            }
        }

        // 2. Current directory version file
        if let Some(version) = self.check_current_directory_version()? {
            if let Some(resolved) = self.find_installed_version(&version).await? {
                return Ok(ResolvedVersion {
                    version: version.clone(),
                    source: VersionSource::CurrentDirectoryFile,
                    path: resolved,
                    description: format!("Current directory version file: {}", version),
                });
            }
        }

        // 3. Parent directory search (up to VCS root or 5 levels)
        if let Some((version, level)) = self.search_parent_directories()? {
            if let Some(resolved) = self.find_installed_version(&version).await? {
                return Ok(ResolvedVersion {
                    version: version.clone(),
                    source: VersionSource::ParentDirectoryFile,
                    path: resolved,
                    description: format!("Parent directory (level {}): {}", level, version),
                });
            }
        }

        // 4. Project manifest file
        if let Some(version) = self.check_project_manifest()? {
            if let Some(resolved) = self.find_installed_version(&version).await? {
                return Ok(ResolvedVersion {
                    version: version.clone(),
                    source: VersionSource::ProjectManifest,
                    path: resolved,
                    description: format!("Project manifest: {}", version),
                });
            }
        }

        // 5. User default
        if let Some(version) = self.get_user_default()? {
            if let Some(resolved) = self.find_installed_version(&version).await? {
                return Ok(ResolvedVersion {
                    version: version.clone(),
                    source: VersionSource::UserDefault,
                    path: resolved,
                    description: format!("User default: {}", version),
                });
            }
        }

        // 6. System default
        if let Some(version) = self.get_system_default()? {
            if let Some(resolved) = self.find_installed_version(&version).await? {
                return Ok(ResolvedVersion {
                    version: version.clone(),
                    source: VersionSource::SystemDefault,
                    path: resolved,
                    description: format!("System default: {}", version),
                });
            }
        }

        // 7. System installed version
        if let Some(resolved) = self.find_system_version().await? {
            return Ok(ResolvedVersion {
                version: "system".to_string(),
                source: VersionSource::SystemInstalled,
                path: resolved,
                description: "System installed version".to_string(),
            });
        }

        // 8. Prompt to install if TTY, error if non-interactive
        if atty::is(atty::Stream::Stdout) {
            bail!("{} not found. Run 'pkmgr {} install <version>' to install a version",
                  self.language, self.language);
        } else {
            bail!("{} not found and running in non-interactive mode", self.language);
        }
    }

    /// Check for version files in current directory
    fn check_current_directory_version(&self) -> Result<Option<String>> {
        let current_dir = env::current_dir()?;
        self.check_version_file_in_dir(&current_dir)
    }

    /// Search parent directories up to VCS root or 5 levels
    fn search_parent_directories(&self) -> Result<Option<(String, usize)>> {
        let mut current_dir = env::current_dir()?;
        let mut level = 0;

        while level < 5 {
            // Move to parent directory
            if let Some(parent) = current_dir.parent() {
                current_dir = parent.to_path_buf();
                level += 1;

                // Check if this is a VCS root
                if self.is_vcs_root(&current_dir) {
                    // Check for version file in VCS root and stop
                    if let Some(version) = self.check_version_file_in_dir(&current_dir)? {
                        return Ok(Some((version, level)));
                    }
                    break;
                }

                // Check for version file in this directory
                if let Some(version) = self.check_version_file_in_dir(&current_dir)? {
                    return Ok(Some((version, level)));
                }
            } else {
                break; // Reached filesystem root
            }
        }

        Ok(None)
    }

    /// Check if directory is a VCS root
    fn is_vcs_root(&self, dir: &Path) -> bool {
        dir.join(".git").exists() ||
        dir.join(".hg").exists() ||
        dir.join(".svn").exists() ||
        dir.join(".bzr").exists()
    }

    /// Check for version file in a specific directory
    fn check_version_file_in_dir(&self, dir: &Path) -> Result<Option<String>> {
        let version_files = self.get_version_file_names();

        for file_name in version_files {
            let file_path = dir.join(file_name);
            if file_path.exists() {
                let content = fs::read_to_string(file_path)
                    .context(format!("Failed to read {}", file_name))?;
                let version = content.trim().to_string();
                if !version.is_empty() {
                    return Ok(Some(version));
                }
            }
        }

        Ok(None)
    }

    /// Get version file names for the language
    fn get_version_file_names(&self) -> Vec<&str> {
        match self.language.as_str() {
            "python" => vec![".python-version"],
            "node" => vec![".nvmrc", ".node-version"],
            "ruby" => vec![".ruby-version"],
            "go" => vec![".go-version"],
            "rust" => vec!["rust-toolchain.toml", "rust-toolchain"],
            "php" => vec![".php-version"],
            "java" => vec![".java-version"],
            "dotnet" => vec!["global.json"],
            _ => vec![],
        }
    }

    /// Check project manifest files for version requirements
    fn check_project_manifest(&self) -> Result<Option<String>> {
        let current_dir = env::current_dir()?;

        match self.language.as_str() {
            "node" => self.check_package_json(&current_dir),
            "python" => self.check_python_manifest(&current_dir),
            "ruby" => self.check_gemfile(&current_dir),
            "go" => self.check_go_mod(&current_dir),
            "dotnet" => self.check_csproj(&current_dir),
            _ => Ok(None),
        }
    }

    /// Check package.json engines.node field
    fn check_package_json(&self, dir: &Path) -> Result<Option<String>> {
        let package_json = dir.join("package.json");
        if package_json.exists() {
            let content = fs::read_to_string(package_json)?;
            let json: Value = serde_json::from_str(&content)?;

            if let Some(engines) = json.get("engines") {
                if let Some(node_version) = engines.get("node") {
                    if let Some(version_str) = node_version.as_str() {
                        // Extract version from range like ">=14.0.0" or "^16.0.0"
                        return Ok(Some(self.extract_version_from_range(version_str)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Check Python manifest files (pyproject.toml, setup.py, requirements.txt)
    fn check_python_manifest(&self, dir: &Path) -> Result<Option<String>> {
        // Check pyproject.toml
        let pyproject = dir.join("pyproject.toml");
        if pyproject.exists() {
            let content = fs::read_to_string(pyproject)?;
            // Basic TOML parsing for python_requires
            for line in content.lines() {
                if line.trim().starts_with("python_requires") {
                    if let Some(version) = self.extract_python_version_from_line(line) {
                        return Ok(Some(version));
                    }
                }
            }
        }

        // Could also check setup.py and requirements.txt here
        Ok(None)
    }

    /// Extract version from requirement ranges
    fn extract_version_from_range(&self, range: &str) -> String {
        // Simple extraction - remove operators and get major.minor
        let cleaned = range.trim_start_matches(&['>', '<', '=', '^', '~'][..]);
        if let Some(dot_pos) = cleaned.find('.') {
            if let Some(second_dot) = cleaned[dot_pos+1..].find('.') {
                // Return major.minor from major.minor.patch
                cleaned[..dot_pos + 1 + second_dot].to_string()
            } else {
                // Already major.minor
                cleaned.to_string()
            }
        } else {
            // Just major version, append .0
            format!("{}.0", cleaned)
        }
    }

    /// Extract Python version from pyproject.toml line
    fn extract_python_version_from_line(&self, line: &str) -> Option<String> {
        // Look for patterns like 'python_requires = ">=3.8"'
        if let Some(start) = line.find('"') {
            if let Some(end) = line.rfind('"') {
                let version_spec = &line[start+1..end];
                return Some(self.extract_version_from_range(version_spec));
            }
        }
        None
    }

    /// Placeholder implementations for other manifest checks
    fn check_gemfile(&self, _dir: &Path) -> Result<Option<String>> { Ok(None) }
    fn check_go_mod(&self, _dir: &Path) -> Result<Option<String>> { Ok(None) }
    fn check_csproj(&self, _dir: &Path) -> Result<Option<String>> { Ok(None) }

    /// Get user default version
    fn get_user_default(&self) -> Result<Option<String>> {
        let home_dir = dirs::home_dir().context("Could not find home directory")?;
        let current_file = home_dir
            .join(".local/share/pkmgr/languages")
            .join(&self.language)
            .join("current");

        if current_file.exists() {
            let version = fs::read_to_string(current_file)?.trim().to_string();
            if !version.is_empty() {
                return Ok(Some(version));
            }
        }

        Ok(None)
    }

    /// Get system default version
    fn get_system_default(&self) -> Result<Option<String>> {
        let current_file = PathBuf::from("/usr/local/share/pkmgr/languages")
            .join(&self.language)
            .join("current");

        if current_file.exists() {
            let version = fs::read_to_string(current_file)?.trim().to_string();
            if !version.is_empty() {
                return Ok(Some(version));
            }
        }

        Ok(None)
    }

    /// Find installed version in pkmgr-managed locations
    async fn find_installed_version(&self, version: &str) -> Result<Option<PathBuf>> {
        // Check user installation first
        if let Some(home_dir) = dirs::home_dir() {
            let user_path = home_dir
                .join(".local/share/pkmgr/languages")
                .join(&self.language)
                .join(version);

            if self.check_version_installation(&user_path) {
                return Ok(Some(user_path));
            }
        }

        // Check system installation
        let system_path = PathBuf::from("/usr/local/share/pkmgr/languages")
            .join(&self.language)
            .join(version);

        if self.check_version_installation(&system_path) {
            return Ok(Some(system_path));
        }

        Ok(None)
    }

    /// Check if a version is properly installed
    fn check_version_installation(&self, path: &Path) -> bool {
        if !path.exists() {
            return false;
        }

        // Check for binary executable
        let binary_name = self.get_primary_binary_name();
        let binary_path = path.join("bin").join(binary_name);

        binary_path.exists() && binary_path.is_file()
    }

    /// Find system-installed version
    async fn find_system_version(&self) -> Result<Option<PathBuf>> {
        let binary_name = self.get_primary_binary_name();

        if let Ok(system_path) = which::which(binary_name) {
            return Ok(Some(system_path));
        }

        Ok(None)
    }

    /// Get primary binary name for the language
    fn get_primary_binary_name(&self) -> &str {
        match self.language.as_str() {
            "python" => "python3",
            "node" => "node",
            "ruby" => "ruby",
            "rust" => "rustc",
            "go" => "go",
            "php" => "php",
            "java" => "java",
            "dotnet" => "dotnet",
            _ => &self.language,
        }
    }
}