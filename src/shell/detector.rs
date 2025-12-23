use anyhow::{Context, Result};
use std::process::Command;
use std::env;
use crate::shell::ShellType;

pub struct ShellDetector;

impl ShellDetector {
    /// Detect the user's default shell
    pub fn detect_default_shell() -> Result<ShellType> {
        // Try multiple detection methods

        // 1. Check SHELL environment variable (most reliable on Unix)
        if let Ok(shell_path) = env::var("SHELL") {
            let shell = ShellType::from_path(&shell_path);
            if shell != ShellType::Unknown {
                return Ok(shell);
            }
        }

        // 2. Check parent process (useful when run from shell)
        #[cfg(unix)]
        {
            if let Ok(parent_pid) = Self::get_parent_pid() {
                if let Ok(parent_cmd) = Self::get_process_command(parent_pid) {
                    let shell = ShellType::from_path(&parent_cmd);
                    if shell != ShellType::Unknown {
                        return Ok(shell);
                    }
                }
            }
        }

        // 3. Check /etc/passwd on Unix
        #[cfg(unix)]
        {
            if let Ok(shell) = Self::get_shell_from_passwd() {
                return Ok(shell);
            }
        }

        // 4. Windows-specific detection
        #[cfg(windows)]
        {
            // Check if running in PowerShell
            if env::var("PSModulePath").is_ok() {
                return Ok(ShellType::PowerShell);
            }

            // Check ComSpec (usually cmd.exe)
            if let Ok(comspec) = env::var("ComSpec") {
                if comspec.to_lowercase().contains("cmd.exe") {
                    // On Windows, we'll default to PowerShell as it's more modern
                    return Ok(ShellType::PowerShell);
                }
            }
        }

        // 5. Check shell-specific environment variables
        if env::var("BASH_VERSION").is_ok() {
            return Ok(ShellType::Bash);
        }
        if env::var("ZSH_VERSION").is_ok() {
            return Ok(ShellType::Zsh);
        }
        if env::var("FISH_VERSION").is_ok() {
            return Ok(ShellType::Fish);
        }
        if env::var("NU_VERSION").is_ok() {
            return Ok(ShellType::Nushell);
        }

        // Default to bash on Unix, PowerShell on Windows
        #[cfg(unix)]
        {
            Ok(ShellType::Bash)
        }
        #[cfg(windows)]
        {
            Ok(ShellType::PowerShell)
        }
    }

    /// Get parent process ID
    #[cfg(unix)]
    fn get_parent_pid() -> Result<u32> {
        let output = Command::new("ps")
            .args(&["-p", &std::process::id().to_string(), "-o", "ppid="])
            .output()
            .context("Failed to get parent PID")?;

        let ppid_str = String::from_utf8_lossy(&output.stdout);
        let ppid = ppid_str.trim().parse::<u32>()
            .context("Failed to parse parent PID")?;

        Ok(ppid)
    }

    /// Get command line of a process
    #[cfg(unix)]
    fn get_process_command(pid: u32) -> Result<String> {
        // Try /proc first (Linux)
        let proc_path = format!("/proc/{}/comm", pid);
        if std::path::Path::new(&proc_path).exists() {
            return std::fs::read_to_string(proc_path)
                .map(|s| s.trim().to_string())
                .context("Failed to read /proc/PID/comm");
        }

        // Fall back to ps command (works on macOS and other Unix)
        let output = Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "comm="])
            .output()
            .context("Failed to get process command")?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Get shell from /etc/passwd
    #[cfg(unix)]
    fn get_shell_from_passwd() -> Result<ShellType> {
        let username = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .context("Failed to get username")?;

        let passwd = std::fs::read_to_string("/etc/passwd")
            .context("Failed to read /etc/passwd")?;

        for line in passwd.lines() {
            if line.starts_with(&format!("{}:", username)) {
                let fields: Vec<&str> = line.split(':').collect();
                if fields.len() >= 7 {
                    let shell_path = fields[6];
                    return Ok(ShellType::from_path(shell_path));
                }
            }
        }

        anyhow::bail!("User not found in /etc/passwd")
    }

    /// Check if shell integration is already installed
    pub fn is_integration_installed(shell: &ShellType) -> bool {
        for config_file in shell.config_files() {
            if let Ok(content) = std::fs::read_to_string(&config_file) {
                if content.contains("pkmgr shell integration") ||
                   content.contains("pkmgr Bash Integration") ||
                   content.contains("pkmgr Zsh Integration") ||
                   content.contains("pkmgr Fish Integration") ||
                   content.contains("pkmgr PowerShell Integration") ||
                   content.contains("pkmgr Nushell Integration") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if completions are installed
    pub fn are_completions_installed(shell: &ShellType) -> bool {
        if let Some(comp_dir) = shell.completion_dir() {
            let comp_file = std::path::PathBuf::from(comp_dir).join("pkmgr");
            return comp_file.exists();
        }

        // For shells without standard completion directories, check config files
        match shell {
            ShellType::PowerShell | ShellType::Nushell => {
                for config_file in shell.config_files() {
                    if let Ok(content) = std::fs::read_to_string(&config_file) {
                        if content.contains("pkmgr completions") ||
                           content.contains("Register-ArgumentCompleter.*pkmgr") {
                            return true;
                        }
                    }
                }
            }
            _ => {}
        }

        false
    }

    /// Suggest installation method for shell integration
    pub fn suggest_installation(shell: &ShellType) -> String {
        match shell {
            ShellType::Bash => {
                "Add to ~/.bashrc:\n  eval \"$(pkmgr shell load)\""
            }
            ShellType::Zsh => {
                "Add to ~/.zshrc:\n  eval \"$(pkmgr shell load)\""
            }
            ShellType::Fish => {
                "Add to ~/.config/fish/config.fish:\n  pkmgr shell load | source"
            }
            ShellType::PowerShell => {
                "Add to $PROFILE:\n  Invoke-Expression (& pkmgr shell load)"
            }
            ShellType::Nushell => {
                "Add to ~/.config/nushell/config.nu:\n  source (pkmgr shell load)"
            }
            ShellType::Unknown => {
                "Unable to detect shell. Specify with: pkmgr shell load <shell>"
            }
        }
        .to_string()
    }
}