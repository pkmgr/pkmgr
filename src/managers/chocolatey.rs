use anyhow::{Result, Context};
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};
use crate::ui::output::Output;

pub struct ChocolateyManager {
    output: Output,
}

impl ChocolateyManager {
    pub fn new() -> Self {
        Self {
            output: Output::new("auto".to_string(), true),
        }
    }

    /// Check if chocolatey is available and install if needed
    async fn ensure_available(&self) -> Result<bool> {
        if self.is_available().await {
            return Ok(true);
        }

        self.output.info("📦 Chocolatey not found. Installing automatically...");
        self.install_chocolatey().await
    }

    /// Install chocolatey according to CLAUDE.md specification
    async fn install_chocolatey(&self) -> Result<bool> {
        self.output.info("⏳ Installing Chocolatey via PowerShell...");

        // Use official PowerShell command from chocolatey.org
        // This uses PowerShell's built-in web client instead of external downloads
        let install_command = r#"
            Set-ExecutionPolicy Bypass -Scope Process -Force;
            [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
            iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
        "#;

        self.output.info("⏳ Installing (requires admin)...");

        // Execute PowerShell command
        let output = Command::new("powershell")
            .args(&[
                "-ExecutionPolicy", "Bypass",
                "-Command", install_command
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute Chocolatey installer")?;

        if output.status.success() {
            self.output.success("✅ Chocolatey installed successfully!");
            Ok(true)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Chocolatey installation failed: {}", error))
        }
    }

    /// Execute chocolatey command with proper error handling
    async fn execute_choco(&self, args: &[&str]) -> Result<std::process::Output> {
        Command::new("choco")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute chocolatey command")
    }

    /// Parse chocolatey search output
    fn parse_search_output(&self, output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in output.lines() {
            if let Some(captures) = Regex::new(r"^(\S+)\s+(\S+)\s+(.+)$")
                .unwrap()
                .captures(line) {

                packages.push(PackageInfo {
                    name: captures[1].to_string(),
                    version: captures[2].to_string(),
                    description: Some(captures[3].to_string()),
                    size: None,
                    installed: false,
                    source: "chocolatey".to_string(),
                });
            }
        }

        packages
    }

    /// Parse chocolatey list output for installed packages
    fn parse_list_output(&self, output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in output.lines() {
            if let Some(captures) = Regex::new(r"^(\S+)\s+(\S+)$")
                .unwrap()
                .captures(line) {

                packages.push(PackageInfo {
                    name: captures[1].to_string(),
                    version: captures[2].to_string(),
                    description: None,
                    size: None,
                    installed: true,
                    source: "chocolatey".to_string(),
                });
            }
        }

        packages
    }
}

#[async_trait]
impl PackageManager for ChocolateyManager {
    fn name(&self) -> &str {
        "chocolatey"
    }

    async fn is_available(&self) -> bool {
        Command::new("choco")
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
            return Err(anyhow::anyhow!("Chocolatey is not available"));
        }

        let output = self.execute_choco(&["search", query, "--limit-output"]).await?;

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
            return Err(anyhow::anyhow!("Chocolatey is not available"));
        }

        let mut args = vec!["install", "-y"];
        let package_args: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
        args.extend(package_args);

        let output = self.execute_choco(&args).await?;

        if output.status.success() {
            Ok(InstallResult {
                success: true,
                message: "Packages installed successfully".to_string(),
                packages_installed: packages.to_vec(),
            })
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Ok(InstallResult {
                success: false,
                message: format!("Installation failed: {}", error),
                packages_installed: vec![],
            })
        }
    }

    async fn remove(&self, packages: &[String]) -> Result<InstallResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Chocolatey is not available"));
        }

        let mut args = vec!["uninstall", "-y"];
        let package_args: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
        args.extend(package_args);

        let output = self.execute_choco(&args).await?;

        if output.status.success() {
            Ok(InstallResult {
                success: true,
                message: "Packages removed successfully".to_string(),
                packages_installed: packages.to_vec(),
            })
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Ok(InstallResult {
                success: false,
                message: format!("Removal failed: {}", error),
                packages_installed: vec![],
            })
        }
    }

    async fn update(&self) -> Result<()> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Chocolatey is not available"));
        }

        let output = self.execute_choco(&["upgrade", "chocolatey"]).await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Update failed: {}", error));
        }

        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        if !self.ensure_available().await? {
            return Err(anyhow::anyhow!("Chocolatey is not available"));
        }

        let args = if let Some(packages) = packages {
            let mut args = vec!["upgrade", "-y"];
            let package_args: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
            args.extend(package_args);
            args
        } else {
            vec!["upgrade", "all", "-y"]
        };

        let output = self.execute_choco(&args).await?;

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

        let output = self.execute_choco(&["list", "--local-only", "--limit-output"]).await?;

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

        let output = self.execute_choco(&["info", package, "--limit-output"]).await?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages = self.parse_search_output(&stdout);
        Ok(packages.into_iter().next())
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