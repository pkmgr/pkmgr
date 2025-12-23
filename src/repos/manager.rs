use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use crate::ui::output::Output;
use crate::core::platform::{PlatformInfo, PackageManager};
use super::{Repository, RepositoryType, TrustLevel, detector::RepositoryDetector, gpg::GpgManager};

pub struct RepositoryManager {
    output: Output,
    platform: PlatformInfo,
    detector: RepositoryDetector,
    gpg: GpgManager,
    repos_dir: PathBuf,
}

impl RepositoryManager {
    pub fn new(output: Output, platform: PlatformInfo) -> Self {
        let repos_dir = Self::get_repos_dir(&platform);

        Self {
            detector: RepositoryDetector::new(output.clone()),
            gpg: GpgManager::new(output.clone()),
            output,
            platform,
            repos_dir,
        }
    }

    /// Get the appropriate repository directory for the platform
    fn get_repos_dir(platform: &PlatformInfo) -> PathBuf {
        let pm_name = platform.primary_package_manager()
            .map(|pm| pm.to_string())
            .unwrap_or_default();
        match pm_name.as_str() {
            "apt" => PathBuf::from("/etc/apt/sources.list.d"),
            "dnf" | "yum" => PathBuf::from("/etc/yum.repos.d"),
            "zypper" => PathBuf::from("/etc/zypp/repos.d"),
            "pacman" => PathBuf::from("/etc/pacman.d"),
            _ => PathBuf::from("/etc/pkmgr/repos.d"),
        }
    }

    /// List all repositories
    pub fn list(&self) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();

        let pm_name = self.platform.primary_package_manager()
            .map(|pm| pm.to_string())
            .unwrap_or_default();
        match pm_name.as_str() {
            "apt" => repos.extend(self.list_apt_repos()?),
            "dnf" | "yum" => repos.extend(self.list_yum_repos()?),
            "pacman" => repos.extend(self.list_pacman_repos()?),
            _ => {}
        }

        Ok(repos)
    }

    /// List APT repositories
    fn list_apt_repos(&self) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();

        // Read main sources.list
        if let Ok(content) = fs::read_to_string("/etc/apt/sources.list") {
            repos.extend(self.parse_apt_sources(&content)?);
        }

        // Read sources.list.d
        if let Ok(entries) = fs::read_dir("/etc/apt/sources.list.d") {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("list") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        repos.extend(self.parse_apt_sources(&content)?);
                    }
                }
            }
        }

        Ok(repos)
    }

    /// Parse APT sources format
    fn parse_apt_sources(&self, content: &str) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse deb or deb-src lines
            if line.starts_with("deb ") || line.starts_with("deb-src ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let url = parts[1].to_string();
                    let suite = parts[2].to_string();
                    let components: Vec<String> = parts[3..].iter()
                        .map(|s| s.to_string())
                        .collect();

                    let name = self.guess_repo_name(&url, &suite);

                    let mut repo = Repository::new(name, url, RepositoryType::Apt);
                    repo.suites = vec![suite];
                    repo.components = components;

                    repos.push(repo);
                }
            }
        }

        Ok(repos)
    }

    /// List YUM/DNF repositories
    fn list_yum_repos(&self) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();

        if let Ok(entries) = fs::read_dir("/etc/yum.repos.d") {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("repo") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        repos.extend(self.parse_yum_repo(&content)?);
                    }
                }
            }
        }

        Ok(repos)
    }

    /// Parse YUM/DNF repo format
    fn parse_yum_repo(&self, content: &str) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();
        let mut current_repo: Option<Repository> = None;
        let mut in_section = false;

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                // Save previous repo
                if let Some(repo) = current_repo.take() {
                    repos.push(repo);
                }

                // Start new repo
                let name = line[1..line.len()-1].to_string();
                current_repo = Some(Repository::new(
                    name,
                    String::new(),
                    RepositoryType::Yum,
                ));
                in_section = true;
            } else if in_section && line.contains('=') {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();

                    if let Some(ref mut repo) = current_repo {
                        match key {
                            "baseurl" | "mirrorlist" => repo.url = value.to_string(),
                            "enabled" => repo.enabled = value == "1",
                            "gpgkey" => {
                                repo.gpg_key = Some(super::GpgKeyInfo {
                                    fingerprint: String::new(),
                                    key_id: String::new(),
                                    key_server: None,
                                    key_url: Some(value.to_string()),
                                    trusted: false,
                                    expires: None,
                                    last_refreshed: None,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Save last repo
        if let Some(repo) = current_repo {
            repos.push(repo);
        }

        Ok(repos)
    }

    /// List Pacman repositories
    fn list_pacman_repos(&self) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();

        if let Ok(content) = fs::read_to_string("/etc/pacman.conf") {
            let mut current_repo: Option<String> = None;

            for line in content.lines() {
                let line = line.trim();

                if line.starts_with('[') && line.ends_with(']') && !line.contains("options") {
                    let name = line[1..line.len()-1].to_string();
                    current_repo = Some(name.clone());

                    let repo = Repository::new(
                        name,
                        String::new(), // URL will be in Server= lines
                        RepositoryType::Pacman,
                    );
                    repos.push(repo);
                }
            }
        }

        Ok(repos)
    }

    /// Add a repository
    pub async fn add(&self, repo_spec: &str) -> Result<()> {
        self.output.progress(&format!("Adding repository: {}", repo_spec));

        // Check if it's a known repository pattern
        if let Some(mut repo) = self.detector.detect_required_repository(repo_spec) {
            // Repository auto-detected
            self.output.info(&format!(
                "Auto-detected {} repository",
                repo.metadata.vendor.as_ref().unwrap_or(&repo.name)
            ));

            // Add GPG key if specified
            if let Some(ref key_info) = repo.gpg_key {
                if let Some(ref key_url) = key_info.key_url {
                    self.output.progress("Importing GPG key");
                    self.gpg.import_key_from_url(key_url).await?;
                    self.output.success("GPG key imported successfully");
                }
            }

            // Write repository configuration
            self.write_repo_config(&repo)?;

            // Update package cache
            self.update_cache().await?;

            self.output.success(&format!("Repository {} added successfully", repo.name));
        } else if repo_spec.starts_with("http://") || repo_spec.starts_with("https://") {
            // URL provided
            self.add_repo_from_url(repo_spec).await?;
        } else if repo_spec.starts_with("ppa:") {
            // PPA repository (Ubuntu)
            self.add_ppa(repo_spec).await?;
        } else {
            // Try to interpret as a package that needs a repository
            if let Some(repo) = self.detector.detect_required_repository(repo_spec) {
                return Box::pin(self.add(&repo.url)).await;
            }

            bail!("Unknown repository format: {}", repo_spec);
        }

        Ok(())
    }

    /// Add repository from URL
    async fn add_repo_from_url(&self, url: &str) -> Result<()> {
        let name = self.guess_repo_name(url, "");
        let repo_type = self.get_repo_type();

        let repo = Repository::new(name.clone(), url.to_string(), repo_type);

        // Check if it's a mirror
        if let Some(mirror_info) = self.detector.detect_mirror(url) {
            self.output.warn(&format!("Detected mirror: {}", mirror_info));
        }

        self.write_repo_config(&repo)?;
        self.update_cache().await?;

        self.output.success(&format!("Repository {} added", name));
        Ok(())
    }

    /// Add PPA repository (Ubuntu/Debian)
    async fn add_ppa(&self, ppa: &str) -> Result<()> {
        if !self.platform.package_managers.iter().any(|pm| *pm == PackageManager::Apt) {
            bail!("PPA repositories are only supported on APT-based systems");
        }

        self.output.progress(&format!("Adding PPA: {}", ppa));

        // Use add-apt-repository if available
        if Path::new("/usr/bin/add-apt-repository").exists() {
            let output = std::process::Command::new("add-apt-repository")
                .arg("-y")
                .arg(ppa)
                .output()
                .context("Failed to add PPA")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Failed to add PPA: {}", stderr);
            }

            self.update_cache().await?;
            self.output.success("PPA added successfully");
        } else {
            bail!("add-apt-repository not found. Install software-properties-common");
        }

        Ok(())
    }

    /// Remove a repository
    pub async fn remove(&self, repo_name: &str) -> Result<()> {
        self.output.progress(&format!("Removing repository: {}", repo_name));

        let pm_name = self.platform.primary_package_manager()
            .map(|pm| pm.to_string())
            .unwrap_or_default();
        match pm_name.as_str() {
            "apt" => self.remove_apt_repo(repo_name)?,
            "dnf" | "yum" => self.remove_yum_repo(repo_name)?,
            "pacman" => bail!("Pacman repository removal not implemented"),
            _ => bail!("Repository removal not supported for this package manager"),
        }

        self.output.success(&format!("Repository {} removed", repo_name));
        Ok(())
    }

    /// Remove APT repository
    fn remove_apt_repo(&self, repo_name: &str) -> Result<()> {
        let sources_dir = PathBuf::from("/etc/apt/sources.list.d");

        // Look for matching .list file
        if let Ok(entries) = fs::read_dir(&sources_dir) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("list") {
                    let filename = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");

                    if filename.contains(repo_name) {
                        fs::remove_file(&path)?;
                        self.output.info(&format!("Removed {}", path.display()));
                        return Ok(());
                    }
                }
            }
        }

        bail!("Repository {} not found", repo_name);
    }

    /// Remove YUM/DNF repository
    fn remove_yum_repo(&self, repo_name: &str) -> Result<()> {
        let repo_file = PathBuf::from(format!("/etc/yum.repos.d/{}.repo", repo_name));

        if repo_file.exists() {
            fs::remove_file(&repo_file)?;
            self.output.info(&format!("Removed {}", repo_file.display()));
            Ok(())
        } else {
            bail!("Repository {} not found", repo_name);
        }
    }

    /// Update repository cache
    pub async fn update_cache(&self) -> Result<()> {
        self.output.progress("Updating repository cache");

        let pm_name = self.platform.primary_package_manager()
            .map(|pm| pm.to_string())
            .unwrap_or_default();
        match pm_name.as_str() {
            "apt" => {
                let output = std::process::Command::new("apt-get")
                    .arg("update")
                    .output()
                    .context("Failed to update APT cache")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to update cache: {}", stderr);
                }
            }
            "dnf" => {
                let output = std::process::Command::new("dnf")
                    .arg("makecache")
                    .output()
                    .context("Failed to update DNF cache")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to update cache: {}", stderr);
                }
            }
            "yum" => {
                let output = std::process::Command::new("yum")
                    .arg("makecache")
                    .output()
                    .context("Failed to update YUM cache")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to update cache: {}", stderr);
                }
            }
            "pacman" => {
                let output = std::process::Command::new("pacman")
                    .arg("-Sy")
                    .output()
                    .context("Failed to update Pacman database")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to update cache: {}", stderr);
                }
            }
            _ => {
                self.output.warn("Cache update not implemented for this package manager");
            }
        }

        self.output.success("Repository cache updated");
        Ok(())
    }

    /// Write repository configuration
    fn write_repo_config(&self, repo: &Repository) -> Result<()> {
        let pm_name = self.platform.primary_package_manager()
            .map(|pm| pm.to_string())
            .unwrap_or_default();
        match pm_name.as_str() {
            "apt" => self.write_apt_repo(repo),
            "dnf" | "yum" => self.write_yum_repo(repo),
            _ => bail!("Repository configuration not implemented for this package manager"),
        }
    }

    /// Write APT repository configuration
    fn write_apt_repo(&self, repo: &Repository) -> Result<()> {
        let filename = format!("{}.list", repo.name.replace('/', "_"));
        let path = PathBuf::from("/etc/apt/sources.list.d").join(&filename);

        let mut content = String::new();
        content.push_str(&format!("# {} - Added by pkmgr\n", repo.name));

        if let Some(ref desc) = repo.metadata.description {
            content.push_str(&format!("# {}\n", desc));
        }

        content.push('\n');

        for suite in &repo.suites {
            content.push_str(&format!(
                "deb {} {} {}\n",
                repo.url,
                suite,
                repo.components.join(" ")
            ));
        }

        let mut file = fs::File::create(&path)?;
        file.write_all(content.as_bytes())?;

        self.output.info(&format!("Created {}", path.display()));
        Ok(())
    }

    /// Write YUM/DNF repository configuration
    fn write_yum_repo(&self, repo: &Repository) -> Result<()> {
        let filename = format!("{}.repo", repo.name);
        let path = PathBuf::from("/etc/yum.repos.d").join(&filename);

        let mut content = String::new();
        content.push_str(&format!("[{}]\n", repo.name));
        content.push_str(&format!("name={}\n",
            repo.metadata.description.as_ref().unwrap_or(&repo.name)));
        content.push_str(&format!("baseurl={}\n", repo.url));
        content.push_str(&format!("enabled={}\n", if repo.enabled { "1" } else { "0" }));
        content.push_str("gpgcheck=1\n");

        if let Some(ref key) = repo.gpg_key {
            if let Some(ref key_url) = key.key_url {
                content.push_str(&format!("gpgkey={}\n", key_url));
            }
        }

        let mut file = fs::File::create(&path)?;
        file.write_all(content.as_bytes())?;

        self.output.info(&format!("Created {}", path.display()));
        Ok(())
    }

    /// Guess repository name from URL
    fn guess_repo_name(&self, url: &str, suite: &str) -> String {
        // Try to extract from known patterns
        for known in super::get_known_repositories() {
            for pattern in known.patterns {
                if url.contains(pattern) {
                    return known.name.to_string();
                }
            }
        }

        // Extract from URL
        if let Some(domain) = url.split("://").nth(1) {
            if let Some(host) = domain.split('/').next() {
                let name = host.replace(".", "-")
                    .replace("www-", "")
                    .replace("-com", "")
                    .replace("-org", "");

                if !suite.is_empty() {
                    return format!("{}-{}", name, suite);
                }
                return name;
            }
        }

        "custom-repo".to_string()
    }

    /// Get repository type for current platform
    fn get_repo_type(&self) -> RepositoryType {
        let pm_name = self.platform.primary_package_manager()
            .map(|pm| pm.to_string())
            .unwrap_or_default();
        match pm_name.as_str() {
            "apt" => RepositoryType::Apt,
            "dnf" => RepositoryType::Dnf,
            "yum" => RepositoryType::Yum,
            "pacman" => RepositoryType::Pacman,
            "zypper" => RepositoryType::Zypper,
            _ => RepositoryType::Custom(pm_name),
        }
    }
}