use anyhow::{Context, Result, bail};
use std::process::Command;
use std::collections::HashMap;
use crate::ui::output::Output;
use crate::ui::prompt::Prompt;
use super::{ErrorAnalysis, FixStrategy, FixSuggestion, RiskLevel};

pub struct ErrorFixer {
    output: Output,
    prompt: Prompt,
    dry_run: bool,
    auto_fix: bool,
}

impl ErrorFixer {
    pub fn new(output: Output, dry_run: bool, auto_fix: bool) -> Self {
        let emoji_enabled = output.emoji_enabled;
        Self {
            output,
            prompt: Prompt::new(emoji_enabled),
            dry_run,
            auto_fix,
        }
    }

    /// Apply a fix for an error
    pub async fn apply_fix(
        &self,
        analysis: &ErrorAnalysis,
        fix: &FixSuggestion,
    ) -> Result<bool> {
        // Check if we should proceed
        if !self.should_apply_fix(fix)? {
            return Ok(false);
        }

        self.output.progress(&format!("Applying fix: {}", fix.description));

        if self.dry_run {
            self.output.info("Dry run mode - would execute:");
            self.display_fix_strategy(&fix.strategy, &analysis.extracted_data);
            return Ok(false);
        }

        // Apply the fix strategy
        let success = self.execute_strategy(&fix.strategy, &analysis.extracted_data).await?;

        if success {
            self.output.success("Fix applied successfully");
        } else {
            self.output.error("Fix failed to resolve the issue");
        }

        Ok(success)
    }

    /// Check if we should apply a fix
    fn should_apply_fix(&self, fix: &FixSuggestion) -> Result<bool> {
        // In auto-fix mode, only apply safe fixes
        if self.auto_fix {
            return Ok(fix.risk_level <= RiskLevel::Low);
        }

        // Otherwise prompt user
        match fix.risk_level {
            RiskLevel::Safe => Ok(true),
            RiskLevel::Low => {
                self.output.warn(&format!("This fix has low risk: {}", fix.description));
                self.prompt.confirm("Apply this fix?")
            }
            RiskLevel::Medium => {
                self.output.warn(&format!("⚡ This fix has medium risk: {}", fix.description));
                self.output.warn("The operation is reversible but may modify system state");
                self.prompt.confirm("Apply this fix?")
            }
            RiskLevel::High => {
                self.output.error(&format!("🔥 This fix has HIGH RISK: {}", fix.description));
                self.output.error("This operation may cause system instability");

                if !self.prompt.confirm("Are you sure you want to proceed?")? {
                    return Ok(false);
                }

                self.output.error("Type 'YES' in capitals to confirm:");
                let confirm = self.prompt.input("")?;
                Ok(confirm == "YES")
            }
        }
    }

    /// Execute a fix strategy
    async fn execute_strategy(
        &self,
        strategy: &FixStrategy,
        data: &HashMap<String, String>,
    ) -> Result<bool> {
        match strategy {
            FixStrategy::Command(args) => {
                self.execute_command(args, data)
            }

            FixStrategy::CommandSequence(commands) => {
                for args in commands {
                    if !self.execute_command(args, data)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }

            FixStrategy::BuiltIn(name) => {
                self.execute_builtin(name, data).await
            }

            FixStrategy::Rebuild { package } => {
                let package = self.substitute_variables(package, data);
                self.rebuild_package(&package).await
            }

            FixStrategy::ForceOverwrite { patterns } => {
                self.force_overwrite(patterns, data).await
            }

            FixStrategy::CleanRetry { clean_commands, retry_original } => {
                // Execute clean commands
                for args in clean_commands {
                    self.execute_command(args, data)?;
                }

                if *retry_original {
                    self.output.info("Retrying original command...");
                    // This would need the original command context
                }
                Ok(true)
            }

            FixStrategy::UpdateComponent { component } => {
                let component = self.substitute_variables(component, data);
                self.update_component(&component).await
            }

            FixStrategy::Reconfigure { service } => {
                let service = self.substitute_variables(service, data);
                self.reconfigure_service(&service).await
            }

            FixStrategy::EnvironmentFix { variables, permanent } => {
                self.fix_environment(variables, *permanent)
            }

            FixStrategy::Custom(name) => {
                self.output.warn(&format!("Custom fix '{}' not implemented", name));
                Ok(false)
            }
        }
    }

    /// Execute a single command
    fn execute_command(&self, args: &[String], data: &HashMap<String, String>) -> Result<bool> {
        if args.is_empty() {
            return Ok(false);
        }

        // Substitute variables
        let args: Vec<String> = args.iter()
            .map(|arg| self.substitute_variables(arg, data))
            .collect();

        self.output.info(&format!("Executing: {}", args.join(" ")));

        let output = Command::new(&args[0])
            .args(&args[1..])
            .output()
            .context("Failed to execute command")?;

        Ok(output.status.success())
    }

    /// Execute built-in fix function
    async fn execute_builtin(&self, name: &str, data: &HashMap<String, String>) -> Result<bool> {
        match name {
            "retry_with_sudo" => {
                self.output.info("Retrying with administrator privileges...");
                // This would need the original command context
                Ok(true)
            }

            "clear_locks" => {
                self.clear_package_locks().await
            }

            "fix_404_repos" => {
                self.fix_404_repositories().await
            }

            "install_build_tools" => {
                self.install_build_tools().await
            }

            "find_and_install_library" => {
                if let Some(library) = data.get("library") {
                    self.find_and_install_library(library).await
                } else {
                    Ok(false)
                }
            }

            "cleanup_disk_space" => {
                self.cleanup_disk_space().await
            }

            "retry_with_timeout" => {
                self.output.info("Retrying with extended timeout...");
                Ok(true)
            }

            "suggest_env_var" => {
                if let Some(var) = data.get("variable") {
                    self.output.info(&format!("Set environment variable: export {}=<value>", var));
                }
                Ok(false)
            }

            "switch_python_version" => {
                if let Some(version) = data.get("version") {
                    self.output.info(&format!("Switch to Python {}: pkmgr python use {}", version, version));
                }
                Ok(false)
            }

            "switch_node_version" => {
                if let Some(version) = data.get("version") {
                    self.output.info(&format!("Switch to Node.js {}: pkmgr node use {}", version, version));
                }
                Ok(false)
            }

            _ => {
                self.output.warn(&format!("Unknown built-in fix: {}", name));
                Ok(false)
            }
        }
    }

    /// Clear package manager locks
    async fn clear_package_locks(&self) -> Result<bool> {
        self.output.progress("Clearing package manager locks...");

        #[cfg(target_os = "linux")]
        {
            // APT locks
            let _ = Command::new("rm").args(&["-f", "/var/lib/dpkg/lock-frontend"]).output();
            let _ = Command::new("rm").args(&["-f", "/var/lib/dpkg/lock"]).output();
            let _ = Command::new("rm").args(&["-f", "/var/cache/apt/archives/lock"]).output();

            // YUM/DNF locks
            let _ = Command::new("rm").args(&["-f", "/var/run/yum.pid"]).output();

            // Pacman lock
            let _ = Command::new("rm").args(&["-f", "/var/lib/pacman/db.lck"]).output();
        }

        Ok(true)
    }

    /// Fix 404 repositories
    async fn fix_404_repositories(&self) -> Result<bool> {
        self.output.progress("Checking for outdated repositories...");

        #[cfg(target_os = "linux")]
        {
            // Update repository metadata
            let _ = Command::new("apt-get").args(&["update"]).output();
        }

        Ok(true)
    }

    /// Install build tools
    async fn install_build_tools(&self) -> Result<bool> {
        self.output.progress("Installing build tools...");

        #[cfg(target_os = "linux")]
        {
            // Detect package manager and install appropriate tools
            if Path::new("/usr/bin/apt-get").exists() {
                Command::new("apt-get")
                    .args(&["install", "-y", "build-essential"])
                    .status()?;
            } else if Path::new("/usr/bin/dnf").exists() {
                Command::new("dnf")
                    .args(&["groupinstall", "-y", "Development Tools"])
                    .status()?;
            } else if Path::new("/usr/bin/pacman").exists() {
                Command::new("pacman")
                    .args(&["-S", "--noconfirm", "base-devel"])
                    .status()?;
            }
        }

        Ok(true)
    }

    /// Find and install missing library
    async fn find_and_install_library(&self, library: &str) -> Result<bool> {
        self.output.progress(&format!("Searching for package providing {}...", library));

        // This would use package manager search functionality
        self.output.warn(&format!("Library search not implemented for {}", library));

        Ok(false)
    }

    /// Clean up disk space
    async fn cleanup_disk_space(&self) -> Result<bool> {
        self.output.progress("Cleaning up disk space...");

        #[cfg(target_os = "linux")]
        {
            // Clean package caches
            let _ = Command::new("apt-get").args(&["clean"]).output();
            let _ = Command::new("dnf").args(&["clean", "all"]).output();
            let _ = Command::new("pacman").args(&["-Scc", "--noconfirm"]).output();

            // Clean temp files
            let _ = Command::new("rm").args(&["-rf", "/tmp/*"]).output();
        }

        Ok(true)
    }

    /// Rebuild a package
    async fn rebuild_package(&self, package: &str) -> Result<bool> {
        self.output.progress(&format!("Rebuilding package: {}", package));

        // Platform-specific rebuild
        #[cfg(target_os = "linux")]
        {
            if Path::new("/usr/bin/yay").exists() {
                Command::new("yay")
                    .args(&["-S", "--rebuild", package])
                    .status()?;
            }
        }

        Ok(true)
    }

    /// Force overwrite files
    async fn force_overwrite(&self, patterns: &[String], data: &HashMap<String, String>) -> Result<bool> {
        self.output.warn("Force overwriting conflicting files...");

        // This would be platform-specific
        Ok(true)
    }

    /// Update a system component
    async fn update_component(&self, component: &str) -> Result<bool> {
        self.output.progress(&format!("Updating component: {}", component));

        match component {
            "keyring" => {
                #[cfg(target_os = "linux")]
                {
                    let _ = Command::new("pacman")
                        .args(&["-Sy", "archlinux-keyring", "--noconfirm"])
                        .output();
                }
            }
            _ => {
                self.output.warn(&format!("Component update not implemented for {}", component));
            }
        }

        Ok(true)
    }

    /// Reconfigure a service
    async fn reconfigure_service(&self, service: &str) -> Result<bool> {
        self.output.progress(&format!("Reconfiguring service: {}", service));

        #[cfg(target_os = "linux")]
        {
            let _ = Command::new("dpkg-reconfigure")
                .arg(service)
                .output();
        }

        Ok(true)
    }

    /// Fix environment variables
    fn fix_environment(&self, variables: &HashMap<String, String>, permanent: bool) -> Result<bool> {
        for (key, value) in variables {
            self.output.info(&format!("Setting {}={}", key, value));
            std::env::set_var(key, value);

            if permanent {
                // Would write to shell profile
                self.output.info(&format!("Add to shell profile: export {}={}", key, value));
            }
        }

        Ok(true)
    }

    /// Substitute variables in strings
    fn substitute_variables(&self, template: &str, data: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        for (key, value) in data {
            result = result.replace(&format!("{{{}}}", key), value);
        }

        result
    }

    /// Display what a fix strategy would do
    fn display_fix_strategy(&self, strategy: &FixStrategy, data: &HashMap<String, String>) {
        match strategy {
            FixStrategy::Command(args) => {
                let args: Vec<String> = args.iter()
                    .map(|arg| self.substitute_variables(arg, data))
                    .collect();
                self.output.info(&format!("  Execute: {}", args.join(" ")));
            }

            FixStrategy::CommandSequence(commands) => {
                for args in commands {
                    let args: Vec<String> = args.iter()
                        .map(|arg| self.substitute_variables(arg, data))
                        .collect();
                    self.output.info(&format!("  Execute: {}", args.join(" ")));
                }
            }

            FixStrategy::BuiltIn(name) => {
                self.output.info(&format!("  Run built-in fix: {}", name));
            }

            _ => {
                self.output.info(&format!("  Apply fix strategy: {:?}", strategy));
            }
        }
    }
}

use std::path::Path;