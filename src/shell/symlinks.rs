use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::fs;
use std::os::unix::fs::symlink;
use crate::ui::output::Output;

/// Language command symlinks manager
pub struct SymlinkManager {
    output: Output,
}

impl SymlinkManager {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Set up all language command symlinks
    pub fn setup_symlinks(&self, install_dir: Option<PathBuf>) -> Result<()> {
        let pkmgr_binary = self.find_pkmgr_binary()?;
        let symlink_dir = self.get_symlink_directory(install_dir)?;

        // Ensure the symlink directory exists
        fs::create_dir_all(&symlink_dir)
            .context(format!("Failed to create directory: {}", symlink_dir.display()))?;

        self.output.section("🔗 Setting up language command symlinks");

        // Define all the symlinks to create according to CLAUDE.md spec
        let symlinks = vec![
            // Python
            ("python", "🐍"),
            ("python3", "🐍"),
            ("pip", "🐍"),
            ("pip3", "🐍"),

            // Node.js
            ("node", "📦"),
            ("npm", "📦"),
            ("npx", "📦"),
            ("yarn", "📦"),

            // Ruby
            ("ruby", "💎"),
            ("gem", "💎"),
            ("bundle", "💎"),
            ("irb", "💎"),

            // Rust
            ("cargo", "🦀"),
            ("rustc", "🦀"),
            ("rustup", "🦀"),

            // Go
            ("go", "🐹"),
            ("gofmt", "🐹"),

            // Java
            ("java", "☕"),
            ("javac", "☕"),
            ("jar", "☕"),

            // .NET
            ("dotnet", "🔷"),

            // PHP
            ("php", "🐘"),
            ("composer", "🐘"),
        ];

        let mut created_count = 0;
        let mut skipped_count = 0;

        for (command, emoji) in symlinks {
            let symlink_path = symlink_dir.join(command);

            // Check if symlink already exists and points to our binary
            if symlink_path.exists() {
                if let Ok(target) = fs::read_link(&symlink_path) {
                    if target == pkmgr_binary {
                        self.output.info(&format!("  {} {} → {} (already exists)", emoji, command, target.display()));
                        skipped_count += 1;
                        continue;
                    } else {
                        self.output.warn(&format!("  {} {} → {} (different target, replacing)", emoji, command, target.display()));
                        fs::remove_file(&symlink_path)?;
                    }
                } else {
                    self.output.warn(&format!("  {} {} (not a symlink, replacing)", emoji, command));
                    fs::remove_file(&symlink_path)?;
                }
            }

            // Create the symlink
            symlink(&pkmgr_binary, &symlink_path)
                .context(format!("Failed to create symlink: {} → {}",
                    symlink_path.display(), pkmgr_binary.display()))?;

            self.output.success(&format!("  {} {} → {}", emoji, command, pkmgr_binary.display()));
            created_count += 1;
        }

        // Summary
        if created_count > 0 {
            self.output.success(&format!("✨ Created {} symlinks", created_count));
        }
        if skipped_count > 0 {
            self.output.info(&format!("ℹ️  Skipped {} existing symlinks", skipped_count));
        }

        // Check if symlink directory is in PATH
        self.check_path_configuration(&symlink_dir)?;

        Ok(())
    }

    /// Remove all language command symlinks
    pub fn remove_symlinks(&self, install_dir: Option<PathBuf>) -> Result<()> {
        let pkmgr_binary = self.find_pkmgr_binary()?;
        let symlink_dir = self.get_symlink_directory(install_dir)?;

        self.output.section("🗑️ Removing language command symlinks");

        if !symlink_dir.exists() {
            self.output.info("No symlinks directory found");
            return Ok(());
        }

        let mut removed_count = 0;

        // Read directory and find symlinks pointing to pkmgr
        for entry in fs::read_dir(&symlink_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_symlink() {
                if let Ok(target) = fs::read_link(&path) {
                    if target == pkmgr_binary {
                        fs::remove_file(&path)?;
                        self.output.success(&format!("  🗑️ Removed {}", path.file_name().unwrap().to_string_lossy()));
                        removed_count += 1;
                    }
                }
            }
        }

        if removed_count > 0 {
            self.output.success(&format!("✨ Removed {} symlinks", removed_count));
        } else {
            self.output.info("No pkmgr symlinks found to remove");
        }

        Ok(())
    }

    /// Find the pkmgr binary path
    fn find_pkmgr_binary(&self) -> Result<PathBuf> {
        // First, try to find pkmgr in PATH
        if let Ok(pkmgr_path) = which::which("pkmgr") {
            return Ok(pkmgr_path);
        }

        // Try common installation locations
        let possible_paths = vec![
            "/usr/local/bin/pkmgr",
            "/usr/bin/pkmgr",
            "~/.local/bin/pkmgr",
            "./target/release/pkmgr", // For development
        ];

        for path_str in possible_paths {
            let path = if path_str.starts_with('~') {
                if let Some(home) = dirs::home_dir() {
                    home.join(&path_str[2..]) // Remove "~/"
                } else {
                    continue;
                }
            } else {
                PathBuf::from(path_str)
            };

            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }

        bail!("Could not find pkmgr binary. Please ensure pkmgr is installed and in PATH.")
    }

    /// Get the directory where symlinks should be created
    fn get_symlink_directory(&self, install_dir: Option<PathBuf>) -> Result<PathBuf> {
        if let Some(dir) = install_dir {
            return Ok(dir.join("bin"));
        }

        // Use ~/.local/bin as default (per CLAUDE.md spec)
        if let Some(home) = dirs::home_dir() {
            Ok(home.join(".local/bin"))
        } else {
            bail!("Could not determine home directory")
        }
    }

    /// Check if the symlink directory is in PATH
    fn check_path_configuration(&self, symlink_dir: &Path) -> Result<()> {
        let path_env = std::env::var("PATH").unwrap_or_default();
        let path_dirs: Vec<&str> = path_env.split(':').collect();

        let symlink_dir_str = symlink_dir.to_string_lossy();

        if path_dirs.contains(&symlink_dir_str.as_ref()) {
            self.output.success(&format!("✅ {} is in PATH", symlink_dir.display()));
        } else {
            self.output.warn(&format!("⚠️  {} is not in PATH", symlink_dir.display()));
            self.output.info(&format!("💡 Add to PATH: export PATH=\"{}:$PATH\"", symlink_dir.display()));
            self.output.info("💡 Or run: eval $(pkmgr shell add)");
        }

        Ok(())
    }

    /// List current symlinks
    pub fn list_symlinks(&self, install_dir: Option<PathBuf>) -> Result<()> {
        let pkmgr_binary = self.find_pkmgr_binary()?;
        let symlink_dir = self.get_symlink_directory(install_dir)?;

        self.output.section("🔗 Language Command Symlinks");

        if !symlink_dir.exists() {
            self.output.info("No symlinks directory found");
            return Ok(());
        }

        let mut symlinks = Vec::new();

        // Read directory and find symlinks pointing to pkmgr
        for entry in fs::read_dir(&symlink_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_symlink() {
                if let Ok(target) = fs::read_link(&path) {
                    if target == pkmgr_binary {
                        symlinks.push(path.file_name().unwrap().to_string_lossy().to_string());
                    }
                }
            }
        }

        if symlinks.is_empty() {
            self.output.info("No pkmgr symlinks found");
            self.output.info("Run 'pkmgr shell setup' to create symlinks");
        } else {
            symlinks.sort();
            self.output.info(&format!("Found {} symlinks in {}:", symlinks.len(), symlink_dir.display()));

            for symlink in symlinks {
                self.output.info(&format!("  🔗 {}", symlink));
            }
        }

        Ok(())
    }
}