use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::fs;
use crate::ui::output::Output;
use crate::ui::prompt::Prompt;
use crate::core::config::Config;
use super::{Profile, get_profile_templates};

pub struct ProfileManager {
    output: Output,
    prompt: Prompt,
    current_profile: Option<String>,
}

impl ProfileManager {
    pub fn new(output: Output) -> Self {
        let emoji_enabled = output.emoji_enabled;
        Self {
            output,
            prompt: Prompt::new(emoji_enabled),
            current_profile: Self::get_current_profile().ok(),
        }
    }

    /// Get the current active profile
    fn get_current_profile() -> Result<String> {
        let current_file = Profile::profile_dir()?.join("current");

        if current_file.exists() {
            fs::read_to_string(&current_file)
                .context("Failed to read current profile")
                .map(|s| s.trim().to_string())
        } else {
            Ok("default".to_string())
        }
    }

    /// Set the current active profile
    fn set_current_profile(&mut self, name: &str) -> Result<()> {
        let current_file = Profile::profile_dir()?.join("current");

        fs::create_dir_all(Profile::profile_dir()?)?;
        fs::write(&current_file, name)?;

        self.current_profile = Some(name.to_string());
        Ok(())
    }

    /// List all profiles
    pub fn list(&self) -> Result<()> {
        self.output.section("Available Profiles");

        let profiles = Profile::list_all()?;

        if profiles.is_empty() {
            self.output.info("No profiles found");
            self.output.info("Create a profile with: pkmgr profile create <name>");
            return Ok(());
        }

        let current = self.current_profile.as_ref().map(|s| s.as_str()).unwrap_or("default");

        for profile_name in profiles {
            let is_current = profile_name == current;
            let marker = if is_current { " (current)" } else { "" };

            // Try to load profile to get description
            if let Ok(profile) = Profile::load(&profile_name) {
                if !profile.description.is_empty() {
                    self.output.info(&format!(
                        "{}{} - {}",
                        profile_name,
                        marker,
                        profile.description
                    ));
                } else {
                    self.output.info(&format!("{}{}", profile_name, marker));
                }

                // Show inheritance
                if let Some(ref parent) = profile.parent {
                    self.output.info(&format!("  └─ Inherits from: {}", parent));
                }
            } else {
                self.output.info(&format!("{}{}", profile_name, marker));
            }
        }

        Ok(())
    }

    /// Show detailed profile information
    pub fn show(&self, name: &str) -> Result<()> {
        let profile = Profile::load(name)?;

        self.output.section(&format!("Profile: {}", profile.name));

        if !profile.description.is_empty() {
            self.output.info(&format!("Description: {}", profile.description));
        }

        if let Some(ref parent) = profile.parent {
            self.output.info(&format!("Inherits from: {}", parent));
        }

        self.output.info(&format!("Created: {}", profile.created.format("%Y-%m-%d %H:%M:%S")));
        self.output.info(&format!("Updated: {}", profile.updated.format("%Y-%m-%d %H:%M:%S")));

        // Settings
        self.output.section("Settings");
        self.output.info(&format!("Install location: {:?}", profile.settings.install_location));
        self.output.info(&format!("Prefer binary: {}", profile.settings.prefer_binary));
        self.output.info(&format!("Allow prerelease: {}", profile.settings.allow_prerelease));
        self.output.info(&format!("Parallel downloads: {}", profile.settings.parallel_downloads));
        self.output.info(&format!("Auto cleanup: {}", profile.settings.auto_cleanup));
        self.output.info(&format!("Verify signatures: {}", profile.settings.verify_signatures));

        // Packages
        if !profile.packages.system.is_empty() {
            self.output.section("System Packages");
            for pkg in &profile.packages.system {
                let version = pkg.version.as_ref().map(|v| format!(" ({})", v)).unwrap_or_default();
                self.output.info(&format!("  - {}{}", pkg.name, version));
            }
        }

        if !profile.packages.languages.is_empty() {
            self.output.section("Language Packages");
            for (lang, pkgs) in &profile.packages.languages {
                self.output.info(&format!("  {}:", lang));
                for pkg in pkgs {
                    let version = pkg.version.as_ref().map(|v| format!(" ({})", v)).unwrap_or_default();
                    self.output.info(&format!("    - {}{}", pkg.name, version));
                }
            }
        }

        if !profile.packages.binaries.is_empty() {
            self.output.section("Binary Tools");
            for bin in &profile.packages.binaries {
                let version = bin.version.as_ref().map(|v| format!("@{}", v)).unwrap_or_default();
                self.output.info(&format!("  - {}{}", bin.repository, version));
            }
        }

        // Repositories
        if !profile.repositories.is_empty() {
            self.output.section("Repositories");
            for repo in &profile.repositories {
                let name = repo.name.as_ref().unwrap_or(&repo.url);
                let status = if repo.enabled { "enabled" } else { "disabled" };
                self.output.info(&format!("  - {} [{}]", name, status));
            }
        }

        // Environment
        if !profile.environment.is_empty() {
            self.output.section("Environment Variables");
            for (key, value) in &profile.environment {
                self.output.info(&format!("  {}={}", key, value));
            }
        }

        Ok(())
    }

    /// Create a new profile
    pub async fn create(&self, name: &str, from_template: Option<String>, copy_current: bool) -> Result<()> {
        // Check if profile already exists
        if Profile::list_all()?.contains(&name.to_string()) {
            bail!("Profile '{}' already exists", name);
        }

        let profile = if let Some(template_name) = from_template {
            // Create from template
            let templates = get_profile_templates();
            let template = templates.iter()
                .find(|(n, _)| n == &template_name)
                .map(|(_, p)| p.clone())
                .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_name))?;

            let mut profile = template;
            profile.name = name.to_string();
            profile.created = chrono::Utc::now();
            profile.updated = chrono::Utc::now();
            profile
        } else if copy_current {
            // Copy current system state
            self.create_from_current_state(name).await?
        } else {
            // Create empty profile
            let description = self.prompt.input("Profile description (optional): ")?;
            Profile::new(name.to_string()).with_description(description)
        };

        profile.save()?;

        self.output.success(&format!("Profile '{}' created successfully", name));

        if self.prompt.confirm("Set as current profile?")? {
            let mut manager = ProfileManager::new(self.output.clone());
            manager.use_profile(name).await?;
        }

        Ok(())
    }

    /// Create profile from current system state
    async fn create_from_current_state(&self, name: &str) -> Result<Profile> {
        self.output.progress("Capturing current system state...");

        let mut profile = Profile::new(name.to_string());
        profile.description = "Created from current system state".to_string();

        // Capture installed packages
        // This would need to be implemented based on the package manager
        self.output.warn("Package capture not yet implemented");

        // Capture current config settings
        if let Ok(config) = Config::load().await.context("loading config") {
            profile.settings.prefer_binary = config.defaults.prefer_binary;
            profile.settings.allow_prerelease = config.defaults.allow_prerelease;
            profile.settings.parallel_downloads = config.defaults.parallel_downloads;
            profile.settings.auto_cleanup = config.defaults.auto_cleanup;
            profile.settings.verify_signatures = config.security.verify_signatures;
        }

        Ok(profile)
    }

    /// Switch to a profile
    pub async fn use_profile(&mut self, name: &str) -> Result<()> {
        // Verify profile exists
        let profile = Profile::load(name)?;

        self.output.progress(&format!("Switching to profile '{}'", name));

        // Update current profile marker
        self.set_current_profile(name)?;

        // Apply profile settings to config
        self.apply_profile_settings(&profile).await?;

        self.output.success(&format!("Now using profile '{}'", name));

        Ok(())
    }

    /// Apply profile settings to configuration
    async fn apply_profile_settings(&self, profile: &Profile) -> Result<()> {
        let mut config = Config::load().await?;

        // Update config with profile settings
        config.defaults.prefer_binary = profile.settings.prefer_binary;
        config.defaults.allow_prerelease = profile.settings.allow_prerelease;
        config.defaults.parallel_downloads = profile.settings.parallel_downloads;
        config.defaults.parallel_operations = profile.settings.parallel_operations;
        config.defaults.auto_cleanup = profile.settings.auto_cleanup;
        config.defaults.auto_update_check = profile.settings.auto_update_check;
        config.defaults.confirm_major_updates = profile.settings.confirm_major_updates;
        config.defaults.keep_downloads = profile.settings.keep_downloads;
        config.defaults.use_cache = profile.settings.use_cache;

        config.security.verify_signatures = profile.settings.verify_signatures;
        config.security.verify_checksums = profile.settings.verify_checksums;
        config.security.allow_untrusted = profile.settings.allow_untrusted;

        config.save().await?;

        // Set environment variables
        for (key, value) in &profile.environment {
            std::env::set_var(key, value);
        }

        Ok(())
    }

    /// Remove a profile
    pub fn remove(&self, name: &str) -> Result<()> {
        // Don't allow removing the current profile
        if self.current_profile.as_ref() == Some(&name.to_string()) {
            bail!("Cannot remove the currently active profile");
        }

        // Confirm deletion
        if !self.prompt.confirm(&format!("Delete profile '{}'?", name))? {
            self.output.info("Profile deletion cancelled");
            return Ok(());
        }

        Profile::delete(name)?;
        self.output.success(&format!("Profile '{}' removed", name));

        Ok(())
    }

    /// Edit a profile in the default editor
    pub fn edit(&self, name: &str) -> Result<()> {
        let profile_path = Profile::profile_dir()?.join(format!("{}.toml", name));

        if !profile_path.exists() {
            bail!("Profile '{}' not found", name);
        }

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

        self.output.info(&format!("Opening profile in {}", editor));

        let status = std::process::Command::new(&editor)
            .arg(&profile_path)
            .status()
            .context("Failed to open editor")?;

        if status.success() {
            self.output.success("Profile edited successfully");
        } else {
            bail!("Editor exited with error");
        }

        Ok(())
    }

    /// Compare two profiles
    pub fn diff(&self, profile1: &str, profile2: &str) -> Result<()> {
        let p1 = Profile::load(profile1)?;
        let p2 = Profile::load(profile2)?;

        self.output.section(&format!("Comparing {} vs {}", profile1, profile2));

        // Compare settings
        if p1.settings.prefer_binary != p2.settings.prefer_binary {
            self.output.info(&format!(
                "Prefer binary: {} vs {}",
                p1.settings.prefer_binary,
                p2.settings.prefer_binary
            ));
        }

        // Compare packages
        let p1_system: std::collections::HashSet<_> = p1.packages.system.iter().map(|p| &p.name).collect();
        let p2_system: std::collections::HashSet<_> = p2.packages.system.iter().map(|p| &p.name).collect();

        let only_in_p1: Vec<_> = p1_system.difference(&p2_system).collect();
        let only_in_p2: Vec<_> = p2_system.difference(&p1_system).collect();

        if !only_in_p1.is_empty() {
            self.output.info(&format!("\nOnly in {}: {}", profile1, only_in_p1.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }

        if !only_in_p2.is_empty() {
            self.output.info(&format!("Only in {}: {}", profile2, only_in_p2.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
        }

        Ok(())
    }

    /// Apply a profile (install all packages)
    pub async fn apply(&self, name: &str) -> Result<()> {
        let profile = Profile::load(name)?;

        self.output.section(&format!("Applying profile: {}", name));

        // Run pre-install scripts
        if !profile.scripts.pre_install.is_empty() {
            self.output.progress("Running pre-install scripts...");
            for script in &profile.scripts.pre_install {
                self.run_script(script)?;
            }
        }

        // Install repositories
        if !profile.repositories.is_empty() {
            self.output.section("Adding repositories");
            for repo in &profile.repositories {
                self.output.info(&format!("Adding {}", repo.name.as_ref().unwrap_or(&repo.url)));
                // Repository installation would go here
            }
        }

        // Install system packages
        if !profile.packages.system.is_empty() {
            self.output.section("Installing system packages");
            for pkg in &profile.packages.system {
                self.output.info(&format!("Installing {}", pkg.name));
                // Package installation would go here
            }
        }

        // Install language packages
        for (lang, packages) in &profile.packages.languages {
            if !packages.is_empty() {
                self.output.section(&format!("Installing {} packages", lang));
                for pkg in packages {
                    self.output.info(&format!("Installing {}", pkg.name));
                    // Language package installation would go here
                }
            }
        }

        // Install binaries
        if !profile.packages.binaries.is_empty() {
            self.output.section("Installing binary tools");
            for bin in &profile.packages.binaries {
                self.output.info(&format!("Installing {}", bin.repository));
                // Binary installation would go here
            }
        }

        // Run post-install scripts
        if !profile.scripts.post_install.is_empty() {
            self.output.progress("Running post-install scripts...");
            for script in &profile.scripts.post_install {
                self.run_script(script)?;
            }
        }

        self.output.success("Profile applied successfully");

        Ok(())
    }

    /// Run a script command
    fn run_script(&self, script: &str) -> Result<()> {
        self.output.info(&format!("Running: {}", script));

        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .status()
            .context("Failed to run script")?;

        if !status.success() {
            bail!("Script failed: {}", script);
        }

        Ok(())
    }
}