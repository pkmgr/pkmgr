use anyhow::{Result, bail, Context};
use std::path::PathBuf;
use reqwest;
use serde_json::Value;
use tokio::fs;
use crate::ui::output::Output;

pub struct BinaryDownloader {
    output: Output,
    install_dir: PathBuf,
}

impl BinaryDownloader {
    pub fn new(output: Output, install_dir: PathBuf) -> Self {
        Self { output, install_dir }
    }

    pub async fn download_from_github(&self, repo: &str, version: Option<&str>) -> Result<PathBuf> {
        self.output.info(&format!("📦 Downloading binary from GitHub: {}", repo));

        // Get release information
        let release_url = if let Some(v) = version {
            format!("https://api.github.com/repos/{}/releases/tags/{}", repo, v)
        } else {
            format!("https://api.github.com/repos/{}/releases/latest", repo)
        };

        let client = reqwest::Client::new();
        let response = client
            .get(&release_url)
            .header("User-Agent", "pkmgr/1.0.0")
            .send()
            .await
            .context("Failed to fetch release info")?;

        if !response.status().is_success() {
            bail!("Failed to fetch release: {}", response.status());
        }

        let release: Value = response.json().await
            .context("Failed to parse release JSON")?;

        let assets = release["assets"].as_array()
            .ok_or_else(|| anyhow::anyhow!("No assets found in release"))?;

        // Select best asset for current platform
        let asset = self.select_best_asset(assets)?;
        let download_url = asset["browser_download_url"].as_str()
            .ok_or_else(|| anyhow::anyhow!("No download URL found"))?;
        let filename = asset["name"].as_str()
            .ok_or_else(|| anyhow::anyhow!("No filename found"))?;

        // Download the asset
        let asset_path = self.download_asset(download_url, filename).await?;

        // Extract and install
        let install_path = self.install_binary(&asset_path, repo).await?;

        Ok(install_path)
    }

    fn select_best_asset<'a>(&self, assets: &'a [Value]) -> Result<&'a Value> {
        let platform = self.detect_platform();
        let arch = self.detect_architecture();

        self.output.info(&format!("🔍 Selecting asset for {} {}", platform, arch));

        // Priority order for asset selection
        let priorities = vec![
            // 1. Static binaries (portable)
            format!("{}-static-{}", platform, arch),
            format!("{}-{}-static", platform, arch),
            format!("static-{}-{}", platform, arch),
            // 2. AppImage (Linux only)
            format!("{}.AppImage", arch),
            "appimage".to_string(),
            // 3. Platform-specific
            format!("{}-{}", platform, arch),
            format!("{}_{}", platform, arch),
            format!("{}.{}", platform, arch),
            // 4. Generic binary name
            platform.to_string(),
            arch.to_string(),
            // 5. Archives
            format!("{}-{}.tar.gz", platform, arch),
            format!("{}-{}.zip", platform, arch),
        ];

        // Find best matching asset
        for pattern in &priorities {
            for asset in assets {
                if let Some(name) = asset["name"].as_str() {
                    let name_lower = name.to_lowercase();
                    let pattern_lower = pattern.to_lowercase();

                    if name_lower.contains(&pattern_lower) {
                        self.output.success(&format!("✅ Selected asset: {}", name));
                        return Ok(asset);
                    }
                }
            }
        }

        // Fallback: try to find any suitable asset
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                let name_lower = name.to_lowercase();

                // Skip obviously wrong architectures
                if arch == "x64" && (name_lower.contains("arm") || name_lower.contains("aarch64")) {
                    continue;
                }
                if arch.contains("arm") && name_lower.contains("x86") {
                    continue;
                }

                // Accept if contains platform or is archive
                if name_lower.contains(platform) ||
                   name_lower.ends_with(".tar.gz") ||
                   name_lower.ends_with(".zip") ||
                   name_lower.ends_with(".7z") {
                    self.output.warn(&format!("⚠️  Using fallback asset: {}", name));
                    return Ok(asset);
                }
            }
        }

        bail!("No suitable binary asset found for {} {}", platform, arch);
    }

    async fn download_asset(&self, url: &str, filename: &str) -> Result<PathBuf> {
        let temp_dir = PathBuf::from("/tmp/pkmgr");
        fs::create_dir_all(&temp_dir).await?;

        let file_path = temp_dir.join(filename);

        self.output.info(&format!("📥 Downloading {}", filename));

        let client = reqwest::Client::new();
        let response = client.get(url).send().await
            .context("Failed to download asset")?;

        if !response.status().is_success() {
            bail!("Download failed with status: {}", response.status());
        }

        let content = response.bytes().await
            .context("Failed to read download content")?;

        let content_len = content.len();

        fs::write(&file_path, content).await
            .context("Failed to write downloaded file")?;

        self.output.success(&format!("✅ Downloaded {} ({} bytes)", filename, content_len));

        Ok(file_path)
    }

    async fn install_binary(&self, asset_path: &PathBuf, repo: &str) -> Result<PathBuf> {
        let filename = asset_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

        let binary_name = repo.split('/').last().unwrap_or(repo);
        let install_path = self.install_dir.join(binary_name);

        self.output.info(&format!("📦 Installing binary: {}", binary_name));

        if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
            // Extract tar.gz
            self.extract_targz(asset_path, &install_path, binary_name).await?;
        } else if filename.ends_with(".zip") {
            // Extract zip
            self.extract_zip(asset_path, &install_path, binary_name).await?;
        } else if filename.ends_with(".AppImage") {
            // AppImage - just copy and make executable
            fs::copy(asset_path, &install_path).await?;
            self.make_executable(&install_path).await?;
        } else {
            // Assume it's a raw binary
            fs::copy(asset_path, &install_path).await?;
            self.make_executable(&install_path).await?;
        }

        self.output.success(&format!("✅ Installed {} to {}", binary_name, install_path.display()));

        Ok(install_path)
    }

    async fn extract_targz(&self, archive_path: &PathBuf, install_path: &PathBuf, binary_name: &str) -> Result<()> {
        use tokio::process::Command;

        let temp_extract = PathBuf::from("/tmp/pkmgr/extract");
        fs::create_dir_all(&temp_extract).await?;

        // Extract archive
        Command::new("tar")
            .args(["-xzf", &archive_path.to_string_lossy(), "-C", &temp_extract.to_string_lossy()])
            .status()
            .await
            .context("Failed to extract tar.gz")?;

        // Find the binary in extracted files
        let binary_path = self.find_binary_in_dir(&temp_extract, binary_name).await?;

        // Copy to install location
        fs::copy(&binary_path, install_path).await?;
        self.make_executable(install_path).await?;

        // Cleanup temp directory
        let _ = fs::remove_dir_all(&temp_extract).await;

        Ok(())
    }

    async fn extract_zip(&self, archive_path: &PathBuf, install_path: &PathBuf, binary_name: &str) -> Result<()> {
        use tokio::process::Command;

        let temp_extract = PathBuf::from("/tmp/pkmgr/extract");
        fs::create_dir_all(&temp_extract).await?;

        // Extract archive
        Command::new("unzip")
            .args(["-o", &archive_path.to_string_lossy(), "-d", &temp_extract.to_string_lossy()])
            .status()
            .await
            .context("Failed to extract zip")?;

        // Find the binary in extracted files
        let binary_path = self.find_binary_in_dir(&temp_extract, binary_name).await?;

        // Copy to install location
        fs::copy(&binary_path, install_path).await?;
        self.make_executable(install_path).await?;

        // Cleanup temp directory
        let _ = fs::remove_dir_all(&temp_extract).await;

        Ok(())
    }

    async fn find_binary_in_dir(&self, dir: &PathBuf, binary_name: &str) -> Result<PathBuf> {
        use tokio::fs::read_dir;
        use futures_util::future::BoxFuture;
        
        // Helper function for recursion with boxing
        fn find_recursive<'a>(
            output: &'a crate::ui::output::Output,
            dir: &'a PathBuf,
            binary_name: &'a str,
        ) -> BoxFuture<'a, Result<PathBuf>> {
            Box::pin(async move {
                let mut entries = read_dir(dir).await?;

                // First, look for exact name match
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        if filename == binary_name {
                            return Ok(path);
                        }
                    }
                }

                // Then look for files containing the binary name
                let mut entries = read_dir(dir).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                            if filename.contains(binary_name) && !filename.ends_with(".txt") && !filename.ends_with(".md") {
                                return Ok(path);
                            }
                        }
                    }
                }

                // Recursively search subdirectories
                let mut entries = read_dir(dir).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(found) = find_recursive(output, &path, binary_name).await {
                            return Ok(found);
                        }
                    }
                }

                bail!("Binary {} not found in extracted archive", binary_name)
            })
        }

        find_recursive(&self.output, dir, binary_name).await
    }

    async fn make_executable(&self, path: &PathBuf) -> Result<()> {
        use tokio::process::Command;

        Command::new("chmod")
            .args(["+x", &path.to_string_lossy()])
            .status()
            .await
            .context("Failed to make binary executable")?;

        Ok(())
    }

    fn detect_platform(&self) -> &'static str {
        if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "darwin"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux" // fallback
        }
    }

    fn detect_architecture(&self) -> &'static str {
        if cfg!(target_arch = "x86_64") {
            "x64"
        } else if cfg!(target_arch = "aarch64") {
            "arm64"
        } else if cfg!(target_arch = "arm") {
            "armv7"
        } else {
            "x64" // fallback
        }
    }

    pub async fn search_github_releases(&self, query: &str) -> Result<Vec<String>> {
        self.output.info(&format!("🔍 Searching GitHub for: {}", query));

        let search_url = format!("https://api.github.com/search/repositories?q={}&sort=stars&order=desc", query);

        let client = reqwest::Client::new();
        let response = client
            .get(&search_url)
            .header("User-Agent", "pkmgr/1.0.0")
            .send()
            .await
            .context("Failed to search GitHub")?;

        if !response.status().is_success() {
            bail!("GitHub search failed: {}", response.status());
        }

        let search_result: Value = response.json().await
            .context("Failed to parse search results")?;

        let items = search_result["items"].as_array()
            .ok_or_else(|| anyhow::anyhow!("No search results"))?;

        let mut repositories = Vec::new();
        for item in items.iter().take(10) {
            if let Some(full_name) = item["full_name"].as_str() {
                repositories.push(full_name.to_string());
            }
        }

        Ok(repositories)
    }
}