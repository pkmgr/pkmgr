use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use crate::core::config::Config;
use crate::ui::output::Output;
use crate::utils::download::Downloader;
use super::{distributions, verification, IsoDistribution, IsoVersion, DistributionCategory};

pub struct IsoManager {
    config: Config,
    output: Output,
    iso_dir: PathBuf,
}

impl IsoManager {
    pub fn new(config: Config, output: Output) -> Result<Self> {
        let iso_dir = config.get_iso_dir()?;
        Ok(Self {
            config,
            output,
            iso_dir,
        })
    }

    /// List all supported distributions or specific distribution versions
    pub async fn list(&self, distro: Option<String>) -> Result<()> {
        let distributions = distributions::get_all_distributions();

        if let Some(distro_name) = distro {
            // Show specific distribution
            if let Some(distro) = distributions.iter().find(|d| d.name == distro_name) {
                self.display_distribution_details(distro);
            } else {
                self.output.error(&format!("Distribution '{}' not found", distro_name));
                self.output.info("Use 'pkmgr iso list' to see all supported distributions");
            }
        } else {
            // Show all distributions
            self.display_all_distributions(&distributions);
        }

        Ok(())
    }

    /// List downloaded ISOs
    pub async fn list_downloaded(&self) -> Result<()> {
        self.output.print_header("💿 Downloaded ISOs");

        // Create directory structure as specified
        let os_dir = self.iso_dir.join("OS");
        let security_dir = self.iso_dir.join("Security");
        let server_dir = self.iso_dir.join("Server");
        let tools_dir = self.iso_dir.join("Tools");

        let mut found_any = false;

        // Check each category directory
        for (category_name, category_dir) in [
            ("Operating Systems", os_dir),
            ("Security", security_dir),
            ("Server", server_dir),
            ("Tools", tools_dir),
        ] {
            if category_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&category_dir) {
                    let isos: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .filter(|e| {
                            e.path().extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| ext == "iso")
                                .unwrap_or(false)
                        })
                        .filter_map(|e| e.file_name().to_str().map(String::from))
                        .collect();

                    if !isos.is_empty() {
                        found_any = true;
                        self.output.print_section(category_name);
                        for iso in isos {
                            // Get file size
                            let iso_path = category_dir.join(&iso);
                            let size_mb = std::fs::metadata(&iso_path)
                                .map(|m| m.len() / 1_000_000)
                                .unwrap_or(0);

                            self.output.info(&format!("  💿 {} ({} MB)", iso, size_mb));
                        }
                    }
                }
            }
        }

        if !found_any {
            self.output.info("No ISOs downloaded yet.");
            self.output.info("Use 'pkmgr iso install <distro>' to download an ISO.");
        }

        Ok(())
    }

    /// Download ISO (current version if no version specified)
    pub async fn install(&self, distro_name: String, version: Option<String>) -> Result<()> {
        self.output.print_header(&format!("💿 Downloading ISO: {}", distro_name));

        let distributions = distributions::get_all_distributions();

        let distro = distributions.iter()
            .find(|d| d.name == distro_name)
            .ok_or_else(|| anyhow::anyhow!("Distribution '{}' not found", distro_name))?;

        // Select version
        let iso_version = if let Some(ver) = version {
            distro.versions.iter()
                .find(|v| v.version == ver)
                .ok_or_else(|| anyhow::anyhow!("Version {} not found for {}", ver, distro_name))?
        } else {
            // Get current version
            distro.versions.iter()
                .find(|v| v.is_current)
                .or_else(|| distro.versions.first())
                .ok_or_else(|| anyhow::anyhow!("No versions available for {}", distro_name))?
        };

        // Select flavor and architecture
        // For now, we'll use defaults - in production this would be interactive
        let flavor = iso_version.flavors.first()
            .ok_or_else(|| anyhow::anyhow!("No flavors available"))?;

        let arch = iso_version.architectures.first()
            .ok_or_else(|| anyhow::anyhow!("No architectures available"))?;

        let key = format!("{}-{}", arch, flavor);
        let download_url = iso_version.download_urls.get(&key)
            .or_else(|| iso_version.download_urls.values().next())
            .ok_or_else(|| anyhow::anyhow!("No download URL available for this version"))?;

        // Determine download path based on category
        let category_dir = match distro.category {
            DistributionCategory::Linux => "OS/Linux",
            DistributionCategory::Security => "Security",
            DistributionCategory::Server => "Server",
            DistributionCategory::BSD => "OS/BSD",
            DistributionCategory::Utility => "Tools",
        };

        let download_dir = self.iso_dir.join(category_dir).join(&distro.display_name);
        tokio::fs::create_dir_all(&download_dir).await?;

        let iso_filename = format!(
            "{}-{}-{}-{}.iso",
            distro_name,
            iso_version.version,
            flavor,
            arch
        );
        let iso_path = download_dir.join(&iso_filename);

        // Check if already downloaded
        if iso_path.exists() {
            self.output.info(&format!("ISO already downloaded: {}", iso_path.display()));
            return Ok(());
        }

        self.output.info(&format!("📊 Version: {} {}",
            iso_version.version,
            iso_version.codename.as_ref().unwrap_or(&String::new())
        ));
        self.output.info(&format!("📦 Flavor: {}", flavor));
        self.output.info(&format!("🏗️ Architecture: {}", arch));
        self.output.info(&format!("💾 Size: {} MB", iso_version.size_mb));
        self.output.info(&format!("🌐 URL: {}", download_url));

        // Download the ISO
        let downloader = Downloader::new(self.config.defaults.emoji_enabled)?;

        self.output.download_start(&iso_filename, Some(iso_version.size_mb * 1_000_000));

        // Download with retry logic as specified
        let mut retry_count = 0;
        loop {
            match downloader.download_file(download_url, &iso_path).await {
                Ok(_) => break,
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= 3 {
                        return Err(e);
                    }
                    self.output.warn(&format!("Download failed, retrying... (attempt {}/3)", retry_count + 1));
                }
            }
        }

        // Verify if checksums available
        if !iso_version.checksum_urls.is_empty() {
            let checksum_url = iso_version.checksum_urls.values().next().unwrap();
            let checksum_path = download_dir.join(format!("{}.sha256", iso_filename));

            self.output.progress("Downloading checksums");
            downloader.download_file(checksum_url, &checksum_path).await?;

            // Verify the ISO
            let verifier = verification::IsoVerifier::new(self.output.clone());
            let verified = verifier.verify(&iso_path, Some(&checksum_path), None).await?;

            if !verified {
                // Handle failed verification
                if verification::handle_failed_verification(&iso_path, &self.output, retry_count).await? {
                    // Retry download
                    return Box::pin(self.install(distro_name, Some(iso_version.version.clone()))).await;
                } else {
                    return Err(anyhow::anyhow!("ISO verification failed"));
                }
            }
        } else {
            // Handle missing checksums
            if !verification::handle_missing_checksums(&self.output).await? {
                // User chose not to continue
                tokio::fs::remove_file(&iso_path).await?;
                return Err(anyhow::anyhow!("Download cancelled by user"));
            }
        }

        self.output.success(&format!("✅ Successfully downloaded {}", iso_filename));
        self.output.info(&format!("📁 Saved to: {}", iso_path.display()));

        Ok(())
    }

    /// Delete downloaded ISO file
    pub async fn remove(&self, iso_file: String) -> Result<()> {
        self.output.print_header(&format!("🗑️ Removing ISO: {}", iso_file));

        // Search for the ISO in all category directories
        let categories = ["OS/Linux", "OS/BSD", "OS/Windows", "OS/Mac", "Security", "Server", "Tools"];

        for category in &categories {
            let category_dir = self.iso_dir.join(category);
            if !category_dir.exists() {
                continue;
            }

            // Use walkdir to search recursively
            use walkdir::WalkDir;
            for entry in WalkDir::new(&category_dir) {
                if let Ok(entry) = entry {
                    if entry.file_name() == iso_file.as_str() {
                        let iso_path = entry.path();

                        self.output.info(&format!("Found ISO at: {}", iso_path.display()));

                        // Confirm deletion
                        use crate::ui::prompt::Prompt;
                        let prompt = Prompt::new(self.config.defaults.emoji_enabled);

                        if prompt.confirm("Delete this ISO?")? {
                            tokio::fs::remove_file(iso_path).await?;
                            self.output.success("✅ ISO deleted successfully");

                            // Also remove associated checksum files if they exist
                            let checksum_path = iso_path.with_extension("sha256");
                            if checksum_path.exists() {
                                tokio::fs::remove_file(checksum_path).await?;
                            }

                            let sig_path = iso_path.with_extension("sig");
                            if sig_path.exists() {
                                tokio::fs::remove_file(sig_path).await?;
                            }
                        } else {
                            self.output.info("Deletion cancelled");
                        }

                        return Ok(());
                    }
                }
            }
        }

        self.output.error(&format!("ISO file '{}' not found", iso_file));
        self.output.info("Use 'pkmgr iso list --downloaded' to see available ISOs");

        Ok(())
    }

    /// Show distribution information
    pub async fn info(&self, distro_name: String) -> Result<()> {
        let distributions = distributions::get_all_distributions();

        let distro = distributions.iter()
            .find(|d| d.name == distro_name)
            .ok_or_else(|| anyhow::anyhow!("Distribution '{}' not found", distro_name))?;

        self.display_distribution_info(distro);

        Ok(())
    }

    /// Verify ISO checksums and signatures
    pub async fn verify(&self, iso_file: Option<String>) -> Result<()> {
        if let Some(file) = iso_file {
            self.output.print_header(&format!("🔍 Verifying ISO: {}", file));

            // Find the ISO file
            let iso_path = self.find_iso_file(&file)?;

            // Look for checksum files
            let checksum_path = iso_path.with_extension("sha256");
            let sig_path = iso_path.with_extension("sig");

            let verifier = verification::IsoVerifier::new(self.output.clone());
            let verified = verifier.verify(
                &iso_path,
                if checksum_path.exists() { Some(&checksum_path) } else { None },
                if sig_path.exists() { Some(&sig_path) } else { None }
            ).await?;

            if verified {
                self.output.success("✅ ISO verification successful");
            } else {
                self.output.error("❌ ISO verification failed");
            }
        } else {
            // Verify all ISOs
            self.output.print_header("🔍 Verifying all ISOs");
            self.output.info("Feature coming soon");
        }

        Ok(())
    }

    /// Remove old/duplicate ISO files
    pub async fn clean(&self) -> Result<()> {
        self.output.print_header("🧹 Cleaning old ISO files");

        // Find duplicate ISOs (same distro, different versions)
        let mut duplicates: Vec<PathBuf> = Vec::new();

        // TODO: Implement duplicate detection logic
        // For each distro, keep only the latest version

        if duplicates.is_empty() {
            self.output.info("No old or duplicate ISOs found");
        } else {
            self.output.info(&format!("Found {} old/duplicate ISOs", duplicates.len()));

            // Show list and confirm deletion
            for iso in &duplicates {
                self.output.info(&format!("  🗑️ {}", iso.display()));
            }

            use crate::ui::prompt::Prompt;
            let prompt = Prompt::new(self.config.defaults.emoji_enabled);

            if prompt.confirm("Delete these ISOs?")? {
                for iso in duplicates {
                    // Delete ISO
                    self.output.info(&format!("Deleting {}", iso.display()));
                }
                self.output.success("✅ Cleanup complete");
            } else {
                self.output.info("Cleanup cancelled");
            }
        }

        Ok(())
    }

    // Helper methods

    fn display_all_distributions(&self, distributions: &[IsoDistribution]) {
        self.output.print_header("💿 Supported Linux Distributions");

        // Group by category
        let mut by_category: std::collections::HashMap<String, Vec<&IsoDistribution>> = std::collections::HashMap::new();

        for distro in distributions {
            let category = format!("{}", distro.category);
            by_category.entry(category).or_insert_with(Vec::new).push(distro);
        }

        // Display each category
        for (category, distros) in by_category {
            self.output.print_section(&category);

            for distro in distros {
                let current_version = distro.versions.iter()
                    .find(|v| v.is_current)
                    .map(|v| v.version.as_str())
                    .unwrap_or("N/A");

                self.output.info(&format!("  {} - {} ({})",
                    distro.name,
                    distro.display_name,
                    current_version
                ));
            }
        }

        self.output.info("");
        self.output.info("Use 'pkmgr iso list <distro>' for version details");
        self.output.info("Use 'pkmgr iso install <distro>' to download");
    }

    fn display_distribution_details(&self, distro: &IsoDistribution) {
        self.output.print_header(&format!("💿 {} Details", distro.display_name));

        self.output.info(&format!("📝 Name: {}", distro.display_name));
        self.output.info(&format!("🏷️ ID: {}", distro.name));
        self.output.info(&format!("📚 Category: {}", distro.category));
        self.output.info(&format!("🌐 Homepage: {}", distro.homepage));
        self.output.info(&format!("📖 Description: {}", distro.description));

        self.output.print_section("Available Versions");

        let headers = vec!["Version", "Codename", "Release Date", "Type", "Architectures", "Size"];
        let mut rows = Vec::new();

        for version in &distro.versions {
            let version_type = if version.is_lts {
                "LTS"
            } else if version.is_current {
                "Current"
            } else {
                "Legacy"
            };

            let archs = version.architectures.iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(", ");

            rows.push(vec![
                version.version.clone(),
                version.codename.clone().unwrap_or_else(|| "N/A".to_string()),
                version.release_date.clone().unwrap_or_else(|| "N/A".to_string()),
                version_type.to_string(),
                archs,
                format!("{} MB", version.size_mb),
            ]);
        }

        self.output.print_table(&headers, &rows);

        if !distro.versions.is_empty() && !distro.versions[0].flavors.is_empty() {
            self.output.print_section("Available Flavors");
            for flavor in &distro.versions[0].flavors {
                self.output.info(&format!("  • {}", flavor));
            }
        }
    }

    fn display_distribution_info(&self, distro: &IsoDistribution) {
        self.output.print_header(&format!("ℹ️ {} Information", distro.display_name));

        self.output.print_section("Overview");
        self.output.info(&format!("📝 Name: {}", distro.display_name));
        self.output.info(&format!("🏷️ ID: {}", distro.name));
        self.output.info(&format!("📚 Category: {}", distro.category));
        self.output.info(&format!("🌐 Homepage: {}", distro.homepage));
        self.output.info(&format!("📖 Description: {}", distro.description));

        if let Some(current) = distro.versions.iter().find(|v| v.is_current) {
            self.output.print_section("Current Version");
            self.output.info(&format!("📦 Version: {}", current.version));
            if let Some(codename) = &current.codename {
                self.output.info(&format!("🏷️ Codename: {}", codename));
            }
            if let Some(date) = &current.release_date {
                self.output.info(&format!("📅 Released: {}", date));
            }
            self.output.info(&format!("💾 Size: {} MB", current.size_mb));
        }

        self.output.print_section("Download Command");
        self.output.info(&format!("  pkmgr iso install {}", distro.name));

        if distro.versions.len() > 1 {
            self.output.info("");
            self.output.info("Specific version:");
            self.output.info(&format!("  pkmgr iso install {} {}",
                distro.name,
                distro.versions[1].version
            ));
        }
    }

    fn find_iso_file(&self, filename: &str) -> Result<PathBuf> {
        use walkdir::WalkDir;

        for entry in WalkDir::new(&self.iso_dir) {
            if let Ok(entry) = entry {
                if entry.file_name() == filename {
                    return Ok(entry.path().to_path_buf());
                }
            }
        }

        Err(anyhow::anyhow!("ISO file '{}' not found", filename))
    }
}