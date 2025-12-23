use anyhow::{Context, Result};
use reqwest::Client;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use sha2::{Sha256, Digest};
use crate::ui::progress::ProgressManager;

pub struct Downloader {
    client: Client,
    progress_manager: ProgressManager,
}

impl Downloader {
    pub fn new(emoji_enabled: bool) -> Result<Self> {
        let client = Client::builder()
            .user_agent("pkmgr/1.0.0")
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        Ok(Self {
            client,
            progress_manager: ProgressManager::new(emoji_enabled),
        })
    }

    pub async fn download_file(&self, url: &str, dest: &Path) -> Result<()> {
        let response = self.client
            .get(url)
            .send()
            .await
            .context("Failed to send download request")?;

        let total_size = response
            .content_length()
            .unwrap_or(0);

        let pb = self.progress_manager.create_download_bar(
            total_size,
            dest.file_name().unwrap_or_default().to_str().unwrap_or("file")
        );

        let mut file = File::create(dest).await
            .context("Failed to create destination file")?;

        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        use futures_util::StreamExt;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to download chunk")?;
            file.write_all(&chunk).await
                .context("Failed to write chunk to file")?;

            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");
        Ok(())
    }

    pub async fn download_with_checksum(&self, url: &str, dest: &Path, expected_checksum: Option<&str>) -> Result<()> {
        self.download_file(url, dest).await?;

        if let Some(expected) = expected_checksum {
            let actual = self.calculate_sha256(dest).await?;
            if actual != expected {
                anyhow::bail!("Checksum mismatch: expected {}, got {}", expected, actual);
            }
        }

        Ok(())
    }

    async fn calculate_sha256(&self, path: &Path) -> Result<String> {
        let mut file = tokio::fs::File::open(path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0; 8192];

        use tokio::io::AsyncReadExt;

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }
}

pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub prerelease: bool,
    pub assets: Vec<GitHubAsset>,
}

pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub struct GitHubClient {
    client: Client,
}

impl GitHubClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("pkmgr/1.0.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<GitHubRelease> {
        let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        self.parse_release(response)
    }

    pub async fn get_releases(&self, owner: &str, repo: &str) -> Result<Vec<GitHubRelease>> {
        let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<Vec<serde_json::Value>>()
            .await?;

        response.into_iter()
            .map(|r| self.parse_release(r))
            .collect()
    }

    fn parse_release(&self, value: serde_json::Value) -> Result<GitHubRelease> {
        let tag_name = value["tag_name"].as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing tag_name"))?
            .to_string();

        let name = value["name"].as_str()
            .unwrap_or(&tag_name)
            .to_string();

        let prerelease = value["prerelease"].as_bool()
            .unwrap_or(false);

        let assets = value["assets"].as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing assets"))?
            .iter()
            .filter_map(|asset| {
                Some(GitHubAsset {
                    name: asset["name"].as_str()?.to_string(),
                    browser_download_url: asset["browser_download_url"].as_str()?.to_string(),
                    size: asset["size"].as_u64()?,
                })
            })
            .collect();

        Ok(GitHubRelease {
            tag_name,
            name,
            prerelease,
            assets,
        })
    }

    pub fn select_asset<'a>(&self, release: &'a GitHubRelease, platform: &str, arch: &str) -> Option<&'a GitHubAsset> {
        // Priority order for asset selection
        let patterns = vec![
            format!("{}-{}-static", platform, arch),
            format!("{}-{}-musl", platform, arch),
            format!("{}_{}", platform, arch),
            format!("{}-{}", platform, arch),
            platform.to_string(),
        ];

        for pattern in patterns {
            if let Some(asset) = release.assets.iter().find(|a| {
                a.name.to_lowercase().contains(&pattern.to_lowercase())
            }) {
                return Some(asset);
            }
        }

        // Fallback: try to find any binary that might work
        release.assets.iter().find(|a| {
            let name = a.name.to_lowercase();
            !name.ends_with(".txt") &&
            !name.ends_with(".md") &&
            !name.ends_with(".sha256") &&
            !name.ends_with(".sig")
        })
    }
}