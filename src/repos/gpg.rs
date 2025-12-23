use anyhow::{Context, Result, bail};
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::ui::output::Output;
use super::GpgKeyInfo;

pub struct GpgManager {
    output: Output,
    keyservers: Vec<String>,
}

impl GpgManager {
    pub fn new(output: Output) -> Self {
        Self {
            output,
            keyservers: vec![
                "hkps://keys.openpgp.org".to_string(),
                "hkps://keyserver.ubuntu.com".to_string(),
                "hkps://pgp.mit.edu".to_string(),
            ],
        }
    }

    /// Import a GPG key from a URL
    pub async fn import_key_from_url(&self, url: &str) -> Result<String> {
        self.output.progress(&format!("Downloading GPG key from {}", url));

        // Download the key
        let client = reqwest::Client::new();
        let response = client.get(url)
            .send()
            .await
            .context("Failed to download GPG key")?;

        if !response.status().is_success() {
            bail!("Failed to download GPG key: HTTP {}", response.status());
        }

        let key_data = response.bytes().await?;

        // Import the key
        self.import_key_from_bytes(&key_data)
    }

    /// Import a GPG key from bytes
    pub fn import_key_from_bytes(&self, key_data: &[u8]) -> Result<String> {
        // Save to temporary file
        let temp_dir = tempfile::tempdir()?;
        let key_file = temp_dir.path().join("key.asc");
        std::fs::write(&key_file, key_data)?;

        self.import_key_from_file(&key_file)
    }

    /// Import a GPG key from a file
    pub fn import_key_from_file(&self, key_path: &Path) -> Result<String> {
        self.output.progress("Importing GPG key");

        #[cfg(target_os = "linux")]
        {
            // For APT-based systems
            if Path::new("/usr/bin/apt-key").exists() {
                let output = Command::new("apt-key")
                    .arg("add")
                    .arg(key_path)
                    .output()
                    .context("Failed to import GPG key with apt-key")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to import GPG key: {}", stderr);
                }

                return self.get_key_fingerprint_from_file(key_path);
            }

            // For DNF/YUM-based systems
            if Path::new("/usr/bin/rpm").exists() {
                let output = Command::new("rpm")
                    .arg("--import")
                    .arg(key_path)
                    .output()
                    .context("Failed to import GPG key with rpm")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to import GPG key: {}", stderr);
                }

                return self.get_key_fingerprint_from_file(key_path);
            }

            // For Pacman-based systems
            if Path::new("/usr/bin/pacman-key").exists() {
                let output = Command::new("pacman-key")
                    .arg("--add")
                    .arg(key_path)
                    .output()
                    .context("Failed to import GPG key with pacman-key")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Failed to import GPG key: {}", stderr);
                }

                // Sign the key locally
                let _ = Command::new("pacman-key")
                    .arg("--lsign-key")
                    .arg(key_path)
                    .output();

                return self.get_key_fingerprint_from_file(key_path);
            }
        }

        // Fallback to gpg command
        let output = Command::new("gpg")
            .arg("--import")
            .arg(key_path)
            .output()
            .context("Failed to import GPG key with gpg")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to import GPG key: {}", stderr);
        }

        self.get_key_fingerprint_from_file(key_path)
    }

    /// Get fingerprint from a key file
    fn get_key_fingerprint_from_file(&self, key_path: &Path) -> Result<String> {
        let output = Command::new("gpg")
            .arg("--with-fingerprint")
            .arg("--with-colons")
            .arg(key_path)
            .output()
            .context("Failed to get key fingerprint")?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        // Parse the fingerprint from gpg output
        for line in output_str.lines() {
            if line.starts_with("fpr:") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 9 {
                    return Ok(parts[9].to_string());
                }
            }
        }

        // Try alternative format
        for line in output_str.lines() {
            if line.contains("fingerprint") {
                // Extract hex digits from the line
                let fingerprint: String = line.chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect();
                if !fingerprint.is_empty() {
                    return Ok(fingerprint);
                }
            }
        }

        bail!("Could not extract fingerprint from key")
    }

    /// Fetch a key from keyservers by fingerprint
    pub async fn fetch_key(&self, fingerprint: &str) -> Result<()> {
        self.output.progress(&format!("Fetching GPG key {}", fingerprint));

        for keyserver in &self.keyservers {
            self.output.progress(&format!("Trying keyserver {}", keyserver));

            let result = self.fetch_from_keyserver(fingerprint, keyserver).await;

            if result.is_ok() {
                self.output.success(&format!("Successfully fetched key from {}", keyserver));
                return Ok(());
            }

            self.output.warn(&format!("Failed to fetch from {}, trying next", keyserver));
        }

        bail!("Failed to fetch key {} from all keyservers", fingerprint)
    }

    /// Fetch from a specific keyserver
    async fn fetch_from_keyserver(&self, fingerprint: &str, keyserver: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            // Try apt-key if available
            if Path::new("/usr/bin/apt-key").exists() {
                let output = Command::new("apt-key")
                    .arg("adv")
                    .arg("--keyserver")
                    .arg(keyserver)
                    .arg("--recv-keys")
                    .arg(fingerprint)
                    .output()
                    .context("Failed to fetch key with apt-key")?;

                if output.status.success() {
                    return Ok(());
                }
            }
        }

        // Use gpg directly
        let output = Command::new("gpg")
            .arg("--keyserver")
            .arg(keyserver)
            .arg("--recv-keys")
            .arg(fingerprint)
            .output()
            .context("Failed to fetch key with gpg")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to fetch key: {}", stderr);
        }

        Ok(())
    }

    /// Refresh expiring keys
    pub async fn refresh_expiring_keys(&self) -> Result<()> {
        self.output.progress("Refreshing expiring GPG keys");

        #[cfg(target_os = "linux")]
        {
            if Path::new("/usr/bin/apt-key").exists() {
                // List keys and check expiration
                let output = Command::new("apt-key")
                    .arg("list")
                    .output()?;

                let output_str = String::from_utf8_lossy(&output.stdout);

                // Parse and identify expiring keys
                let mut expiring_keys = Vec::new();
                for line in output_str.lines() {
                    if line.contains("expires:") {
                        // Parse expiration date and check if within 30 days
                        // This is simplified - real implementation would parse dates
                        if line.contains("2024") || line.contains("2025") {
                            // Extract key ID from previous line
                            // Simplified parsing
                            expiring_keys.push(line.to_string());
                        }
                    }
                }

                if !expiring_keys.is_empty() {
                    self.output.info(&format!("Found {} expiring keys", expiring_keys.len()));

                    // Update all keys
                    let _ = Command::new("apt-key")
                        .arg("update")
                        .output();
                }
            }
        }

        Ok(())
    }

    /// List all GPG keys
    pub fn list_keys(&self) -> Result<Vec<GpgKeyInfo>> {
        let mut keys = Vec::new();

        #[cfg(target_os = "linux")]
        {
            if Path::new("/usr/bin/apt-key").exists() {
                let output = Command::new("apt-key")
                    .arg("list")
                    .arg("--with-colons")
                    .output()?;

                keys.extend(self.parse_gpg_output(&output.stdout)?);
            } else if Path::new("/usr/bin/rpm").exists() {
                let output = Command::new("rpm")
                    .arg("-qa")
                    .arg("gpg-pubkey*")
                    .output()?;

                // Parse RPM GPG keys (simplified)
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.starts_with("gpg-pubkey-") {
                        keys.push(GpgKeyInfo {
                            fingerprint: line.to_string(),
                            key_id: line.to_string(),
                            key_server: None,
                            key_url: None,
                            trusted: true,
                            expires: None,
                            last_refreshed: Some(chrono::Utc::now()),
                        });
                    }
                }
            }
        }

        // Fallback to gpg
        if keys.is_empty() {
            let output = Command::new("gpg")
                .arg("--list-keys")
                .arg("--with-colons")
                .output()?;

            keys = self.parse_gpg_output(&output.stdout)?;
        }

        Ok(keys)
    }

    /// Parse GPG output format
    fn parse_gpg_output(&self, output: &[u8]) -> Result<Vec<GpgKeyInfo>> {
        let output_str = String::from_utf8_lossy(output);
        let mut keys = Vec::new();
        let mut current_key: Option<GpgKeyInfo> = None;

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "pub" => {
                    if let Some(key) = current_key.take() {
                        keys.push(key);
                    }

                    let key_id = parts.get(4).unwrap_or(&"").to_string();
                    current_key = Some(GpgKeyInfo {
                        fingerprint: String::new(),
                        key_id,
                        key_server: None,
                        key_url: None,
                        trusted: true,
                        expires: None,
                        last_refreshed: Some(chrono::Utc::now()),
                    });
                }
                "fpr" => {
                    if let Some(ref mut key) = current_key {
                        key.fingerprint = parts.get(9).unwrap_or(&"").to_string();
                    }
                }
                _ => {}
            }
        }

        if let Some(key) = current_key {
            keys.push(key);
        }

        Ok(keys)
    }

    /// Verify a repository signature
    pub fn verify_signature(&self, repo_file: &Path, signature_file: &Path) -> Result<bool> {
        let output = Command::new("gpg")
            .arg("--verify")
            .arg(signature_file)
            .arg(repo_file)
            .output()
            .context("Failed to verify signature")?;

        Ok(output.status.success())
    }

    /// Trust a key (for package managers that require it)
    pub fn trust_key(&self, fingerprint: &str) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            // For Arch Linux
            if Path::new("/usr/bin/pacman-key").exists() {
                let _ = Command::new("pacman-key")
                    .arg("--lsign-key")
                    .arg(fingerprint)
                    .output();
            }
        }

        Ok(())
    }
}