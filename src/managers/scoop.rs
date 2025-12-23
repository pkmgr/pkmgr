use anyhow::{Result, Context};
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};
use crate::ui::output::Output;

pub struct ScoopManager {
    output: Output,
}

impl ScoopManager {
    pub fn new() -> Self {
        Self {
            output: Output::new("auto".to_string(), true),
        }
    }

    /// Check if scoop is available and install if needed
    async fn ensure_available(&self) -> Result<bool> {
        if self.is_available().await {
            return Ok(true);
        }

        self.output.info("📦 Scoop not found. Installing automatically...");
        self.install_scoop().await
    }

    /// Install scoop according to CLAUDE.md specification
    async fn install_scoop(&self) -> Result<bool> {
        self.output.info("⏳ Installing Scoop via PowerShell...");

        // Use official PowerShell command from scoop.sh
        // This uses PowerShell's built-in web client instead of external downloads
        let install_command = r#"
            Set-ExecutionPolicy RemoteSigned -Scope CurrentUser;
            irm get.scoop.sh | iex
        "#;

        self.output.info("⏳ Installing (requires PowerShell 5+)...");

        // Execute PowerShell command
        let output = Command::new("powershell")
            .args(&[
                "-ExecutionPolicy", "RemoteSigned",
                "-Command", install_command
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute Scoop installer")?;

        if output.status.success() {
            self.output.success("✅ Scoop installed successfully!");
            Ok(true)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Scoop installation failed: {}", error))
        }
    }

    /// Execute scoop command with proper error handling
    async fn execute_scoop(&self, args: &[&str]) -> Result<std::process::Output> {
        Command::new("scoop")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute scoop command")
    }

    /// Parse scoop search output
    fn parse_search_output(&self, output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in output.lines() {
            // Scoop search output format: "bucket/name (version): description"
            if let Some(captures) = Regex::new(r"^(\S+)\s+\(([^)]+)\):\s*(.*)$")
                .unwrap()
                .captures(line) {

                packages.push(PackageInfo {
                    name: captures[1].to_string(),
                    version: captures[2].to_string(),
                    description: Some(captures[3].to_string()),
                    size: None,
                    installed: false,
                    source: "scoop".to_string(),
                });
            }
        }

        packages
    }

    /// Parse scoop list output for installed packages
    fn parse_list_output(&self, output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in output.lines() {
            // Skip header line
            if line.starts_with("Installed apps:") || line.trim().is_empty() {
                continue;
            }

            // Scoop list output format: "name version [bucket] *global"
            if let Some(captures) = Regex::new(r"^\s*(\S+)\s+(\S+)")
                .unwrap()
                .captures(line) {

                packages.push(PackageInfo {
                    name: captures[1].to_string(),
                    version: captures[2].to_string(),
                    description: None,
                    size: None,
                    installed: true,
                    source: "scoop".to_string(),
                });
            }
        }

        packages
    }
}

#[async_trait]
impl PackageManager for ScoopManager {
    fn name(&self) -> &str {
        "scoop"
    }

    async fn is_available(&self) -> bool {
        Command::new("scoop")
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
            return Err(anyhow::anyhow!("Scoop is not available"));
        }

        let output = self.execute_scoop(&["search", query]).await?;

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
            return Err(anyhow::anyhow!("Scoop is not available"));
        }

        let mut success_count = 0;
        let mut errors = Vec::new();

        for package in packages {
            let output = self.execute_scoop(&["install", package]).await?;

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
            return Err(anyhow::anyhow!("Scoop is not available"));
        }

        let mut success_count = 0;
        let mut errors = Vec::new();

        for package in packages {
            let output = self.execute_scoop(&["uninstall", package]).await?;

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
            return Err(anyhow::anyhow!("Scoop is not available"));
        }

        let output = self.execute_scoop(&["update"]).await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Update failed: {}", error));
        }

        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Scoop is not available"));
        }

        let args = if let Some(packages) = packages {
            let mut args = vec!["update"];
            let package_args: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
            args.extend(package_args);
            args
        } else {
            vec!["update", "*"]
        };

        let output = self.execute_scoop(&args).await?;

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

        let output = self.execute_scoop(&["list"]).await?;

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

        let output = self.execute_scoop(&["info", package]).await?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse scoop info output for package details
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
            source: "scoop".to_string(),
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