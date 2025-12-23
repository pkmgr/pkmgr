use anyhow::{Result, Context, bail};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use regex::Regex;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};

pub struct DnfManager {
    sudo_available: bool,
}

impl DnfManager {
    pub fn new() -> Self {
        Self {
            sudo_available: Self::check_sudo_available(),
        }
    }

    fn check_sudo_available() -> bool {
        Command::new("sudo")
            .args(["-n", "true"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    fn run_command(&self, args: &[&str], needs_sudo: bool) -> Result<String> {
        let mut cmd = if needs_sudo && self.sudo_available {
            let mut c = Command::new("sudo");
            c.arg("dnf");
            c
        } else {
            Command::new("dnf")
        };

        cmd.args(args);
        cmd.arg("-y"); // Auto-confirm
        cmd.arg("--quiet"); // Minimal output

        let output = cmd.output()
            .context("Failed to execute dnf command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("DNF command failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn parse_package_info(&self, info_output: &str) -> Option<PackageInfo> {
        let mut name = String::new();
        let mut version = String::new();
        let mut description = None;
        let mut size = None;
        let mut installed = false;

        for line in info_output.lines() {
            let line = line.trim();
            if line.starts_with("Name") {
                name = line.split(':').nth(1)?.trim().to_string();
            } else if line.starts_with("Version") {
                version = line.split(':').nth(1)?.trim().to_string();
            } else if line.starts_with("Summary") {
                description = Some(line.split(':').nth(1)?.trim().to_string());
            } else if line.starts_with("Size") {
                if let Some(size_str) = line.split(':').nth(1) {
                    size = Self::parse_size(size_str.trim());
                }
            } else if line.contains("installed") {
                installed = true;
            }
        }

        if !name.is_empty() && !version.is_empty() {
            Some(PackageInfo {
                name,
                version,
                description,
                size,
                installed,
                source: "dnf".to_string(),
            })
        } else {
            None
        }
    }

    fn parse_size(size_str: &str) -> Option<u64> {
        let re = Regex::new(r"([\d.]+)\s*([kKmMgG]?)[Bb]?").ok()?;
        let caps = re.captures(size_str)?;

        let number: f64 = caps.get(1)?.as_str().parse().ok()?;
        let unit = caps.get(2).map(|m| m.as_str().to_lowercase()).unwrap_or_default();

        let multiplier = match unit.as_str() {
            "k" => 1024,
            "m" => 1024 * 1024,
            "g" => 1024 * 1024 * 1024,
            _ => 1,
        };

        Some((number * multiplier as f64) as u64)
    }

    fn parse_search_results(&self, search_output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();
        let mut current_package = None;

        for line in search_output.lines() {
            let line = line.trim();

            // Package name line format: "package-name.arch : summary"
            if let Some(colon_pos) = line.find(" : ") {
                if let Some(pkg) = current_package.take() {
                    packages.push(pkg);
                }

                let (name_arch, summary) = line.split_at(colon_pos);
                let name = name_arch.split('.').next().unwrap_or(name_arch).trim();
                let summary = &summary[3..].trim(); // Remove " : "

                current_package = Some(PackageInfo {
                    name: name.to_string(),
                    version: "unknown".to_string(),
                    description: Some(summary.to_string()),
                    size: None,
                    installed: false,
                    source: "dnf".to_string(),
                });
            }
        }

        if let Some(pkg) = current_package {
            packages.push(pkg);
        }

        packages
    }
}

#[async_trait]
impl PackageManager for DnfManager {
    fn name(&self) -> &str {
        "dnf"
    }

    async fn is_available(&self) -> bool {
        which::which("dnf").is_ok()
    }

    async fn search(&self, query: &str) -> Result<SearchResult> {
        let output = self.run_command(&["search", query], false)?;
        let packages = self.parse_search_results(&output);
        let total_count = packages.len();

        Ok(SearchResult { packages, total_count })
    }

    async fn install(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["install"];
        for package in packages {
            args.push(package);
        }

        let output = self.run_command(&args, true)?;

        Ok(InstallResult {
            success: true,
            message: format!("Successfully installed {} packages", packages.len()),
            packages_installed: packages.to_vec(),
        })
    }

    async fn remove(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["remove"];
        for package in packages {
            args.push(package);
        }

        let output = self.run_command(&args, true)?;

        Ok(InstallResult {
            success: true,
            message: format!("Successfully removed {} packages", packages.len()),
            packages_installed: packages.to_vec(),
        })
    }

    async fn update(&self) -> Result<()> {
        self.run_command(&["check-update"], false)?;
        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        let mut args = vec!["upgrade"];

        if let Some(pkgs) = packages {
            for package in pkgs {
                args.push(package);
            }
        }

        let output = self.run_command(&args, true)?;

        Ok(InstallResult {
            success: true,
            message: "System upgraded successfully".to_string(),
            packages_installed: packages.map(|p| p.to_vec()).unwrap_or_default(),
        })
    }

    async fn list_installed(&self) -> Result<Vec<PackageInfo>> {
        let output = self.run_command(&["list", "installed"], false)?;
        let packages = self.parse_search_results(&output);
        Ok(packages)
    }

    async fn info(&self, package: &str) -> Result<Option<PackageInfo>> {
        match self.run_command(&["info", package], false) {
            Ok(output) => Ok(self.parse_package_info(&output)),
            Err(_) => Ok(None), // Package not found
        }
    }

    async fn is_installed(&self, packages: &[String]) -> Result<HashMap<String, bool>> {
        let mut result = HashMap::new();

        for package in packages {
            let is_installed = self.run_command(&["list", "installed", package], false).is_ok();
            result.insert(package.clone(), is_installed);
        }

        Ok(result)
    }
}