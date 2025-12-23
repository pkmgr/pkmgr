use anyhow::{Result, Context, bail};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use regex::Regex;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};

pub struct PacmanManager {
    sudo_available: bool,
}

impl PacmanManager {
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
            c.arg("pacman");
            c
        } else {
            Command::new("pacman")
        };

        cmd.args(args);
        cmd.arg("--noconfirm"); // Auto-confirm
        cmd.env("LANG", "C"); // English output

        let output = cmd.output()
            .context("Failed to execute pacman command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Pacman command failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn parse_search_results(&self, search_output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in search_output.lines() {
            // Pacman search format: "repo/package-name version"
            //   description
            if let Some(captures) = Regex::new(r"^(\S+)/(\S+)\s+(\S+)")
                .unwrap()
                .captures(line) {
                
                let name = captures[2].to_string();
                let version = captures[3].to_string();
                
                packages.push(PackageInfo {
                    name,
                    version,
                    description: None,
                    size: None,
                    installed: false,
                    source: "pacman".to_string(),
                });
            }
        }

        packages
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
            } else if line.starts_with("Description") {
                description = Some(line.split(':').nth(1)?.trim().to_string());
            } else if line.starts_with("Installed Size") || line.starts_with("Download Size") {
                if let Some(size_str) = line.split(':').nth(1) {
                    size = Self::parse_size(size_str.trim());
                }
            }
        }

        // Check if it's showing local package info (installed)
        if info_output.contains("local/") {
            installed = true;
        }

        if !name.is_empty() && !version.is_empty() {
            Some(PackageInfo {
                name,
                version,
                description,
                size,
                installed,
                source: "pacman".to_string(),
            })
        } else {
            None
        }
    }

    fn parse_size(size_str: &str) -> Option<u64> {
        let re = Regex::new(r"([\d.]+)\s*([KMG]?i?B)").ok()?;
        let caps = re.captures(size_str)?;

        let number: f64 = caps.get(1)?.as_str().parse().ok()?;
        let unit = caps.get(2).map(|m| m.as_str()).unwrap_or("");

        let multiplier = match unit {
            "KiB" => 1024,
            "MiB" => 1024 * 1024,
            "GiB" => 1024 * 1024 * 1024,
            "KB" => 1000,
            "MB" => 1000 * 1000,
            "GB" => 1000 * 1000 * 1000,
            _ => 1,
        };

        Some((number * multiplier as f64) as u64)
    }
}

#[async_trait]
impl PackageManager for PacmanManager {
    fn name(&self) -> &str {
        "pacman"
    }

    async fn is_available(&self) -> bool {
        which::which("pacman").is_ok()
    }

    async fn search(&self, query: &str) -> Result<SearchResult> {
        let output = self.run_command(&["-Ss", query], false)?;
        let packages = self.parse_search_results(&output);
        let total_count = packages.len();

        Ok(SearchResult { packages, total_count })
    }

    async fn install(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["-S"];
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
        let mut args = vec!["-R"];
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
        self.run_command(&["-Sy"], true)?;
        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        let args = if let Some(pkgs) = packages {
            let mut args = vec!["-S"];
            for package in pkgs {
                args.push(package);
            }
            args
        } else {
            vec!["-Syu"]
        };

        let output = self.run_command(&args, true)?;

        Ok(InstallResult {
            success: true,
            message: "System upgraded successfully".to_string(),
            packages_installed: packages.map(|p| p.to_vec()).unwrap_or_default(),
        })
    }

    async fn list_installed(&self) -> Result<Vec<PackageInfo>> {
        let output = self.run_command(&["-Q"], false)?;
        
        let mut packages = Vec::new();
        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(PackageInfo {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: None,
                    size: None,
                    installed: true,
                    source: "pacman".to_string(),
                });
            }
        }
        
        Ok(packages)
    }

    async fn info(&self, package: &str) -> Result<Option<PackageInfo>> {
        // Try local package first
        match self.run_command(&["-Qi", package], false) {
            Ok(output) => return Ok(self.parse_package_info(&output)),
            Err(_) => {},
        }

        // Try remote package
        match self.run_command(&["-Si", package], false) {
            Ok(output) => Ok(self.parse_package_info(&output)),
            Err(_) => Ok(None),
        }
    }

    async fn is_installed(&self, packages: &[String]) -> Result<HashMap<String, bool>> {
        let mut result = HashMap::new();

        for package in packages {
            let is_installed = self.run_command(&["-Q", package], false).is_ok();
            result.insert(package.clone(), is_installed);
        }

        Ok(result)
    }
}