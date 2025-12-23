use anyhow::{Result, Context};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};
use crate::ui::output::Output;

pub struct WingetManager {
    output: Output,
}

impl WingetManager {
    pub fn new() -> Self {
        Self {
            output: Output::new("auto".to_string(), true),
        }
    }

    /// Check if winget is available and install if needed
    async fn ensure_available(&self) -> Result<bool> {
        if self.is_available().await {
            return Ok(true);
        }

        self.output.info("📦 Winget not found. Installing App Installer...");
        self.install_winget().await
    }

    /// Install winget according to CLAUDE.md specification
    async fn install_winget(&self) -> Result<bool> {
        self.output.info("⏳ Downloading App Installer from Microsoft Store or GitHub...");

        // Try to install via Microsoft Store first
        let store_output = Command::new("powershell")
            .args(&[
                "-Command",
                "Get-AppxPackage -Name Microsoft.DesktopAppInstaller | Select-Object PackageFullName"
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to check App Installer status")?;

        if store_output.status.success() {
            self.output.info("⏳ Installing via Microsoft Store...");

            let install_output = Command::new("powershell")
                .args(&[
                    "-Command",
                    "Add-AppxPackage -RegisterByFamilyName -MainPackage Microsoft.DesktopAppInstaller_8wekyb3d8bbwe"
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
                .context("Failed to install App Installer")?;

            if install_output.status.success() {
                self.output.success("✅ Winget installed successfully via Microsoft Store!");
                return Ok(true);
            }
        }

        // Fallback to GitHub release
        self.output.info("⏳ Downloading from GitHub...");
        let github_url = "https://api.github.com/repos/microsoft/winget-cli/releases/latest";

        // In a real implementation, we would:
        // 1. Fetch the latest release info from GitHub API
        // 2. Download the .msixbundle file
        // 3. Install it using Add-AppxPackage

        self.output.warn("⚠️ Automatic winget installation requires manual intervention");
        self.output.info("Please install the Microsoft App Installer from the Microsoft Store");

        Ok(false)
    }

    /// Execute winget command with proper error handling
    async fn execute_winget(&self, args: &[&str]) -> Result<std::process::Output> {
        Command::new("winget")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute winget command")
    }

    /// Parse winget search output
    fn parse_search_output(&self, output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in output.lines() {
            // Skip header lines and separators
            if line.starts_with("Name") || line.starts_with("---") || line.trim().is_empty() {
                continue;
            }

            // Winget search output is tab-separated or space-separated
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let id = parts[1].to_string();
                let version = parts[2].to_string();
                let description = parts.get(3..).map(|p| p.join(" ")).unwrap_or_default();

                packages.push(PackageInfo {
                    name: id, // Use package ID as name for winget
                    version,
                    description: Some(description),
                    size: None,
                    installed: false,
                    source: "winget".to_string(),
                });
            }
        }

        packages
    }

    /// Parse winget list output for installed packages
    fn parse_list_output(&self, output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in output.lines() {
            // Skip header lines and separators
            if line.starts_with("Name") || line.starts_with("---") || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[0].to_string();
                let id = parts[1].to_string();
                let version = parts[2].to_string();

                packages.push(PackageInfo {
                    name: id,
                    version,
                    description: None,
                    size: None,
                    installed: true,
                    source: "winget".to_string(),
                });
            }
        }

        packages
    }
}

#[async_trait]
impl PackageManager for WingetManager {
    fn name(&self) -> &str {
        "winget"
    }

    async fn is_available(&self) -> bool {
        Command::new("winget")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|status| status.success())
            .unwrap_or(false)
    }

    async fn search(&self, query: &str) -> Result<SearchResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Winget is not available"));
        }

        let output = self.execute_winget(&["search", query]).await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Search failed: {}", error));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages = self.parse_search_output(&stdout);
        let total_count = packages.len();

        Ok(SearchResult { packages, total_count })
    }

    async fn install(&self, packages: &[String]) -> Result<InstallResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Winget is not available"));
        }

        let mut success_count = 0;
        let mut errors = Vec::new();

        for package in packages {
            let output = self.execute_winget(&["install", package, "--silent"]).await?;

            if output.status.success() {
                success_count += 1;
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                errors.push(format!("{}: {}", package, error));
            }
        }

        let success = errors.is_empty();
        let message = if success {
            format!("All {} packages installed successfully", success_count)
        } else {
            format!("{} of {} packages installed. Errors: {}",
                    success_count, packages.len(), errors.join("; "))
        };

        Ok(InstallResult {
            success,
            message,
            packages_installed: packages.to_vec(),
        })
    }

    async fn remove(&self, packages: &[String]) -> Result<InstallResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Winget is not available"));
        }

        let mut success_count = 0;
        let mut errors = Vec::new();

        for package in packages {
            let output = self.execute_winget(&["uninstall", package, "--silent"]).await?;

            if output.status.success() {
                success_count += 1;
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                errors.push(format!("{}: {}", package, error));
            }
        }

        let success = errors.is_empty();
        let message = if success {
            format!("All {} packages removed successfully", success_count)
        } else {
            format!("{} of {} packages removed. Errors: {}",
                    success_count, packages.len(), errors.join("; "))
        };

        Ok(InstallResult {
            success,
            message,
            packages_installed: packages.to_vec(),
        })
    }

    async fn update(&self) -> Result<()> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Winget is not available"));
        }

        let output = self.execute_winget(&["source", "update"]).await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Update failed: {}", error));
        }

        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Winget is not available"));
        }

        let args = if let Some(packages) = packages {
            let mut args = vec!["upgrade"];
            let package_args: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
            args.extend(package_args);
            args.push("--silent");
            args
        } else {
            vec!["upgrade", "--all", "--silent"]
        };

        let output = self.execute_winget(&args).await?;

        if output.status.success() {
            let packages_upgraded = packages.map(|p| p.to_vec()).unwrap_or_else(Vec::new);
            Ok(InstallResult {
                success: true,
                message: "Packages upgraded successfully".to_string(),
                packages_installed: packages_upgraded,
            })
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Ok(InstallResult {
                success: false,
                message: format!("Upgrade failed: {}", error),
                packages_installed: vec![],
            })
        }
    }

    async fn list_installed(&self) -> Result<Vec<PackageInfo>> {
        if !self.ensure_available().await? {
            return Ok(vec![]);
        }

        let output = self.execute_winget(&["list"]).await?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_list_output(&stdout))
    }

    async fn info(&self, package: &str) -> Result<Option<PackageInfo>> {
        if !self.ensure_available().await? {
            return Ok(None);
        }

        let output = self.execute_winget(&["show", package]).await?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse winget show output for package details
        let mut name = package.to_string();
        let mut version = String::new();
        let mut description = String::new();

        for line in stdout.lines() {
            if line.starts_with("Version:") {
                version = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("Description:") {
                description = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }

        Ok(Some(PackageInfo {
            name,
            version,
            description: Some(description),
            size: None,
            installed: false,
            source: "winget".to_string(),
        }))
    }

    async fn is_installed(&self, packages: &[String]) -> Result<HashMap<String, bool>> {
        let installed_packages = self.list_installed().await?;
        let installed_names: std::collections::HashSet<String> =
            installed_packages.into_iter().map(|p| p.name).collect();

        Ok(packages.iter()
            .map(|package| (package.clone(), installed_names.contains(package)))
            .collect())
    }
}