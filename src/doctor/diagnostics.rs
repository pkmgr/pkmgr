use anyhow::{Context, Result};
use std::process::Command;

use crate::doctor::{Finding, HealthReport, Severity};
use crate::ui::output::Output;
use crate::recovery::ErrorFixer;
use crate::core::platform::Platform;

pub struct Diagnostics {
    output: Output,
    auto_fix: bool,
    dry_run: bool,
}

impl Diagnostics {
    pub fn new(output: Output, auto_fix: bool, dry_run: bool) -> Self {
        Self {
            output,
            auto_fix,
            dry_run,
        }
    }

    /// Run diagnostic tests
    pub async fn run_diagnostics(&self, report: &HealthReport) -> Result<()> {
        self.output.section("🔬 Running Diagnostics");

        // Test package manager operations
        self.test_package_manager().await?;

        // Test network operations
        self.test_network_operations().await?;

        // Test file system operations
        self.test_filesystem_operations().await?;

        // Test command execution
        self.test_command_execution().await?;

        Ok(())
    }

    /// Apply automatic fixes for issues
    pub async fn apply_fixes(&self, report: &HealthReport) -> Result<()> {
        let fixable: Vec<_> = report.findings.iter()
            .filter(|f| f.auto_fixable && f.severity >= Severity::Warning)
            .collect();

        if fixable.is_empty() {
            self.output.success("✅ No auto-fixable issues found");
            return Ok(());
        }

        self.output.section(&format!("🔧 Applying {} Automatic Fixes", fixable.len()));

        let platform = Platform::detect()?;
        let fixer = ErrorFixer::new(self.output.clone(), self.dry_run, self.auto_fix);

        for finding in fixable {
            self.output.progress(&format!("Fixing: {}", finding.message));

            match finding.category.as_str() {
                "Storage" => {
                    if finding.name.contains("Disk Space") {
                        self.fix_disk_space().await?;
                    }
                }
                "Cache" => {
                    if finding.name.contains("Cache Usage") {
                        self.fix_cache_usage().await?;
                    } else if finding.name.contains("Expired") {
                        self.fix_expired_cache().await?;
                    }
                }
                "Repository" => {
                    if finding.message.contains("metadata") {
                        self.fix_repository_metadata().await?;
                    }
                }
                "Security" => {
                    if finding.name.contains("GPG Keys") {
                        self.fix_gpg_keys().await?;
                    }
                }
                "Configuration" => {
                    if finding.name.contains("PATH") {
                        self.fix_path_configuration().await?;
                    }
                }
                "Packages" => {
                    if finding.name.contains("Broken") {
                        self.fix_broken_packages().await?;
                    }
                }
                _ => {
                    self.output.info(&format!("⏭️  Skipping: {} (manual fix required)",
                        finding.message));
                }
            }
        }

        self.output.success("✅ Automatic fixes applied");

        Ok(())
    }

    /// Test package manager operations
    async fn test_package_manager(&self) -> Result<()> {
        self.output.progress("Testing package manager...");

        // Try to search for a common package
        let test_packages = vec!["git", "curl", "wget"];

        for package in test_packages {
            if self.test_package_search(package).await? {
                self.output.success(&format!("✓ Package search working (tested with {})", package));
                return Ok(());
            }
        }

        self.output.warn("⚠️  Package search may not be working correctly");
        Ok(())
    }

    /// Test network operations
    async fn test_network_operations(&self) -> Result<()> {
        self.output.progress("Testing network...");

        // Test DNS resolution
        let dns_test = Command::new("nslookup")
            .args(&["github.com"])
            .output();

        if dns_test.is_ok() && dns_test.unwrap().status.success() {
            self.output.success("✓ DNS resolution working");
        } else {
            self.output.warn("⚠️  DNS resolution issues detected");
        }

        // Test HTTPS connectivity
        let https_test = Command::new("curl")
            .args(&["-I", "--max-time", "5", "https://github.com"])
            .output();

        if https_test.is_ok() && https_test.unwrap().status.success() {
            self.output.success("✓ HTTPS connectivity working");
        } else {
            self.output.warn("⚠️  HTTPS connectivity issues detected");
        }

        Ok(())
    }

    /// Test file system operations
    async fn test_filesystem_operations(&self) -> Result<()> {
        self.output.progress("Testing filesystem...");

        // Test temp directory write
        let temp_file = std::env::temp_dir().join("pkmgr_test.txt");

        if std::fs::write(&temp_file, "test").is_ok() {
            let _ = std::fs::remove_file(&temp_file);
            self.output.success("✓ Temp directory writable");
        } else {
            self.output.error("❌ Cannot write to temp directory");
        }

        // Test home directory access
        if let Some(home) = dirs::home_dir() {
            if home.exists() && home.is_dir() {
                self.output.success("✓ Home directory accessible");
            } else {
                self.output.warn("⚠️  Home directory not accessible");
            }
        }

        Ok(())
    }

    /// Test command execution
    async fn test_command_execution(&self) -> Result<()> {
        self.output.progress("Testing command execution...");

        // Test basic commands
        let commands = vec![
            ("echo", vec!["test"]),
            ("true", vec![]),
            ("ls", vec!["-la", "/"]),
        ];

        for (cmd, args) in commands {
            if which::which(cmd).is_ok() {
                let test = Command::new(cmd)
                    .args(&args)
                    .output();

                if test.is_ok() && test.unwrap().status.success() {
                    self.output.success(&format!("✓ Command execution working ({})", cmd));
                    return Ok(());
                }
            }
        }

        self.output.warn("⚠️  Command execution may have issues");
        Ok(())
    }

    // Fix implementations

    async fn fix_disk_space(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would run: pkmgr cache clean");
        } else {
            // Run cache clean command
            use crate::cache::cleaner::CacheCleaner;
            let mut cleaner = CacheCleaner::new(self.output.clone(), false)?;
            cleaner.clean_expired().await?;
        }
        Ok(())
    }

    async fn fix_cache_usage(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would run: pkmgr cache clean");
        } else {
            use crate::cache::cleaner::CacheCleaner;
            let mut cleaner = CacheCleaner::new(self.output.clone(), false)?;
            cleaner.clean_all(true).await?;
        }
        Ok(())
    }

    async fn fix_expired_cache(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would run: pkmgr cache clean --expired");
        } else {
            use crate::cache::cleaner::CacheCleaner;
            let mut cleaner = CacheCleaner::new(self.output.clone(), false)?;
            cleaner.clean_expired().await?;
        }
        Ok(())
    }

    async fn fix_repository_metadata(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would run: pkmgr repos update");
        } else {
            // Update repository metadata
            use crate::repos::manager::RepositoryManager;
            use crate::core::platform::Platform;

            let platform = Platform::detect()?;
            let mut manager = RepositoryManager::new(self.output.clone(), platform);
            manager.update_cache().await?;
        }
        Ok(())
    }

    async fn fix_gpg_keys(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would refresh GPG keys");
        } else {
            // Refresh expired GPG keys
            Command::new("gpg")
                .args(&["--refresh-keys"])
                .status()?;
        }
        Ok(())
    }

    async fn fix_path_configuration(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would add ~/.local/bin to PATH");
        } else {
            self.output.info("Run: eval $(pkmgr shell add)");
        }
        Ok(())
    }

    async fn fix_broken_packages(&self) -> Result<()> {
        if self.dry_run {
            self.output.info("Would run: pkmgr fix");
        } else {
            // Run package fix commands based on platform
            use crate::core::platform::Platform;
            let platform = Platform::detect()?;

            match platform.platform {
                Platform::Linux => {
                    if platform.distribution.as_ref().map_or(false, |d| d.contains("ubuntu") || d.contains("debian")) {
                        Command::new("dpkg")
                            .args(&["--configure", "-a"])
                            .status()?;

                        Command::new("apt-get")
                            .args(&["--fix-broken", "install", "-y"])
                            .status()?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn test_package_search(&self, package: &str) -> Result<bool> {
        // Try to search for package using native package manager
        use crate::core::platform::Platform;
        let platform = Platform::detect()?;

        let result = match platform.platform {
            Platform::Linux => {
                if which::which("apt-cache").is_ok() {
                    Command::new("apt-cache")
                        .args(&["search", package])
                        .output()
                } else if which::which("dnf").is_ok() {
                    Command::new("dnf")
                        .args(&["search", package])
                        .output()
                } else if which::which("pacman").is_ok() {
                    Command::new("pacman")
                        .args(&["-Ss", package])
                        .output()
                } else {
                    return Ok(false);
                }
            }
            Platform::MacOs => {
                if which::which("brew").is_ok() {
                    Command::new("brew")
                        .args(&["search", package])
                        .output()
                } else {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        };

        Ok(result.is_ok() && result.unwrap().status.success())
    }
}