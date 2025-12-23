use anyhow::{Result, bail, Context};
use std::path::PathBuf;
use std::collections::HashMap;
use reqwest;
use tokio::fs;
use tokio::process::Command;
use crate::ui::output::Output;
use crate::core::config::Config;

pub struct LanguageInstaller {
    language: String,
    output: Output,
    base_dir: PathBuf,
}

impl LanguageInstaller {
    pub fn new(language: String, output: Output, config: &Config) -> Self {
        let base_dir = PathBuf::from(&config.paths.data_dir)
            .join("languages")
            .join(&language);
        Self { language, output, base_dir }
    }

    pub async fn install_version(&self, version: &str) -> Result<PathBuf> {
        self.output.info(&format!("📥 Installing {} {}", self.language, version));

        // Create installation directory
        let install_path = self.base_dir.join(version);
        fs::create_dir_all(&install_path).await
            .context("Failed to create installation directory")?;

        // Download and install based on language
        match self.language.as_str() {
            "python" => self.install_python(version, &install_path).await,
            "node" => self.install_node(version, &install_path).await,
            "go" => self.install_go(version, &install_path).await,
            "rust" => self.install_rust(version, &install_path).await,
            "ruby" => self.install_ruby(version, &install_path).await,
            _ => bail!("Unsupported language: {}", self.language),
        }?;

        self.output.success(&format!("✅ Installed {} {} to {}",
            self.language, version, install_path.display()));

        Ok(install_path)
    }

    async fn install_python(&self, version: &str, install_path: &PathBuf) -> Result<()> {
        self.output.info("🐍 Installing Python from python.org...");

        let arch = self.detect_architecture();
        let download_url = format!(
            "https://www.python.org/ftp/python/{}/Python-{}.tgz",
            version, version
        );

        // Download source
        let archive_path = self.download_file(&download_url, &format!("Python-{}.tgz", version)).await?;

        // Extract and build
        self.output.info("🔧 Building Python from source...");
        let build_dir = archive_path.parent().unwrap().join(format!("Python-{}", version));

        // Extract
        Command::new("tar")
            .args(["-xzf", &archive_path.to_string_lossy(), "-C", &archive_path.parent().unwrap().to_string_lossy()])
            .status()
            .await
            .context("Failed to extract Python archive")?;

        // Configure and build
        Command::new("./configure")
            .current_dir(&build_dir)
            .args([&format!("--prefix={}", install_path.display())])
            .status()
            .await
            .context("Failed to configure Python build")?;

        Command::new("make")
            .current_dir(&build_dir)
            .args(["-j", &num_cpus::get().to_string()])
            .status()
            .await
            .context("Failed to build Python")?;

        Command::new("make")
            .current_dir(&build_dir)
            .arg("install")
            .status()
            .await
            .context("Failed to install Python")?;

        Ok(())
    }

    async fn install_node(&self, version: &str, install_path: &PathBuf) -> Result<()> {
        self.output.info("📦 Installing Node.js from nodejs.org...");

        let arch = self.detect_architecture();
        let platform = if cfg!(target_os = "macos") { "darwin" } else { "linux" };

        let download_url = format!(
            "https://nodejs.org/dist/v{}/node-v{}-{}-{}.tar.xz",
            version, version, platform, arch
        );

        // Download binary distribution
        let archive_path = self.download_file(&download_url, &format!("node-v{}-{}-{}.tar.xz", version, platform, arch)).await?;

        // Extract
        self.output.info("📦 Extracting Node.js...");
        Command::new("tar")
            .args(["-xJf", &archive_path.to_string_lossy(), "-C", &install_path.to_string_lossy(), "--strip-components=1"])
            .status()
            .await
            .context("Failed to extract Node.js archive")?;

        Ok(())
    }

    async fn install_go(&self, version: &str, install_path: &PathBuf) -> Result<()> {
        self.output.info("🐹 Installing Go from go.dev...");

        let arch = self.detect_architecture();
        let platform = if cfg!(target_os = "macos") { "darwin" } else { "linux" };

        let download_url = format!(
            "https://go.dev/dl/go{}.{}-{}.tar.gz",
            version, platform, arch
        );

        // Download binary distribution
        let archive_path = self.download_file(&download_url, &format!("go{}.{}-{}.tar.gz", version, platform, arch)).await?;

        // Extract
        self.output.info("📦 Extracting Go...");
        Command::new("tar")
            .args(["-xzf", &archive_path.to_string_lossy(), "-C", &install_path.to_string_lossy(), "--strip-components=1"])
            .status()
            .await
            .context("Failed to extract Go archive")?;

        Ok(())
    }

    async fn install_rust(&self, version: &str, install_path: &PathBuf) -> Result<()> {
        self.output.info("🦀 Installing Rust via rustup...");

        // Use rustup for Rust installation
        let rustup_home = install_path.join("rustup");
        let cargo_home = install_path.join("cargo");

        fs::create_dir_all(&rustup_home).await?;
        fs::create_dir_all(&cargo_home).await?;

        Command::new("rustup")
            .env("RUSTUP_HOME", &rustup_home)
            .env("CARGO_HOME", &cargo_home)
            .args(["toolchain", "install", version])
            .status()
            .await
            .context("Failed to install Rust toolchain")?;

        Ok(())
    }

    async fn install_ruby(&self, version: &str, install_path: &PathBuf) -> Result<()> {
        self.output.info("💎 Installing Ruby from ruby-lang.org...");

        let download_url = format!(
            "https://cache.ruby-lang.org/pub/ruby/{}/ruby-{}.tar.xz",
            &version[..3], // First 3 characters (e.g., "3.2" from "3.2.0")
            version
        );

        // Download source
        let archive_path = self.download_file(&download_url, &format!("ruby-{}.tar.xz", version)).await?;

        // Extract and build
        self.output.info("🔧 Building Ruby from source...");
        let build_dir = archive_path.parent().unwrap().join(format!("ruby-{}", version));

        // Extract
        Command::new("tar")
            .args(["-xJf", &archive_path.to_string_lossy(), "-C", &archive_path.parent().unwrap().to_string_lossy()])
            .status()
            .await
            .context("Failed to extract Ruby archive")?;

        // Configure and build
        Command::new("./configure")
            .current_dir(&build_dir)
            .args([&format!("--prefix={}", install_path.display())])
            .status()
            .await
            .context("Failed to configure Ruby build")?;

        Command::new("make")
            .current_dir(&build_dir)
            .args(["-j", &num_cpus::get().to_string()])
            .status()
            .await
            .context("Failed to build Ruby")?;

        Command::new("make")
            .current_dir(&build_dir)
            .arg("install")
            .status()
            .await
            .context("Failed to install Ruby")?;

        Ok(())
    }

    async fn download_file(&self, url: &str, filename: &str) -> Result<PathBuf> {
        let temp_dir = PathBuf::from("/tmp/pkmgr");
        fs::create_dir_all(&temp_dir).await?;

        let file_path = temp_dir.join(filename);

        self.output.info(&format!("📥 Downloading from {}", url));

        let client = reqwest::Client::new();
        let response = client.get(url).send().await
            .context("Failed to download file")?;

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

    fn detect_architecture(&self) -> &'static str {
        if cfg!(target_arch = "x86_64") {
            "x64"
        } else if cfg!(target_arch = "aarch64") {
            "arm64"
        } else if cfg!(target_arch = "arm") {
            "armv7l"
        } else {
            "x64" // fallback
        }
    }

    pub async fn get_available_versions(&self) -> Result<Vec<String>> {
        self.output.info(&format!("🔍 Fetching available {} versions...", self.language));

        // This would ideally fetch from APIs, but for now return common versions
        let versions = match self.language.as_str() {
            "python" => vec!["3.12.1", "3.11.7", "3.10.13", "3.9.18"],
            "node" => vec!["20.10.0", "18.19.0", "16.20.2", "14.21.3"],
            "go" => vec!["1.21.5", "1.20.12", "1.19.13"],
            "rust" => vec!["1.75.0", "1.74.1", "1.73.0"],
            "ruby" => vec!["3.3.0", "3.2.2", "3.1.4", "3.0.6"],
            _ => vec![],
        };

        Ok(versions.into_iter().map(String::from).collect())
    }
}