use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpdateBranch {
    Stable,
    Beta,
    Daily,
}

impl UpdateBranch {
    pub fn as_str(&self) -> &str {
        match self {
            UpdateBranch::Stable => "stable",
            UpdateBranch::Beta => "beta",
            UpdateBranch::Daily => "daily",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "stable" => Ok(UpdateBranch::Stable),
            "beta" => Ok(UpdateBranch::Beta),
            "daily" => Ok(UpdateBranch::Daily),
            _ => anyhow::bail!("Invalid branch: {}. Valid branches: stable, beta, daily", s),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateConfig {
    branch: String,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            branch: "stable".to_string(),
        }
    }
}

pub struct UpdateManager {
    config_path: PathBuf,
    current_version: String,
    repo_owner: String,
    repo_name: String,
}

impl UpdateManager {
    pub fn new(current_version: String) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("pkmgr");
        
        fs::create_dir_all(&config_dir)?;
        
        Ok(Self {
            config_path: config_dir.join("update.toml"),
            current_version,
            repo_owner: "pkmgr".to_string(),
            repo_name: "pkmgr".to_string(),
        })
    }

    pub fn get_branch(&self) -> Result<UpdateBranch> {
        if !self.config_path.exists() {
            return Ok(UpdateBranch::Stable);
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: UpdateConfig = toml::from_str(&content)?;
        UpdateBranch::from_str(&config.branch)
    }

    pub fn set_branch(&self, branch: UpdateBranch) -> Result<()> {
        let config = UpdateConfig {
            branch: branch.as_str().to_string(),
        };

        let content = toml::to_string_pretty(&config)?;
        fs::write(&self.config_path, content)?;

        println!("✅ Update branch set to: {}", branch.as_str());
        Ok(())
    }

    pub fn check_for_updates(&self) -> Result<Option<String>> {
        let branch = self.get_branch()?;
        
        println!("🔍 Checking for updates on {} branch...", branch.as_str());
        
        let latest_version = self.fetch_latest_version(branch)?;
        
        if let Some(ref version) = latest_version {
            if version != &self.current_version {
                println!("✨ New version available: {} → {}", self.current_version, version);
                return Ok(Some(version.clone()));
            } else {
                println!("✅ Already up to date ({})", self.current_version);
            }
        } else {
            println!("ℹ️  No updates available");
        }

        Ok(latest_version)
    }

    fn fetch_latest_version(&self, branch: UpdateBranch) -> Result<Option<String>> {
        let url = match branch {
            UpdateBranch::Stable => {
                format!(
                    "https://api.github.com/repos/{}/{}/releases/latest",
                    self.repo_owner, self.repo_name
                )
            }
            UpdateBranch::Beta | UpdateBranch::Daily => {
                format!(
                    "https://api.github.com/repos/{}/{}/releases",
                    self.repo_owner, self.repo_name
                )
            }
        };

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("pkmgr")
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let response = client.get(&url).send()?;

        if response.status() == 404 {
            return Ok(None);
        }

        response.error_for_status_ref()?;

        let json: serde_json::Value = response.json()?;

        match branch {
            UpdateBranch::Stable => {
                let tag = json["tag_name"]
                    .as_str()
                    .context("No tag_name in response")?;
                Ok(Some(tag.trim_start_matches('v').to_string()))
            }
            UpdateBranch::Beta => {
                let releases = json
                    .as_array()
                    .context("Expected array of releases")?;
                
                for release in releases {
                    if release["prerelease"].as_bool() == Some(true) {
                        let tag = release["tag_name"]
                            .as_str()
                            .context("No tag_name in release")?;
                        
                        if tag.contains("-beta") {
                            return Ok(Some(tag.trim_start_matches('v').to_string()));
                        }
                    }
                }
                Ok(None)
            }
            UpdateBranch::Daily => {
                let releases = json
                    .as_array()
                    .context("Expected array of releases")?;
                
                for release in releases {
                    if release["prerelease"].as_bool() == Some(true) {
                        let tag = release["tag_name"]
                            .as_str()
                            .context("No tag_name in release")?;
                        
                        // Daily builds are timestamps without -beta suffix
                        if !tag.contains("-beta") && tag.chars().all(|c| c.is_ascii_digit()) {
                            return Ok(Some(tag.to_string()));
                        }
                    }
                }
                Ok(None)
            }
        }
    }

    pub fn perform_update(&self) -> Result<()> {
        let branch = self.get_branch()?;
        
        println!("⏳ Checking for updates on {} branch...", branch.as_str());
        
        let latest_version = self.fetch_latest_version(branch)?;
        
        match latest_version {
            Some(version) if version != self.current_version => {
                println!("📦 Downloading version {}...", version);
                self.download_and_install(&version, branch)?;
            }
            Some(_) => {
                println!("✅ Already up to date ({})", self.current_version);
            }
            None => {
                println!("ℹ️  No updates available");
            }
        }

        Ok(())
    }

    fn download_and_install(&self, version: &str, branch: UpdateBranch) -> Result<()> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        
        let tag = match branch {
            UpdateBranch::Stable => format!("v{}", version),
            _ => version.to_string(),
        };

        let binary_name = if os == "windows" {
            format!("pkmgr-{}-{}.exe", os, arch)
        } else {
            format!("pkmgr-{}-{}", os, arch)
        };

        let download_url = format!(
            "https://github.com/{}/{}/releases/download/{}/{}",
            self.repo_owner, self.repo_name, tag, binary_name
        );

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("pkmgr")
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        println!("📥 Downloading from: {}", download_url);
        
        let response = client.get(&download_url).send()?;
        response.error_for_status_ref()?;

        let bytes = response.bytes()?;
        
        let current_exe = std::env::current_exe()?;
        let backup_path = current_exe.with_extension("bak");
        
        println!("💾 Creating backup...");
        fs::copy(&current_exe, &backup_path)?;

        println!("✨ Installing new version...");
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let temp_path = current_exe.with_extension("new");
            fs::write(&temp_path, &bytes)?;
            let mut perms = fs::metadata(&current_exe)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&temp_path, perms)?;
            fs::rename(&temp_path, &current_exe)?;
        }

        #[cfg(windows)]
        {
            let temp_path = current_exe.with_extension("new");
            fs::write(&temp_path, &bytes)?;
            fs::rename(&temp_path, &current_exe)?;
        }

        println!("✅ Update complete! Version {} installed", version);
        println!("💡 Restart pkmgr to use the new version");

        Ok(())
    }
}
