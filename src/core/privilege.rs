use anyhow::{Context, Result, bail};
use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

use crate::core::platform::{Platform, PlatformInfo};
use crate::ui::output::Output;
use crate::ui::prompt::Prompt;

/// Privilege escalation detection and management
pub struct PrivilegeManager {
    platform: PlatformInfo,
    output: Output,
    prompt: Prompt,
}

impl PrivilegeManager {
    pub fn new(output: Output) -> Result<Self> {
        let platform = Platform::detect()?;
        let emoji_enabled = output.emoji_enabled;

        Ok(Self {
            platform,
            output,
            prompt: Prompt::new(emoji_enabled),
        })
    }

    /// Check if running as root/administrator
    pub fn is_root(&self) -> bool {
        #[cfg(unix)]
        {
            unsafe { libc::geteuid() == 0 }
        }

        #[cfg(windows)]
        {
            // Check if running as administrator on Windows
            self.check_windows_admin()
        }

        #[cfg(not(any(unix, windows)))]
        {
            false
        }
    }

    /// Check if user has sudo/admin privileges
    pub fn has_privileges(&self) -> PrivilegeStatus {
        if self.is_root() {
            return PrivilegeStatus::Root;
        }

        match self.platform.platform {
            Platform::Linux | Platform::MacOs => self.check_sudo_access(),
            Platform::Windows => self.check_windows_elevation(),
            _ => PrivilegeStatus::None,
        }
    }

    /// Check sudo access on Unix systems
    fn check_sudo_access(&self) -> PrivilegeStatus {
        // First check if sudo exists
        if which::which("sudo").is_err() {
            return PrivilegeStatus::None;
        }

        // Test passwordless sudo
        let test = Command::new("sudo")
            .args(&["-n", "true"])
            .output();

        if let Ok(output) = test {
            if output.status.success() {
                return PrivilegeStatus::Passwordless;
            }
        }

        // Check if user is in sudo/wheel/admin group
        if self.is_in_sudo_group() {
            return PrivilegeStatus::Available;
        }

        PrivilegeStatus::None
    }

    /// Check if user is in sudo/wheel/admin group
    fn is_in_sudo_group(&self) -> bool {
        #[cfg(unix)]
        {
            let groups_output = Command::new("groups")
                .output();

            if let Ok(output) = groups_output {
                let groups = String::from_utf8_lossy(&output.stdout);
                return groups.contains("sudo") ||
                       groups.contains("wheel") ||
                       groups.contains("admin");
            }
        }

        false
    }

    /// Check Windows administrator status
    #[cfg(windows)]
    fn check_windows_admin(&self) -> bool {
        // Use Windows API to check admin status
        // This is a simplified check
        Command::new("net")
            .args(&["session"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Check Windows elevation capability
    #[cfg(windows)]
    fn check_windows_elevation(&self) -> PrivilegeStatus {
        if self.check_windows_admin() {
            PrivilegeStatus::Root
        } else {
            // Check if UAC elevation is possible
            PrivilegeStatus::Available
        }
    }

    #[cfg(not(windows))]
    fn check_windows_elevation(&self) -> PrivilegeStatus {
        PrivilegeStatus::None
    }

    /// Escalate privileges for a command
    pub fn escalate_command(&self, command: &[String], operation: &str) -> Result<Vec<String>> {
        let status = self.has_privileges();

        match status {
            PrivilegeStatus::Root => {
                // Already root, no escalation needed
                Ok(command.to_vec())
            }
            PrivilegeStatus::Passwordless => {
                // Can use sudo without password
                let mut escalated = vec!["sudo".to_string()];
                escalated.extend_from_slice(command);
                Ok(escalated)
            }
            PrivilegeStatus::Available => {
                // Need password for sudo
                self.handle_password_required(command, operation)
            }
            PrivilegeStatus::None => {
                // No privileges available
                self.handle_no_privileges(operation)
            }
        }
    }

    /// Handle case where password is required
    fn handle_password_required(&self, command: &[String], operation: &str) -> Result<Vec<String>> {
        // Check if we're in CI/CD environment
        if self.is_ci_environment() {
            bail!("Cannot escalate privileges in CI/CD environment. Run with appropriate permissions.");
        }

        // Check if TTY is available
        if !atty::is(atty::Stream::Stdin) {
            bail!("No TTY available for password prompt. Cannot escalate privileges.");
        }

        self.output.warn(&format!("⚠️  {} requires administrator privileges", operation));

        if self.prompt.confirm("Proceed with sudo (will prompt for password)?")? {
            let mut escalated = vec!["sudo".to_string()];
            escalated.extend_from_slice(command);
            Ok(escalated)
        } else {
            bail!("Operation cancelled - requires privileges");
        }
    }

    /// Handle case where no privileges are available
    fn handle_no_privileges(&self, operation: &str) -> Result<Vec<String>> {
        self.output.error(&format!("❌ {} requires administrator privileges", operation));
        self.output.info("Options:");
        self.output.info("  1. Run pkmgr with sudo: sudo pkmgr ...");
        self.output.info("  2. Use --user flag for user installation");
        self.output.info("  3. Ask administrator to add you to sudo group");

        bail!("Insufficient privileges for {}", operation);
    }

    /// Check if running in CI/CD environment
    fn is_ci_environment(&self) -> bool {
        // Check common CI environment variables
        std::env::var("CI").is_ok() ||
        std::env::var("CONTINUOUS_INTEGRATION").is_ok() ||
        std::env::var("GITHUB_ACTIONS").is_ok() ||
        std::env::var("GITLAB_CI").is_ok() ||
        std::env::var("JENKINS_HOME").is_ok() ||
        std::env::var("TRAVIS").is_ok() ||
        std::env::var("CIRCLECI").is_ok()
    }

    /// Configure sudo for passwordless pkmgr execution
    pub fn configure_sudo(&self) -> Result<()> {
        if !self.is_root() {
            bail!("Sudo configuration requires root privileges. Run with: sudo pkmgr privilege configure");
        }

        self.output.section("Configuring Passwordless sudo for pkmgr");

        // Get pkmgr binary path
        let pkmgr_path = std::env::current_exe()
            .context("Failed to determine pkmgr path")?;

        // Create sudoers.d file content
        let content = self.generate_sudoers_content(&pkmgr_path)?;

        // Validate content with visudo
        self.validate_sudoers_content(&content)?;

        // Write to sudoers.d
        self.write_sudoers_file(&content)?;

        self.output.success("✅ Sudo configuration complete");
        self.output.info("pkmgr can now be run without password prompts");

        Ok(())
    }

    /// Generate sudoers configuration content
    fn generate_sudoers_content(&self, pkmgr_path: &Path) -> Result<String> {
        let pkmgr_str = pkmgr_path.to_string_lossy();

        Ok(format!(r#"# /etc/sudoers.d/pkmgr
# Generated by pkmgr - safe static binary
# Allows passwordless execution of pkmgr only
#
# This configuration is safe because:
# - pkmgr is a single static binary
# - No shell execution or script interpretation
# - All operations are memory-safe (Rust)
# - Input validation prevents injection attacks

# Allow sudo group members to run pkmgr without password
%sudo ALL=(ALL) NOPASSWD: {}

# Allow wheel group members (RedHat/Fedora)
%wheel ALL=(ALL) NOPASSWD: {}

# Allow admin group members (macOS)
%admin ALL=(ALL) NOPASSWD: {}

# Also allow if installed in user directories
%sudo ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
%wheel ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr
%admin ALL=(ALL) NOPASSWD: /home/*/bin/pkmgr

# Allow common installation paths
%sudo ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
%wheel ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
%admin ALL=(ALL) NOPASSWD: /usr/local/bin/pkmgr
"#, pkmgr_str, pkmgr_str, pkmgr_str))
    }

    /// Validate sudoers content with visudo
    fn validate_sudoers_content(&self, content: &str) -> Result<()> {
        self.output.progress("Validating sudoers configuration...");

        // Write to temp file
        let temp_file = tempfile::NamedTempFile::new()?;
        fs::write(temp_file.path(), content)?;

        // Validate with visudo
        let validation = Command::new("visudo")
            .args(&["-c", "-f"])
            .arg(temp_file.path())
            .output()
            .context("Failed to run visudo")?;

        if !validation.status.success() {
            let error = String::from_utf8_lossy(&validation.stderr);
            bail!("Sudoers validation failed: {}", error);
        }

        self.output.success("✓ Configuration validated");
        Ok(())
    }

    /// Write sudoers configuration file
    fn write_sudoers_file(&self, content: &str) -> Result<()> {
        let sudoers_path = Path::new("/etc/sudoers.d/pkmgr");

        // Create sudoers.d directory if it doesn't exist
        let sudoers_dir = Path::new("/etc/sudoers.d");
        if !sudoers_dir.exists() {
            fs::create_dir_all(sudoers_dir)
                .context("Failed to create /etc/sudoers.d")?;
        }

        // Check if file already exists
        if sudoers_path.exists() {
            self.output.warn("⚠️  Sudoers file already exists");

            if !self.prompt.confirm("Overwrite existing configuration?")? {
                bail!("Configuration cancelled");
            }

            // Backup existing file
            let backup_path = format!("{}.backup.{}",
                sudoers_path.display(),
                chrono::Utc::now().timestamp()
            );
            fs::copy(sudoers_path, &backup_path)?;
            self.output.info(&format!("📄 Backed up to: {}", backup_path));
        }

        // Write file with correct permissions
        fs::write(sudoers_path, content)
            .context("Failed to write sudoers file")?;

        // Set permissions to 0440 (r--r-----)
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(sudoers_path)?.permissions();
            perms.set_mode(0o440);
            fs::set_permissions(sudoers_path, perms)?;
        }

        self.output.success(&format!("✓ Written to: {}", sudoers_path.display()));

        Ok(())
    }

    /// Check current sudo configuration
    pub fn check_configuration(&self) -> Result<()> {
        self.output.section("Sudo Configuration Status");

        let sudoers_path = Path::new("/etc/sudoers.d/pkmgr");

        if sudoers_path.exists() {
            self.output.success("✅ pkmgr sudoers file exists");

            // Read and display content
            if let Ok(content) = fs::read_to_string(sudoers_path) {
                self.output.info("Current configuration:");
                for line in content.lines() {
                    if !line.starts_with('#') && !line.is_empty() {
                        self.output.info(&format!("  {}", line));
                    }
                }
            }
        } else {
            self.output.warn("⚠️  No pkmgr sudoers configuration found");
            self.output.info("Run 'sudo pkmgr privilege configure' to set up");
        }

        // Check current privilege status
        let status = self.has_privileges();
        self.output.section("Current Privilege Status");

        match status {
            PrivilegeStatus::Root => {
                self.output.success("✅ Running as root");
            }
            PrivilegeStatus::Passwordless => {
                self.output.success("✅ Passwordless sudo available");
            }
            PrivilegeStatus::Available => {
                self.output.info("ℹ️  Sudo available (password required)");
            }
            PrivilegeStatus::None => {
                self.output.warn("⚠️  No sudo privileges");
            }
        }

        // Check group membership
        if self.is_in_sudo_group() {
            self.output.success("✅ User is in sudo/wheel/admin group");
        } else {
            self.output.info("ℹ️  User is not in sudo/wheel/admin group");
        }

        Ok(())
    }

    /// Determine installation location based on privileges
    pub fn determine_install_location(&self, prefer_user: bool) -> InstallLocation {
        if prefer_user {
            return InstallLocation::User;
        }

        match self.has_privileges() {
            PrivilegeStatus::Root | PrivilegeStatus::Passwordless => InstallLocation::System,
            PrivilegeStatus::Available => {
                // Ask user if they want system or user install
                if !self.is_ci_environment() && atty::is(atty::Stream::Stdin) {
                    if let Ok(use_system) = self.prompt.confirm("Install system-wide (requires sudo)?") {
                        if use_system {
                            InstallLocation::System
                        } else {
                            InstallLocation::User
                        }
                    } else {
                        InstallLocation::User
                    }
                } else {
                    InstallLocation::User
                }
            }
            PrivilegeStatus::None => InstallLocation::User,
        }
    }
}

/// Privilege status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum PrivilegeStatus {
    Root,           // Running as root/administrator
    Passwordless,   // Can sudo without password
    Available,      // Can sudo but needs password
    None,          // No privileges available
}

impl PrivilegeStatus {
    pub fn can_escalate(&self) -> bool {
        matches!(self, PrivilegeStatus::Root | PrivilegeStatus::Passwordless | PrivilegeStatus::Available)
    }

    pub fn is_passwordless(&self) -> bool {
        matches!(self, PrivilegeStatus::Root | PrivilegeStatus::Passwordless)
    }
}

/// Installation location
#[derive(Debug, Clone, PartialEq)]
pub enum InstallLocation {
    System, // System-wide installation (/usr/local)
    User,   // User installation (~/.local)
}

impl InstallLocation {
    pub fn base_dir(&self) -> PathBuf {
        match self {
            InstallLocation::System => PathBuf::from("/usr/local"),
            InstallLocation::User => {
                dirs::home_dir()
                    .map(|h| h.join(".local"))
                    .unwrap_or_else(|| PathBuf::from("~/.local"))
            }
        }
    }

    pub fn bin_dir(&self) -> PathBuf {
        self.base_dir().join("bin")
    }

    pub fn share_dir(&self) -> PathBuf {
        self.base_dir().join("share").join("pkmgr")
    }

    pub fn config_dir(&self) -> PathBuf {
        match self {
            InstallLocation::System => PathBuf::from("/etc/pkmgr"),
            InstallLocation::User => {
                dirs::config_dir()
                    .map(|c| c.join("pkmgr"))
                    .unwrap_or_else(|| PathBuf::from("~/.config/pkmgr"))
            }
        }
    }
}