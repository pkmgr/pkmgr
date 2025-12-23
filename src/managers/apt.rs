use anyhow::{Result, Context, bail};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use regex::Regex;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};

pub struct AptManager {
    sudo_available: bool,
}

impl AptManager {
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

    fn run_command(&self, cmd: &str, args: &[&str], needs_sudo: bool) -> Result<String> {
        let mut command = if needs_sudo && self.sudo_available {
            let mut c = Command::new("sudo");
            c.arg(cmd);
            c
        } else {
            Command::new(cmd)
        };

        command.args(args);
        command.env("DEBIAN_FRONTEND", "noninteractive");

        let output = command.output()
            .context(format!("Failed to execute {} command", cmd))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("{} command failed: {}", cmd, stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn parse_apt_search(&self, search_output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();
        let mut current_package = None;

        for line in search_output.lines() {
            let line = line.trim();

            // Package line format: "package-name/suite,suite [version] [arch] [status]"
            if let Some(slash_pos) = line.find('/') {
                if let Some(pkg) = current_package.take() {
                    packages.push(pkg);
                }

                let package_part = &line[..slash_pos];
                let rest = &line[slash_pos + 1..];

                // Extract version if available
                let version = if let Some(bracket_start) = rest.find('[') {
                    if let Some(bracket_end) = rest.find(']') {
                        rest[bracket_start + 1..bracket_end].to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    "unknown".to_string()
                };

                current_package = Some(PackageInfo {
                    name: package_part.to_string(),
                    version,
                    description: None,
                    size: None,
                    installed: rest.contains("installed"),
                    source: "apt".to_string(),
                });
            } else if line.starts_with("  ") && current_package.is_some() {
                // Description line (indented)
                if let Some(ref mut pkg) = current_package {
                    pkg.description = Some(line.trim().to_string());
                }
            }
        }

        if let Some(pkg) = current_package {
            packages.push(pkg);
        }

        packages
    }

    fn parse_apt_show(&self, show_output: &str) -> Option<PackageInfo> {
        let mut name = String::new();
        let mut version = String::new();
        let mut description = None;
        let mut size = None;
        let mut installed = false;

        for line in show_output.lines() {
            let line = line.trim();
            if line.starts_with("Package:") {
                name = line.split(':').nth(1)?.trim().to_string();
            } else if line.starts_with("Version:") {
                version = line.split(':').nth(1)?.trim().to_string();
            } else if line.starts_with("Description:") {
                description = Some(line.split(':').nth(1)?.trim().to_string());
            } else if line.starts_with("Installed-Size:") {
                if let Some(size_str) = line.split(':').nth(1) {
                    // APT shows size in KB
                    if let Ok(kb) = size_str.trim().parse::<u64>() {
                        size = Some(kb * 1024);
                    }
                }
            } else if line.starts_with("Status:") && line.contains("installed") {
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
                source: "apt".to_string(),
            })
        } else {
            None
        }
    }
}

#[async_trait]
impl PackageManager for AptManager {
    fn name(&self) -> &str {
        "apt"
    }

    async fn is_available(&self) -> bool {
        which::which("apt").is_ok()
    }

    async fn search(&self, query: &str) -> Result<SearchResult> {
        let output = self.run_command("apt", &["search", query], false)?;
        let packages = self.parse_apt_search(&output);
        let total_count = packages.len();

        Ok(SearchResult { packages, total_count })
    }

    async fn install(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["install", "-y"];
        for package in packages {
            args.push(package);
        }

        let output = self.run_command("apt", &args, true)?;

        Ok(InstallResult {
            success: true,
            message: format!("Successfully installed {} packages", packages.len()),
            packages_installed: packages.to_vec(),
        })
    }

    async fn remove(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["remove", "-y"];
        for package in packages {
            args.push(package);
        }

        let output = self.run_command("apt", &args, true)?;

        Ok(InstallResult {
            success: true,
            message: format!("Successfully removed {} packages", packages.len()),
            packages_installed: packages.to_vec(),
        })
    }

    async fn update(&self) -> Result<()> {
        self.run_command("apt", &["update"], true)?;
        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        let mut args = vec!["upgrade", "-y"];

        if let Some(pkgs) = packages {
            for package in pkgs {
                args.push(package);
            }
        }

        let output = self.run_command("apt", &args, true)?;

        Ok(InstallResult {
            success: true,
            message: "System upgraded successfully".to_string(),
            packages_installed: packages.map(|p| p.to_vec()).unwrap_or_default(),
        })
    }

    async fn list_installed(&self) -> Result<Vec<PackageInfo>> {
        let output = self.run_command("apt", &["list", "--installed"], false)?;
        let packages = self.parse_apt_search(&output);
        Ok(packages)
    }

    async fn info(&self, package: &str) -> Result<Option<PackageInfo>> {
        match self.run_command("apt", &["show", package], false) {
            Ok(output) => Ok(self.parse_apt_show(&output)),
            Err(_) => Ok(None), // Package not found
        }
    }

    async fn is_installed(&self, packages: &[String]) -> Result<HashMap<String, bool>> {
        let mut result = HashMap::new();

        for package in packages {
            let is_installed = self.run_command("dpkg", &["-l", package], false).is_ok();
            result.insert(package.clone(), is_installed);
        }

        Ok(result)
    }
}