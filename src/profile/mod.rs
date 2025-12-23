use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

pub mod manager;
pub mod exporter;
pub mod importer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub parent: Option<String>,
    pub settings: ProfileSettings,
    pub packages: ProfilePackages,
    pub repositories: Vec<ProfileRepository>,
    pub environment: HashMap<String, String>,
    pub scripts: ProfileScripts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub install_location: InstallLocation,
    pub prefer_binary: bool,
    pub allow_prerelease: bool,
    pub parallel_downloads: u32,
    pub parallel_operations: u32,
    pub auto_cleanup: bool,
    pub auto_update_check: bool,
    pub confirm_major_updates: bool,
    pub keep_downloads: bool,
    pub use_cache: bool,
    pub verify_signatures: bool,
    pub verify_checksums: bool,
    pub allow_untrusted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallLocation {
    Auto,
    System,
    User,
    Custom(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePackages {
    pub system: Vec<PackageSpec>,
    pub languages: HashMap<String, Vec<PackageSpec>>,
    pub binaries: Vec<BinarySpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSpec {
    pub name: String,
    pub version: Option<String>,
    pub source: Option<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinarySpec {
    pub repository: String,
    pub version: Option<String>,
    pub asset_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRepository {
    pub url: String,
    pub name: Option<String>,
    pub enabled: bool,
    pub priority: u32,
    pub gpg_key_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileScripts {
    pub pre_install: Vec<String>,
    pub post_install: Vec<String>,
    pub pre_update: Vec<String>,
    pub post_update: Vec<String>,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            created: chrono::Utc::now(),
            updated: chrono::Utc::now(),
            parent: None,
            settings: ProfileSettings::default(),
            packages: ProfilePackages::default(),
            repositories: Vec::new(),
            environment: HashMap::new(),
            scripts: ProfileScripts::default(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_parent(mut self, parent: String) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Merge with another profile (for inheritance)
    pub fn merge(&mut self, other: &Profile) {
        // Merge packages
        for pkg in &other.packages.system {
            if !self.packages.system.iter().any(|p| p.name == pkg.name) {
                self.packages.system.push(pkg.clone());
            }
        }

        // Merge language packages
        for (lang, pkgs) in &other.packages.languages {
            let entry = self.packages.languages.entry(lang.clone()).or_insert_with(Vec::new);
            for pkg in pkgs {
                if !entry.iter().any(|p| p.name == pkg.name) {
                    entry.push(pkg.clone());
                }
            }
        }

        // Merge binaries
        for bin in &other.packages.binaries {
            if !self.packages.binaries.iter().any(|b| b.repository == bin.repository) {
                self.packages.binaries.push(bin.clone());
            }
        }

        // Merge repositories
        for repo in &other.repositories {
            if !self.repositories.iter().any(|r| r.url == repo.url) {
                self.repositories.push(repo.clone());
            }
        }

        // Merge environment variables (child overrides parent)
        for (key, value) in &other.environment {
            if !self.environment.contains_key(key) {
                self.environment.insert(key.clone(), value.clone());
            }
        }
    }

    /// Get the profile directory path
    pub fn profile_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("pkmgr").join("profiles"))
    }

    /// Load a profile from disk
    pub fn load(name: &str) -> Result<Self> {
        let profile_path = Self::profile_dir()?.join(format!("{}.toml", name));

        if !profile_path.exists() {
            bail!("Profile '{}' not found", name);
        }

        let content = fs::read_to_string(&profile_path)
            .context("Failed to read profile file")?;

        let mut profile: Profile = toml::from_str(&content)
            .context("Failed to parse profile")?;

        // Handle inheritance
        if let Some(ref parent_name) = profile.parent {
            let parent = Self::load(parent_name)?;
            let mut merged = parent.clone();
            merged.merge(&profile);
            merged.name = profile.name;
            merged.description = profile.description;
            profile = merged;
        }

        Ok(profile)
    }

    /// Save profile to disk
    pub fn save(&self) -> Result<()> {
        let profile_dir = Self::profile_dir()?;
        fs::create_dir_all(&profile_dir)?;

        let profile_path = profile_dir.join(format!("{}.toml", self.name));

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize profile")?;

        fs::write(&profile_path, content)
            .context("Failed to write profile file")?;

        Ok(())
    }

    /// List all available profiles
    pub fn list_all() -> Result<Vec<String>> {
        let profile_dir = Self::profile_dir()?;

        if !profile_dir.exists() {
            return Ok(Vec::new());
        }

        let mut profiles = Vec::new();

        for entry in fs::read_dir(&profile_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    profiles.push(name.to_string());
                }
            }
        }

        profiles.sort();
        Ok(profiles)
    }

    /// Delete a profile
    pub fn delete(name: &str) -> Result<()> {
        let profile_path = Self::profile_dir()?.join(format!("{}.toml", name));

        if profile_path.exists() {
            fs::remove_file(&profile_path)?;
            Ok(())
        } else {
            bail!("Profile '{}' not found", name);
        }
    }

    /// Create a copy of the profile
    pub fn copy(&self, new_name: &str) -> Result<Profile> {
        let mut new_profile = self.clone();
        new_profile.name = new_name.to_string();
        new_profile.created = chrono::Utc::now();
        new_profile.updated = chrono::Utc::now();
        new_profile.parent = Some(self.name.clone());

        new_profile.save()?;
        Ok(new_profile)
    }
}

impl Default for ProfileSettings {
    fn default() -> Self {
        Self {
            install_location: InstallLocation::Auto,
            prefer_binary: true,
            allow_prerelease: false,
            parallel_downloads: 4,
            parallel_operations: 2,
            auto_cleanup: true,
            auto_update_check: true,
            confirm_major_updates: true,
            keep_downloads: false,
            use_cache: true,
            verify_signatures: true,
            verify_checksums: true,
            allow_untrusted: false,
        }
    }
}

impl Default for ProfilePackages {
    fn default() -> Self {
        Self {
            system: Vec::new(),
            languages: HashMap::new(),
            binaries: Vec::new(),
        }
    }
}

impl Default for ProfileScripts {
    fn default() -> Self {
        Self {
            pre_install: Vec::new(),
            post_install: Vec::new(),
            pre_update: Vec::new(),
            post_update: Vec::new(),
        }
    }
}

/// Predefined profile templates
pub fn get_profile_templates() -> Vec<(String, Profile)> {
    vec![
        ("development", create_development_profile()),
        ("server", create_server_profile()),
        ("minimal", create_minimal_profile()),
        ("security", create_security_profile()),
        ("data-science", create_data_science_profile()),
        ("devops", create_devops_profile()),
    ].into_iter().map(|(name, profile)| (name.to_string(), profile)).collect()
}

fn create_development_profile() -> Profile {
    let mut profile = Profile::new("development".to_string());
    profile.description = "Full development environment with common tools".to_string();

    // System packages
    profile.packages.system = vec![
        PackageSpec { name: "git".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "build-essential".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "curl".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "wget".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "vim".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "tmux".to_string(), version: None, source: None, options: HashMap::new() },
    ];

    // Language packages
    profile.packages.languages.insert("node".to_string(), vec![
        PackageSpec { name: "typescript".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "eslint".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "prettier".to_string(), version: None, source: None, options: HashMap::new() },
    ]);

    profile.packages.languages.insert("python".to_string(), vec![
        PackageSpec { name: "pip".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "virtualenv".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "black".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "pytest".to_string(), version: None, source: None, options: HashMap::new() },
    ]);

    // Binary tools
    profile.packages.binaries = vec![
        BinarySpec { repository: "jesseduffield/lazygit".to_string(), version: None, asset_pattern: None },
        BinarySpec { repository: "jesseduffield/lazydocker".to_string(), version: None, asset_pattern: None },
        BinarySpec { repository: "junegunn/fzf".to_string(), version: None, asset_pattern: None },
    ];

    profile
}

fn create_server_profile() -> Profile {
    let mut profile = Profile::new("server".to_string());
    profile.description = "Server environment with monitoring and management tools".to_string();

    profile.packages.system = vec![
        PackageSpec { name: "nginx".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "docker-ce".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "htop".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "fail2ban".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "ufw".to_string(), version: None, source: None, options: HashMap::new() },
    ];

    profile
}

fn create_minimal_profile() -> Profile {
    let mut profile = Profile::new("minimal".to_string());
    profile.description = "Minimal system with only essential tools".to_string();

    profile.packages.system = vec![
        PackageSpec { name: "curl".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "nano".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "htop".to_string(), version: None, source: None, options: HashMap::new() },
    ];

    profile.settings.auto_cleanup = true;
    profile.settings.keep_downloads = false;

    profile
}

fn create_security_profile() -> Profile {
    let mut profile = Profile::new("security".to_string());
    profile.description = "Security-focused profile with hardened settings".to_string();

    profile.settings.verify_signatures = true;
    profile.settings.verify_checksums = true;
    profile.settings.allow_untrusted = false;

    profile.packages.system = vec![
        PackageSpec { name: "nmap".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "wireshark".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "metasploit-framework".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "lynis".to_string(), version: None, source: None, options: HashMap::new() },
    ];

    profile
}

fn create_data_science_profile() -> Profile {
    let mut profile = Profile::new("data-science".to_string());
    profile.description = "Data science environment with Python and R".to_string();

    profile.packages.languages.insert("python".to_string(), vec![
        PackageSpec { name: "jupyter".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "pandas".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "numpy".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "scikit-learn".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "matplotlib".to_string(), version: None, source: None, options: HashMap::new() },
    ]);

    profile
}

fn create_devops_profile() -> Profile {
    let mut profile = Profile::new("devops".to_string());
    profile.description = "DevOps tools for CI/CD and infrastructure".to_string();

    profile.packages.system = vec![
        PackageSpec { name: "docker-ce".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "kubectl".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "terraform".to_string(), version: None, source: None, options: HashMap::new() },
        PackageSpec { name: "ansible".to_string(), version: None, source: None, options: HashMap::new() },
    ];

    profile.packages.binaries = vec![
        BinarySpec { repository: "helm/helm".to_string(), version: None, asset_pattern: None },
        BinarySpec { repository: "kubernetes-sigs/kind".to_string(), version: None, asset_pattern: None },
    ];

    profile
}