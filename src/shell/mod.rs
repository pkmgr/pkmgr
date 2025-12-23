pub mod completion;
pub mod integration;
pub mod detector;
pub mod symlinks;

use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Nushell,
    Unknown,
}

impl ShellType {
    /// Detect the current shell from environment
    pub fn detect() -> Self {
        // Check SHELL environment variable
        if let Ok(shell) = env::var("SHELL") {
            return Self::from_path(&shell);
        }

        // Check parent process on Windows
        #[cfg(target_os = "windows")]
        {
            if env::var("PSModulePath").is_ok() {
                return ShellType::PowerShell;
            }
        }

        // Check for shell-specific variables
        if env::var("BASH_VERSION").is_ok() {
            return ShellType::Bash;
        }
        if env::var("ZSH_VERSION").is_ok() {
            return ShellType::Zsh;
        }
        if env::var("FISH_VERSION").is_ok() {
            return ShellType::Fish;
        }
        if env::var("NU_VERSION").is_ok() {
            return ShellType::Nushell;
        }

        ShellType::Unknown
    }

    /// Parse shell type from path
    pub fn from_path(path: &str) -> Self {
        let shell_name = path.split('/').last().unwrap_or(path);

        match shell_name {
            "bash" | "sh" => ShellType::Bash,
            "zsh" => ShellType::Zsh,
            "fish" => ShellType::Fish,
            "pwsh" | "powershell" => ShellType::PowerShell,
            "nu" => ShellType::Nushell,
            _ => ShellType::Unknown,
        }
    }

    /// Parse shell type from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "bash" | "sh" => Ok(ShellType::Bash),
            "zsh" => Ok(ShellType::Zsh),
            "fish" => Ok(ShellType::Fish),
            "powershell" | "pwsh" | "ps" => Ok(ShellType::PowerShell),
            "nushell" | "nu" => Ok(ShellType::Nushell),
            _ => anyhow::bail!("Unknown shell type: {}", s),
        }
    }

    /// Get shell display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ShellType::Bash => "Bash",
            ShellType::Zsh => "Zsh",
            ShellType::Fish => "Fish",
            ShellType::PowerShell => "PowerShell",
            ShellType::Nushell => "Nushell",
            ShellType::Unknown => "Unknown",
        }
    }

    /// Get shell configuration file paths
    pub fn config_files(&self) -> Vec<String> {
        let home = dirs::home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "~".to_string());

        match self {
            ShellType::Bash => vec![
                format!("{}/.bashrc", home),
                format!("{}/.bash_profile", home),
                format!("{}/.profile", home),
            ],
            ShellType::Zsh => vec![
                format!("{}/.zshrc", home),
                format!("{}/.zprofile", home),
            ],
            ShellType::Fish => vec![
                format!("{}/.config/fish/config.fish", home),
            ],
            ShellType::PowerShell => vec![
                format!("{}/.config/powershell/profile.ps1", home),
                format!("{}/Documents/PowerShell/profile.ps1", home),
            ],
            ShellType::Nushell => vec![
                format!("{}/.config/nushell/config.nu", home),
                format!("{}/.config/nushell/env.nu", home),
            ],
            ShellType::Unknown => vec![],
        }
    }

    /// Get completion directory for shell
    pub fn completion_dir(&self) -> Option<String> {
        let home = dirs::home_dir()?;
        let home_str = home.to_string_lossy();

        match self {
            ShellType::Bash => {
                // Try common completion directories
                for dir in &[
                    "/usr/share/bash-completion/completions",
                    "/etc/bash_completion.d",
                    &format!("{}/.local/share/bash-completion/completions", home_str),
                ] {
                    if std::path::Path::new(dir).exists() {
                        return Some(dir.to_string());
                    }
                }
                None
            }
            ShellType::Zsh => {
                // Zsh uses fpath
                for dir in &[
                    "/usr/share/zsh/site-functions",
                    "/usr/local/share/zsh/site-functions",
                    &format!("{}/.zsh/completions", home_str),
                ] {
                    if std::path::Path::new(dir).exists() {
                        return Some(dir.to_string());
                    }
                }
                None
            }
            ShellType::Fish => Some(format!("{}/.config/fish/completions", home_str)),
            ShellType::PowerShell => None, // PowerShell uses module system
            ShellType::Nushell => None, // Nushell has different completion system
            ShellType::Unknown => None,
        }
    }
}