use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use crate::ui::output::Output;
use super::Profile;

pub struct ProfileExporter {
    output: Output,
}

impl ProfileExporter {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Export a profile to a file
    pub fn export(&self, profile_name: &str, output_path: &Path, format: ExportFormat) -> Result<()> {
        let profile = Profile::load(profile_name)?;

        self.output.progress(&format!("Exporting profile '{}' to {}", profile_name, output_path.display()));

        match format {
            ExportFormat::Toml => self.export_toml(&profile, output_path)?,
            ExportFormat::Json => self.export_json(&profile, output_path)?,
            ExportFormat::Yaml => self.export_yaml(&profile, output_path)?,
            ExportFormat::Shell => self.export_shell(&profile, output_path)?,
            ExportFormat::Dockerfile => self.export_dockerfile(&profile, output_path)?,
        }

        self.output.success(&format!("Profile exported to {}", output_path.display()));

        Ok(())
    }

    /// Export as TOML
    fn export_toml(&self, profile: &Profile, output_path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(profile)
            .context("Failed to serialize profile to TOML")?;

        fs::write(output_path, content)
            .context("Failed to write TOML file")?;

        Ok(())
    }

    /// Export as JSON
    fn export_json(&self, profile: &Profile, output_path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(profile)
            .context("Failed to serialize profile to JSON")?;

        fs::write(output_path, content)
            .context("Failed to write JSON file")?;

        Ok(())
    }

    /// Export as YAML
    fn export_yaml(&self, profile: &Profile, output_path: &Path) -> Result<()> {
        // Note: Would need serde_yaml dependency
        self.output.warn("YAML export not yet implemented, using JSON format");
        self.export_json(profile, output_path)
    }

    /// Export as shell script
    fn export_shell(&self, profile: &Profile, output_path: &Path) -> Result<()> {
        let mut script = String::new();

        script.push_str("#!/bin/bash\n");
        script.push_str(&format!("# pkmgr profile: {}\n", profile.name));
        script.push_str(&format!("# {}\n", profile.description));
        script.push_str(&format!("# Generated at: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));

        script.push_str("set -e\n\n");

        // Environment variables
        if !profile.environment.is_empty() {
            script.push_str("# Environment variables\n");
            for (key, value) in &profile.environment {
                script.push_str(&format!("export {}=\"{}\"\n", key, value));
            }
            script.push_str("\n");
        }

        // Pre-install scripts
        if !profile.scripts.pre_install.is_empty() {
            script.push_str("# Pre-install scripts\n");
            for cmd in &profile.scripts.pre_install {
                script.push_str(&format!("{}\n", cmd));
            }
            script.push_str("\n");
        }

        // Repositories
        if !profile.repositories.is_empty() {
            script.push_str("# Add repositories\n");
            for repo in &profile.repositories {
                script.push_str(&format!("pkmgr repos add \"{}\"\n", repo.url));
            }
            script.push_str("\n");
        }

        // System packages
        if !profile.packages.system.is_empty() {
            script.push_str("# Install system packages\n");
            script.push_str("pkmgr install");
            for pkg in &profile.packages.system {
                script.push_str(&format!(" \\\n  {}", pkg.name));
                if let Some(ref version) = pkg.version {
                    script.push_str(&format!("@{}", version));
                }
            }
            script.push_str("\n\n");
        }

        // Language packages
        for (lang, packages) in &profile.packages.languages {
            if !packages.is_empty() {
                script.push_str(&format!("# Install {} packages\n", lang));
                for pkg in packages {
                    script.push_str(&format!("pkmgr {} install {}", lang, pkg.name));
                    if let Some(ref version) = pkg.version {
                        script.push_str(&format!("@{}", version));
                    }
                    script.push_str("\n");
                }
                script.push_str("\n");
            }
        }

        // Binary tools
        if !profile.packages.binaries.is_empty() {
            script.push_str("# Install binary tools\n");
            for bin in &profile.packages.binaries {
                script.push_str(&format!("pkmgr binary install {}", bin.repository));
                if let Some(ref version) = bin.version {
                    script.push_str(&format!("@{}", version));
                }
                script.push_str("\n");
            }
            script.push_str("\n");
        }

        // Post-install scripts
        if !profile.scripts.post_install.is_empty() {
            script.push_str("# Post-install scripts\n");
            for cmd in &profile.scripts.post_install {
                script.push_str(&format!("{}\n", cmd));
            }
            script.push_str("\n");
        }

        script.push_str("echo \"Profile installation complete!\"\n");

        fs::write(output_path, script)
            .context("Failed to write shell script")?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(output_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(output_path, perms)?;
        }

        Ok(())
    }

    /// Export as Dockerfile
    fn export_dockerfile(&self, profile: &Profile, output_path: &Path) -> Result<()> {
        let mut dockerfile = String::new();

        dockerfile.push_str(&format!("# pkmgr profile: {}\n", profile.name));
        dockerfile.push_str(&format!("# {}\n\n", profile.description));

        dockerfile.push_str("FROM ubuntu:22.04\n\n");

        // Install pkmgr
        dockerfile.push_str("# Install pkmgr\n");
        dockerfile.push_str("RUN apt-get update && apt-get install -y curl && \\\n");
        dockerfile.push_str("    curl -sSL https://github.com/pkmgr/pkmgr/releases/latest/download/pkmgr-linux-x86_64 -o /usr/local/bin/pkmgr && \\\n");
        dockerfile.push_str("    chmod +x /usr/local/bin/pkmgr\n\n");

        // Environment variables
        if !profile.environment.is_empty() {
            dockerfile.push_str("# Environment variables\n");
            for (key, value) in &profile.environment {
                dockerfile.push_str(&format!("ENV {}=\"{}\"\n", key, value));
            }
            dockerfile.push_str("\n");
        }

        // Add repositories
        if !profile.repositories.is_empty() {
            dockerfile.push_str("# Add repositories\n");
            dockerfile.push_str("RUN");
            for (i, repo) in profile.repositories.iter().enumerate() {
                if i > 0 {
                    dockerfile.push_str(" && \\\n   ");
                } else {
                    dockerfile.push_str(" ");
                }
                dockerfile.push_str(&format!("pkmgr repos add \"{}\"", repo.url));
            }
            dockerfile.push_str("\n\n");
        }

        // Install packages
        if !profile.packages.system.is_empty() {
            dockerfile.push_str("# Install system packages\n");
            dockerfile.push_str("RUN pkmgr install -y");
            for pkg in &profile.packages.system {
                dockerfile.push_str(&format!(" \\\n    {}", pkg.name));
                if let Some(ref version) = pkg.version {
                    dockerfile.push_str(&format!("@{}", version));
                }
            }
            dockerfile.push_str("\n\n");
        }

        // Install language packages
        for (lang, packages) in &profile.packages.languages {
            if !packages.is_empty() {
                dockerfile.push_str(&format!("# Install {} packages\n", lang));
                dockerfile.push_str("RUN");
                for (i, pkg) in packages.iter().enumerate() {
                    if i > 0 {
                        dockerfile.push_str(" && \\\n   ");
                    } else {
                        dockerfile.push_str(" ");
                    }
                    dockerfile.push_str(&format!("pkmgr {} install {}", lang, pkg.name));
                    if let Some(ref version) = pkg.version {
                        dockerfile.push_str(&format!("@{}", version));
                    }
                }
                dockerfile.push_str("\n\n");
            }
        }

        // Install binaries
        if !profile.packages.binaries.is_empty() {
            dockerfile.push_str("# Install binary tools\n");
            dockerfile.push_str("RUN");
            for (i, bin) in profile.packages.binaries.iter().enumerate() {
                if i > 0 {
                    dockerfile.push_str(" && \\\n   ");
                } else {
                    dockerfile.push_str(" ");
                }
                dockerfile.push_str(&format!("pkmgr binary install {}", bin.repository));
                if let Some(ref version) = bin.version {
                    dockerfile.push_str(&format!("@{}", version));
                }
            }
            dockerfile.push_str("\n\n");
        }

        dockerfile.push_str("CMD [\"/bin/bash\"]\n");

        fs::write(output_path, dockerfile)
            .context("Failed to write Dockerfile")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Toml,
    Json,
    Yaml,
    Shell,
    Dockerfile,
}

impl std::str::FromStr for ExportFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "toml" => Ok(ExportFormat::Toml),
            "json" => Ok(ExportFormat::Json),
            "yaml" | "yml" => Ok(ExportFormat::Yaml),
            "shell" | "sh" | "bash" => Ok(ExportFormat::Shell),
            "dockerfile" | "docker" => Ok(ExportFormat::Dockerfile),
            _ => Err(anyhow::anyhow!("Unknown export format: {}", s)),
        }
    }
}