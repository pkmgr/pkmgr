use anyhow::{Context, Result};
use clap::Subcommand;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;
use crate::shell::{ShellType, integration::ShellIntegration, completion::CompletionGenerator, detector::ShellDetector};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Subcommand, Clone)]
pub enum ShellCommands {
    /// Load shell integration script
    Load {
        /// Shell type (auto-detected if not specified)
        shell: Option<String>
    },
    /// Generate shell completions
    Completions {
        /// Shell type (bash, zsh, fish, powershell)
        shell: String
    },
    /// Add ~/.local/bin to PATH
    Add,
    /// Remove ~/.local/bin from PATH
    Remove,
    /// Show shell environment status
    Env,
}

pub async fn execute(cmd: ShellCommands, _cli: &Cli, _config: &Config, output: &Output) -> Result<()> {
    match cmd {
        ShellCommands::Load { shell } => {
            load_integration(shell, output).await
        }
        ShellCommands::Completions { shell } => {
            generate_completions(&shell, output).await
        }
        ShellCommands::Add => {
            modify_path(true, output).await
        }
        ShellCommands::Remove => {
            modify_path(false, output).await
        }
        ShellCommands::Env => {
            show_environment(output).await
        }
    }
}

async fn load_integration(shell_name: Option<String>, output: &Output) -> Result<()> {
    let shell = if let Some(name) = shell_name {
        ShellType::from_str(&name)?
    } else {
        ShellDetector::detect_default_shell()
            .unwrap_or_else(|_| {
                output.warn("⚠️  Could not detect shell type");
                output.info("Defaulting to Bash. Specify shell with: pkmgr shell load <shell>");
                ShellType::Bash
            })
    };

    let integration = ShellIntegration::new(shell.clone(), output.clone());
    let script = integration.generate_script();

    // Output the script for evaluation
    println!("{}", script);

    // Also provide installation instructions on stderr
    if ShellDetector::is_integration_installed(&shell) {
        eprintln!("✅ Shell integration appears to be installed");
    } else {
        eprintln!("\n💡 To install permanently:");
        eprintln!("{}", ShellDetector::suggest_installation(&shell));
    }

    Ok(())
}

async fn generate_completions(shell_name: &str, output: &Output) -> Result<()> {
    let shell = ShellType::from_str(shell_name)?;

    let generator = CompletionGenerator::new(shell.clone(), output.clone());
    let completions = generator.generate_custom();

    // Determine where to install
    if let Some(comp_dir) = shell.completion_dir() {
        let comp_path = PathBuf::from(&comp_dir).join("pkmgr");

        output.info(&format!("📝 Installing completions to: {}", comp_path.display()));

        // Create directory if needed
        fs::create_dir_all(&comp_dir)
            .context("Failed to create completion directory")?;

        // Write completion file
        fs::write(&comp_path, completions)
            .context("Failed to write completion file")?;

        // Make executable for shells that need it
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&comp_path)?.permissions();
            perms.set_mode(0o644);
            fs::set_permissions(&comp_path, perms)?;
        }

        output.success(&format!("✅ Completions installed for {}", shell.display_name()));

        // Shell-specific reload instructions
        match shell {
            ShellType::Bash => {
                output.info("💡 Reload with: source ~/.bashrc");
            }
            ShellType::Zsh => {
                output.info("💡 Reload with: source ~/.zshrc");
                output.info("   Or: rm -f ~/.zcompdump && compinit");
            }
            ShellType::Fish => {
                output.info("💡 Completions will be available in new shells");
            }
            _ => {}
        }
    } else {
        // Output to stdout for manual installation
        println!("{}", completions);
        output.info(&format!("💡 {} doesn't have a standard completion directory", shell.display_name()));
        output.info("   Save the output above to an appropriate location");
    }

    Ok(())
}

async fn modify_path(add: bool, output: &Output) -> Result<()> {
    let shell = ShellDetector::detect_default_shell()
        .unwrap_or(ShellType::Bash);

    let integration = ShellIntegration::new(shell.clone(), output.clone());
    let script = integration.generate_path_script(add);

    // Output the script for evaluation
    println!("{}", script);

    if add {
        eprintln!("✅ PATH modification script generated");
        eprintln!("💡 To apply: eval \"$(pkmgr shell add)\"");
    } else {
        eprintln!("✅ PATH removal script generated");
        eprintln!("💡 To apply: eval \"$(pkmgr shell remove)\"");
    }

    Ok(())
}

async fn show_environment(output: &Output) -> Result<()> {
    let shell = ShellDetector::detect_default_shell()
        .unwrap_or(ShellType::Bash);

    let integration = ShellIntegration::new(shell, output.clone());
    integration.display_env();

    Ok(())
}
