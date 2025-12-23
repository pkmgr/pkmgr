use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;
use std::fs;

use crate::doctor::{CheckCategory, Finding, HealthReport, Severity, SystemInfo};
use crate::core::platform::{Platform, PlatformInfo, Architecture};
use crate::ui::output::Output;
use crate::cache::manager::CacheManager;
use crate::repos::manager::RepositoryManager;

pub struct HealthChecker {
    platform: PlatformInfo,
    output: Output,
    auto_fix: bool,
}

impl HealthChecker {
    pub fn new(output: Output, auto_fix: bool) -> Result<Self> {
        let platform = Platform::detect()?;
        Ok(Self {
            platform,
            output,
            auto_fix,
        })
    }

    /// Run all health checks
    pub async fn check_all(&self) -> Result<HealthReport> {
        self.output.section("🏥 Running System Health Checks");

        let system_info = SystemInfo::gather()?;
        let mut report = HealthReport::new(system_info);

        // Run checks in order of importance
        self.check_system(&mut report).await?;
        self.check_packages(&mut report).await?;
        self.check_storage(&mut report).await?;
        self.check_network(&mut report).await?;
        self.check_security(&mut report).await?;
        self.check_repositories(&mut report).await?;
        self.check_languages(&mut report).await?;
        self.check_cache(&mut report).await?;
        self.check_configuration(&mut report).await?;
        self.check_shell(&mut report).await?;

        // Generate recommendations
        report.generate_recommendations();

        Ok(report)
    }

    /// Run package-specific health checks
    pub async fn check_packages_only(&self) -> Result<HealthReport> {
        let system_info = SystemInfo::gather()?;
        let mut report = HealthReport::new(system_info);

        self.check_packages(&mut report).await?;
        self.check_repositories(&mut report).await?;

        report.generate_recommendations();
        Ok(report)
    }

    /// Run USB-specific health checks
    pub async fn check_usb_only(&self) -> Result<HealthReport> {
        let system_info = SystemInfo::gather()?;
        let mut report = HealthReport::new(system_info);

        self.check_usb_devices(&mut report).await?;

        report.generate_recommendations();
        Ok(report)
    }

    /// Run security-specific health checks
    pub async fn check_security_only(&self) -> Result<HealthReport> {
        let system_info = SystemInfo::gather()?;
        let mut report = HealthReport::new(system_info);

        self.check_security(&mut report).await?;

        report.generate_recommendations();
        Ok(report)
    }

    /// Check system basics
    async fn check_system(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking system...");

        // Check OS support
        let supported_os = matches!(self.platform.platform, Platform::Linux | Platform::MacOs | Platform::Windows);
        if supported_os {
            report.add_finding(Finding::new(
                "System",
                "OS Support",
                Severity::Ok,
                format!("Operating system {:?} is supported", self.platform.platform),
            ));
        } else {
            report.add_finding(Finding::new(
                "System",
                "OS Support",
                Severity::Warning,
                format!("Operating system {:?} has limited support", self.platform.platform),
            ).with_details("Some features may not work correctly"));
        }

        // Check architecture
        let supported_arch = matches!(self.platform.architecture, Architecture::X86_64 | Architecture::Aarch64);
        if supported_arch {
            report.add_finding(Finding::new(
                "System",
                "Architecture",
                Severity::Ok,
                format!("Architecture {:?} is supported", self.platform.architecture),
            ));
        } else {
            report.add_finding(Finding::new(
                "System",
                "Architecture",
                Severity::Warning,
                format!("Architecture {:?} has limited support", self.platform.architecture),
            ));
        }

        // Check uptime (warn if just rebooted)
        if report.system_info.uptime.contains("minutes") && !report.system_info.uptime.contains("hours") {
            report.add_finding(Finding::new(
                "System",
                "Recent Reboot",
                Severity::Info,
                "System was recently rebooted",
            ).with_details(&format!("Uptime: {}", report.system_info.uptime)));
        }

        // Check CPU
        if report.system_info.cpu_count < 2 {
            report.add_finding(Finding::new(
                "System",
                "CPU Cores",
                Severity::Warning,
                "System has limited CPU cores",
            ).with_details("Performance may be impacted for parallel operations"));
        } else {
            report.add_finding(Finding::new(
                "System",
                "CPU Cores",
                Severity::Ok,
                format!("{} CPU cores available", report.system_info.cpu_count),
            ));
        }

        // Check memory
        let memory_gb = report.system_info.memory_total / (1024 * 1024 * 1024);
        let memory_available_gb = report.system_info.memory_available / (1024 * 1024 * 1024);

        if memory_available_gb < 1 {
            report.add_finding(Finding::new(
                "System",
                "Memory",
                Severity::Warning,
                "Low available memory",
            ).with_details(&format!("{} GB available of {} GB total", memory_available_gb, memory_gb))
            .with_fix("Close unnecessary applications to free memory", false));
        } else {
            report.add_finding(Finding::new(
                "System",
                "Memory",
                Severity::Ok,
                format!("{} GB memory available", memory_available_gb),
            ));
        }

        Ok(())
    }

    /// Check package managers
    async fn check_packages(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking package management...");

        // Check for package manager
        let pm_check = match self.platform.platform {
            Platform::Linux => self.check_linux_package_manager(),
            Platform::MacOs => self.check_macos_package_manager(),
            Platform::Windows => self.check_windows_package_manager(),
            _ => Ok(None),
        };

        if let Ok(Some(pm_name)) = pm_check {
            report.add_finding(Finding::new(
                "Packages",
                "Package Manager",
                Severity::Ok,
                format!("{} is available", pm_name),
            ));

            // Check for updates
            self.check_package_updates(report, &pm_name).await?;

            // Check for broken packages
            self.check_broken_packages(report, &pm_name).await?;

            // Check for held packages
            self.check_held_packages(report, &pm_name).await?;

        } else {
            report.add_finding(Finding::new(
                "Packages",
                "Package Manager",
                Severity::Critical,
                "No package manager detected",
            ).with_fix("Install a supported package manager", false));
        }

        Ok(())
    }

    /// Check storage
    async fn check_storage(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking storage...");

        // Check disk space
        let disk_available_gb = report.system_info.disk_available / (1024 * 1024 * 1024);
        let disk_total_gb = report.system_info.disk_total / (1024 * 1024 * 1024);
        let disk_usage_percent = if disk_total_gb > 0 {
            ((disk_total_gb - disk_available_gb) as f32 / disk_total_gb as f32 * 100.0) as u8
        } else {
            0
        };

        let severity = if disk_available_gb < 1 {
            Severity::Critical
        } else if disk_available_gb < 5 {
            Severity::Error
        } else if disk_available_gb < 10 {
            Severity::Warning
        } else {
            Severity::Ok
        };

        let message = if severity == Severity::Ok {
            format!("{} GB disk space available", disk_available_gb)
        } else {
            format!("Low disk space: {} GB available ({}% used)", disk_available_gb, disk_usage_percent)
        };

        let mut finding = Finding::new("Storage", "Disk Space", severity.clone(), message);

        if severity != Severity::Ok {
            finding = finding
                .with_details(&format!("{} GB of {} GB total", disk_available_gb, disk_total_gb))
                .with_fix("Run 'pkmgr cache clean' to free space", true);
        }

        report.add_finding(finding);

        // Check temp directory
        self.check_temp_directory(report).await?;

        Ok(())
    }

    /// Check network connectivity
    async fn check_network(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking network...");

        // Check DNS resolution
        let dns_check = Command::new("nslookup")
            .args(&["github.com"])
            .output();

        if dns_check.is_ok() && dns_check.unwrap().status.success() {
            report.add_finding(Finding::new(
                "Network",
                "DNS Resolution",
                Severity::Ok,
                "DNS resolution working",
            ));
        } else {
            report.add_finding(Finding::new(
                "Network",
                "DNS Resolution",
                Severity::Error,
                "DNS resolution failing",
            ).with_fix("Check network settings and DNS configuration", false));
        }

        // Check internet connectivity
        let ping_check = Command::new("ping")
            .args(&["-c", "1", "-W", "2", "8.8.8.8"])
            .output();

        if ping_check.is_ok() && ping_check.unwrap().status.success() {
            report.add_finding(Finding::new(
                "Network",
                "Internet Connectivity",
                Severity::Ok,
                "Internet connection available",
            ));
        } else {
            report.add_finding(Finding::new(
                "Network",
                "Internet Connectivity",
                Severity::Warning,
                "Internet connection issues detected",
            ).with_details("Package downloads may fail"));
        }

        // Check proxy settings
        if std::env::var("HTTP_PROXY").is_ok() || std::env::var("HTTPS_PROXY").is_ok() {
            report.add_finding(Finding::new(
                "Network",
                "Proxy Configuration",
                Severity::Info,
                "Proxy settings detected",
            ).with_details("Using configured proxy for network requests"));
        }

        Ok(())
    }

    /// Check security settings
    async fn check_security(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking security...");

        // Check GPG
        let gpg_check = Command::new("gpg")
            .arg("--version")
            .output();

        if gpg_check.is_ok() && gpg_check.unwrap().status.success() {
            report.add_finding(Finding::new(
                "Security",
                "GPG",
                Severity::Ok,
                "GPG is installed",
            ));

            // Check for expired keys
            self.check_gpg_keys(report).await?;
        } else {
            report.add_finding(Finding::new(
                "Security",
                "GPG",
                Severity::Warning,
                "GPG not installed",
            ).with_details("Package signature verification will be limited")
            .with_fix("Install GPG for package verification", false));
        }

        // Check sudo/admin access
        self.check_privileges(report).await?;

        // Check for SSL certificates
        self.check_ssl_certs(report).await?;

        Ok(())
    }

    /// Check repositories
    async fn check_repositories(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking repositories...");

        let repo_manager = RepositoryManager::new(self.output.clone(), self.platform.clone());
        let repos = repo_manager.list()?;

        if repos.is_empty() {
            report.add_finding(Finding::new(
                "Repository",
                "Repository List",
                Severity::Warning,
                "No repositories configured",
            ).with_details("Limited packages available"));
        } else {
            report.add_finding(Finding::new(
                "Repository",
                "Repository Count",
                Severity::Ok,
                format!("{} repositories configured", repos.len()),
            ));

            // Check for outdated repos
            for repo in &repos {
                if repo.enabled && repo.metadata.last_updated.is_some() {
                    let last_update = repo.metadata.last_updated.unwrap();
                    let now = chrono::Utc::now();
                    let age = now.signed_duration_since(last_update);

                    if age.num_days() > 7 {
                        report.add_finding(Finding::new(
                            "Repository",
                            &format!("Repository: {}", repo.name),
                            Severity::Info,
                            format!("Repository metadata is {} days old", age.num_days()),
                        ).with_fix("Run 'pkmgr repos update' to refresh", true));
                    }
                }
            }
        }

        Ok(())
    }

    /// Check language versions
    async fn check_languages(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking language versions...");

        let languages = vec![
            ("python", "python3", "--version"),
            ("node", "node", "--version"),
            ("ruby", "ruby", "--version"),
            ("go", "go", "version"),
            ("rust", "rustc", "--version"),
            ("java", "java", "-version"),
            ("php", "php", "--version"),
            ("dotnet", "dotnet", "--version"),
        ];

        for (name, cmd, arg) in languages {
            let check = Command::new(cmd)
                .arg(arg)
                .output();

            if check.is_ok() && check.unwrap().status.success() {
                report.add_finding(Finding::new(
                    "Languages",
                    &format!("{} Version", name),
                    Severity::Ok,
                    format!("{} is installed", name),
                ));
            }
        }

        Ok(())
    }

    /// Check cache
    async fn check_cache(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking cache...");

        let cache_manager = CacheManager::new(self.output.clone())?;
        let stats = cache_manager.get_stats()?;

        // Check cache size
        let cache_size_gb = stats.total_size as f64 / (1024.0 * 1024.0 * 1024.0);
        let severity = if stats.cache_usage_percent > 90.0 {
            Severity::Warning
        } else {
            Severity::Ok
        };

        report.add_finding(Finding::new(
            "Cache",
            "Cache Usage",
            severity,
            format!("Cache using {:.1} GB ({:.0}% of limit)", cache_size_gb, stats.cache_usage_percent),
        ).with_fix("Run 'pkmgr cache clean' to free space", true));

        // Check expired entries
        if stats.expired_entries > 0 {
            report.add_finding(Finding::new(
                "Cache",
                "Expired Entries",
                Severity::Info,
                format!("{} expired cache entries", stats.expired_entries),
            ).with_fix("Run 'pkmgr cache clean --expired'", true));
        }

        Ok(())
    }

    /// Check configuration
    async fn check_configuration(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking configuration...");

        // Check config file
        let config_path = dirs::config_dir()
            .map(|d| d.join("pkmgr").join("config.toml"))
            .unwrap_or_default();

        if config_path.exists() {
            report.add_finding(Finding::new(
                "Configuration",
                "Config File",
                Severity::Ok,
                "Configuration file exists",
            ));
        } else {
            report.add_finding(Finding::new(
                "Configuration",
                "Config File",
                Severity::Info,
                "Using default configuration",
            ).with_details("Create config with 'pkmgr config set'"));
        }

        // Check ~/.local/bin in PATH
        let path = std::env::var("PATH").unwrap_or_default();
        let local_bin = dirs::home_dir()
            .map(|h| h.join(".local").join("bin"))
            .unwrap_or_default();

        if path.contains(local_bin.to_str().unwrap_or("")) {
            report.add_finding(Finding::new(
                "Configuration",
                "PATH Configuration",
                Severity::Ok,
                "~/.local/bin is in PATH",
            ));
        } else {
            report.add_finding(Finding::new(
                "Configuration",
                "PATH Configuration",
                Severity::Warning,
                "~/.local/bin not in PATH",
            ).with_fix("Run 'eval $(pkmgr shell add)'", true));
        }

        Ok(())
    }

    /// Check shell integration
    async fn check_shell(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking shell integration...");

        use crate::shell::detector::ShellDetector;
        use crate::shell::ShellType;

        let shell = ShellDetector::detect_default_shell()
            .unwrap_or(ShellType::Unknown);

        if shell != ShellType::Unknown {
            report.add_finding(Finding::new(
                "Shell",
                "Shell Detection",
                Severity::Ok,
                format!("{} shell detected", shell.display_name()),
            ));

            // Check if integration installed
            if ShellDetector::is_integration_installed(&shell) {
                report.add_finding(Finding::new(
                    "Shell",
                    "Shell Integration",
                    Severity::Ok,
                    "Shell integration is installed",
                ));
            } else {
                report.add_finding(Finding::new(
                    "Shell",
                    "Shell Integration",
                    Severity::Info,
                    "Shell integration not installed",
                ).with_fix(&format!("Run 'eval $(pkmgr shell load)'"), false));
            }

            // Check if completions installed
            if ShellDetector::are_completions_installed(&shell) {
                report.add_finding(Finding::new(
                    "Shell",
                    "Shell Completions",
                    Severity::Ok,
                    "Shell completions are installed",
                ));
            } else {
                report.add_finding(Finding::new(
                    "Shell",
                    "Shell Completions",
                    Severity::Info,
                    "Shell completions not installed",
                ).with_fix(&format!("Run 'pkmgr shell completions {}'",
                    shell.display_name().to_lowercase()), false));
            }
        } else {
            report.add_finding(Finding::new(
                "Shell",
                "Shell Detection",
                Severity::Info,
                "Could not detect shell type",
            ));
        }

        Ok(())
    }

    /// Check USB devices
    async fn check_usb_devices(&self, report: &mut HealthReport) -> Result<()> {
        self.output.progress("Checking USB devices...");

        use crate::usb::device::DeviceDetector;

        let detector = DeviceDetector;
        let devices = detector.list_usb_devices()?;

        if devices.is_empty() {
            report.add_finding(Finding::new(
                "USB",
                "USB Devices",
                Severity::Info,
                "No USB storage devices detected",
            ));
        } else {
            report.add_finding(Finding::new(
                "USB",
                "USB Devices",
                Severity::Ok,
                format!("{} USB storage devices detected", devices.len()),
            ));

            for device in &devices {
                let size_gb = device.size_bytes / (1024 * 1024 * 1024);
                report.add_finding(Finding::new(
                    "USB",
                    &format!("Device: {}", device.name),
                    Severity::Info,
                    format!("{} - {} GB", device.model.as_ref().unwrap_or(&"Unknown".to_string()), size_gb),
                ).with_details(&format!("Path: {}, Filesystem: {:?}",
                    device.path.display(),
                    device.filesystem
                )));
            }
        }

        Ok(())
    }

    // Helper methods

    fn check_linux_package_manager(&self) -> Result<Option<String>> {
        let managers = vec![
            ("apt-get", "APT"),
            ("dnf", "DNF"),
            ("yum", "YUM"),
            ("pacman", "Pacman"),
            ("zypper", "Zypper"),
            ("apk", "APK"),
        ];

        for (cmd, name) in managers {
            if which::which(cmd).is_ok() {
                return Ok(Some(name.to_string()));
            }
        }

        Ok(None)
    }

    fn check_macos_package_manager(&self) -> Result<Option<String>> {
        if which::which("brew").is_ok() {
            Ok(Some("Homebrew".to_string()))
        } else if which::which("port").is_ok() {
            Ok(Some("MacPorts".to_string()))
        } else {
            Ok(None)
        }
    }

    fn check_windows_package_manager(&self) -> Result<Option<String>> {
        if which::which("winget").is_ok() {
            Ok(Some("WinGet".to_string()))
        } else if which::which("choco").is_ok() {
            Ok(Some("Chocolatey".to_string()))
        } else if which::which("scoop").is_ok() {
            Ok(Some("Scoop".to_string()))
        } else {
            Ok(None)
        }
    }

    async fn check_package_updates(&self, report: &mut HealthReport, pm: &str) -> Result<()> {
        // This would check for available updates
        // For now, just add an info finding
        report.add_finding(Finding::new(
            "Packages",
            "Package Updates",
            Severity::Info,
            "Check for updates with 'pkmgr check'",
        ));
        Ok(())
    }

    async fn check_broken_packages(&self, report: &mut HealthReport, _pm: &str) -> Result<()> {
        // Platform-specific check for broken packages
        if self.platform.platform == Platform::Linux {
            if self.platform.distribution.as_ref().map_or(false, |d| d.contains("ubuntu") || d.contains("debian")) {
                let check = Command::new("dpkg")
                    .args(&["--audit"])
                    .output();

                if let Ok(output) = check {
                    if output.stdout.is_empty() && output.status.success() {
                        report.add_finding(Finding::new(
                            "Packages",
                            "Package Integrity",
                            Severity::Ok,
                            "No broken packages detected",
                        ));
                    } else {
                        report.add_finding(Finding::new(
                            "Packages",
                            "Package Integrity",
                            Severity::Error,
                            "Broken packages detected",
                        ).with_fix("Run 'pkmgr fix'", true));
                    }
                }
            }
        }
        Ok(())
    }

    async fn check_held_packages(&self, report: &mut HealthReport, _pm: &str) -> Result<()> {
        // Check for held/pinned packages
        if self.platform.platform == Platform::Linux {
            if self.platform.distribution.as_ref().map_or(false, |d| d.contains("ubuntu") || d.contains("debian")) {
                let check = Command::new("apt-mark")
                    .arg("showhold")
                    .output();

                if let Ok(output) = check {
                    if !output.stdout.is_empty() {
                        let held_count = String::from_utf8_lossy(&output.stdout).lines().count();
                        report.add_finding(Finding::new(
                            "Packages",
                            "Held Packages",
                            Severity::Info,
                            format!("{} packages are held", held_count),
                        ).with_details("Held packages will not be updated"));
                    }
                }
            }
        }
        Ok(())
    }

    async fn check_temp_directory(&self, report: &mut HealthReport) -> Result<()> {
        let temp_dir = std::env::temp_dir();

        if let Ok(entries) = fs::read_dir(&temp_dir) {
            let file_count = entries.count();

            if file_count > 10000 {
                report.add_finding(Finding::new(
                    "Storage",
                    "Temp Directory",
                    Severity::Warning,
                    format!("Temp directory has {} files", file_count),
                ).with_fix("Clean temp directory", false));
            }
        }

        Ok(())
    }

    async fn check_gpg_keys(&self, report: &mut HealthReport) -> Result<()> {
        let check = Command::new("gpg")
            .args(&["--list-keys", "--with-colons"])
            .output();

        if let Ok(output) = check {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let expired_count = output_str.lines()
                .filter(|line| line.contains(":e:"))
                .count();

            if expired_count > 0 {
                report.add_finding(Finding::new(
                    "Security",
                    "GPG Keys",
                    Severity::Warning,
                    format!("{} GPG keys are expired", expired_count),
                ).with_fix("Run 'pkmgr repos update' to refresh keys", true));
            } else {
                report.add_finding(Finding::new(
                    "Security",
                    "GPG Keys",
                    Severity::Ok,
                    "All GPG keys are valid",
                ));
            }
        }

        Ok(())
    }

    async fn check_privileges(&self, report: &mut HealthReport) -> Result<()> {
        // Check sudo access
        let sudo_check = Command::new("sudo")
            .args(&["-n", "true"])
            .output();

        if sudo_check.is_ok() && sudo_check.unwrap().status.success() {
            report.add_finding(Finding::new(
                "Security",
                "Admin Access",
                Severity::Ok,
                "Passwordless sudo available",
            ));
        } else if which::which("sudo").is_ok() {
            report.add_finding(Finding::new(
                "Security",
                "Admin Access",
                Severity::Info,
                "Sudo available (password required)",
            ));
        } else {
            report.add_finding(Finding::new(
                "Security",
                "Admin Access",
                Severity::Warning,
                "No sudo access detected",
            ).with_details("System-wide installations will fail"));
        }

        Ok(())
    }

    async fn check_ssl_certs(&self, report: &mut HealthReport) -> Result<()> {
        // Check for ca-certificates
        let cert_paths = vec![
            "/etc/ssl/certs",
            "/etc/pki/tls/certs",
            "/usr/share/ca-certificates",
        ];

        for path in cert_paths {
            if Path::new(path).exists() {
                report.add_finding(Finding::new(
                    "Security",
                    "SSL Certificates",
                    Severity::Ok,
                    "SSL certificate store found",
                ));
                return Ok(());
            }
        }

        report.add_finding(Finding::new(
            "Security",
            "SSL Certificates",
            Severity::Warning,
            "SSL certificate store not found",
        ).with_details("HTTPS connections may fail"));

        Ok(())
    }
}