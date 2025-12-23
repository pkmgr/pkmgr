use anyhow::{Result, Context, bail};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use serde_json::Value;
use crate::core::{PackageManager, PackageInfo, SearchResult, InstallResult};

pub struct HomebrewManager {
    sudo_available: bool,
}

impl HomebrewManager {
    pub fn new() -> Self {
        Self {
            sudo_available: false, // Homebrew doesn't need sudo
        }
    }

    fn run_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("brew")
            .args(args)
            .env("HOMEBREW_NO_AUTO_UPDATE", "1") // Disable auto-update during operations
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute brew command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Brew command failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn parse_search_json(&self, json_output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        if let Ok(data) = serde_json::from_str::<Value>(json_output) {
            if let Some(formulae) = data.get("formulae").and_then(|v| v.as_array()) {
                for formula in formulae {
                    if let (Some(name), Some(version), Some(desc)) = (
                        formula.get("name").and_then(|v| v.as_str()),
                        formula.get("version").and_then(|v| v.as_str()),
                        formula.get("desc").and_then(|v| v.as_str()),
                    ) {
                        packages.push(PackageInfo {
                            name: name.to_string(),
                            version: version.to_string(),
                            description: Some(desc.to_string()),
                            size: None,
                            installed: false,
                            source: "homebrew".to_string(),
                        });
                    }
                }
            }

            if let Some(casks) = data.get("casks").and_then(|v| v.as_array()) {
                for cask in casks {
                    if let (Some(token), Some(version), Some(name)) = (
                        cask.get("token").and_then(|v| v.as_str()),
                        cask.get("version").and_then(|v| v.as_str()),
                        cask.get("name").and_then(|v| v.as_array()).and_then(|arr| arr.first()).and_then(|v| v.as_str()),
                    ) {
                        packages.push(PackageInfo {
                            name: token.to_string(),
                            version: version.to_string(),
                            description: Some(name.to_string()),
                            size: None,
                            installed: false,
                            source: "homebrew".to_string(),
                        });
                    }
                }
            }
        }

        packages
    }

    fn parse_list_output(&self, list_output: &str) -> Vec<PackageInfo> {
        let mut packages = Vec::new();

        for line in list_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(PackageInfo {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: None,
                    size: None,
                    installed: true,
                    source: "homebrew".to_string(),
                });
            }
        }

        packages
    }

    fn parse_info_json(&self, json_output: &str) -> Option<PackageInfo> {
        if let Ok(data) = serde_json::from_str::<Value>(json_output) {
            if let Some(formula_array) = data.as_array() {
                if let Some(formula) = formula_array.first() {
                    let name = formula.get("name").and_then(|v| v.as_str())?;
                    let version = formula.get("version").and_then(|v| v.as_str())?;
                    let desc = formula.get("desc").and_then(|v| v.as_str()).map(|s| s.to_string());
                    
                    // Check if installed
                    let installed = formula.get("installed").and_then(|v| v.as_array()).map(|arr| !arr.is_empty()).unwrap_or(false);

                    return Some(PackageInfo {
                        name: name.to_string(),
                        version: version.to_string(),
                        description: desc,
                        size: None,
                        installed,
                        source: "homebrew".to_string(),
                    });
                }
            }
        }

        None
    }
}

#[async_trait]
impl PackageManager for HomebrewManager {
    fn name(&self) -> &str {
        "homebrew"
    }

    async fn is_available(&self) -> bool {
        which::which("brew").is_ok()
    }

    async fn search(&self, query: &str) -> Result<SearchResult> {
        let output = self.run_command(&["search", query, "--json"])?;
        let packages = self.parse_search_json(&output);
        let total_count = packages.len();

        Ok(SearchResult { packages, total_count })
    }

    async fn install(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["install"];
        for package in packages {
            args.push(package);
        }

        let output = self.run_command(&args)?;

        Ok(InstallResult {
            success: true,
            message: format!("Successfully installed {} packages", packages.len()),
            packages_installed: packages.to_vec(),
        })
    }

    async fn remove(&self, packages: &[String]) -> Result<InstallResult> {
        let mut args = vec!["uninstall"];
        for package in packages {
            args.push(package);
        }

        let output = self.run_command(&args)?;

        Ok(InstallResult {
            success: true,
            message: format!("Successfully removed {} packages", packages.len()),
            packages_installed: packages.to_vec(),
        })
    }

    async fn update(&self) -> Result<()> {
        self.run_command(&["update"])?;
        Ok(())
    }

    async fn upgrade(&self, packages: Option<&[String]>) -> Result<InstallResult> {
        let args = if let Some(pkgs) = packages {
            let mut args = vec!["upgrade"];
            for package in pkgs {
                args.push(package);
            }
            args
        } else {
            vec!["upgrade"]
        };

        let output = self.run_command(&args)?;

        Ok(InstallResult {
            success: true,
            message: "System upgraded successfully".to_string(),
            packages_installed: packages.map(|p| p.to_vec()).unwrap_or_default(),
        })
    }

    async fn list_installed(&self) -> Result<Vec<PackageInfo>> {
        let output = self.run_command(&["list", "--versions"])?;
        Ok(self.parse_list_output(&output))
    }

    async fn info(&self, package: &str) -> Result<Option<PackageInfo>> {
        match self.run_command(&["info", package, "--json"]) {
            Ok(output) => Ok(self.parse_info_json(&output)),
            Err(_) => Ok(None),
        }
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