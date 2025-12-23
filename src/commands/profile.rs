use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;
use crate::profile::manager::ProfileManager;
use crate::profile::exporter::{ProfileExporter, ExportFormat};
use crate::profile::importer::ProfileImporter;

#[derive(Debug, Subcommand, Clone)]
pub enum ProfileCommands {
    /// List all profiles
    List {
        /// Show specific profile details
        #[arg(long)]
        name: Option<String>,
    },

    /// Create new profile
    Create {
        /// Profile name
        name: String,

        /// Create from template (development, server, minimal, security, data-science, devops)
        #[arg(long)]
        from_template: Option<String>,

        /// Copy current system state
        #[arg(long)]
        copy_current: bool,
    },

    /// Switch to profile
    Use {
        /// Profile name
        name: String,
    },

    /// Remove profile
    Remove {
        /// Profile name
        name: String,
    },

    /// Edit profile in $EDITOR
    Edit {
        /// Profile name
        name: String,
    },

    /// Compare two profiles
    Diff {
        /// First profile
        profile1: String,

        /// Second profile
        profile2: String,
    },

    /// Export profile to file
    Export {
        /// Profile name
        name: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Export format (toml, json, yaml, shell, dockerfile)
        #[arg(short, long, default_value = "toml")]
        format: String,
    },

    /// Import profile from file
    Import {
        /// File path or URL
        source: String,

        /// Override profile name
        #[arg(long)]
        name: Option<String>,
    },

    /// Apply profile (install all packages)
    Apply {
        /// Profile name
        name: String,

        /// Skip confirmation
        #[arg(long)]
        yes: bool,
    },

    /// Show available templates
    Templates,
}

pub async fn execute(cmd: ProfileCommands, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    let mut manager = ProfileManager::new(output.clone());

    match cmd {
        ProfileCommands::List { name } => {
            if let Some(name) = name {
                manager.show(&name)?;
            } else {
                manager.list()?;
            }
        }

        ProfileCommands::Create { name, from_template, copy_current } => {
            manager.create(&name, from_template, copy_current).await?;
        }

        ProfileCommands::Use { name } => {
            manager.use_profile(&name).await?;
        }

        ProfileCommands::Remove { name } => {
            manager.remove(&name)?;
        }

        ProfileCommands::Edit { name } => {
            manager.edit(&name)?;
        }

        ProfileCommands::Diff { profile1, profile2 } => {
            manager.diff(&profile1, &profile2)?;
        }

        ProfileCommands::Export { name, output: output_path, format } => {
            let exporter = ProfileExporter::new(output.clone());
            let format = format.parse::<ExportFormat>()?;
            exporter.export(&name, &output_path, format)?;
        }

        ProfileCommands::Import { source, name } => {
            let importer = ProfileImporter::new(output.clone());
            importer.import(&source, name).await?;
        }

        ProfileCommands::Apply { name, yes } => {
            if !yes {
                output.warn(&format!("This will apply all settings and packages from profile '{}'", name));

                use crate::ui::prompt::Prompt;
                let prompt = Prompt::new(output.emoji_enabled);
                if !prompt.confirm("Continue?")? {
                    output.info("Profile application cancelled");
                    return Ok(());
                }
            }

            manager.apply(&name).await?;
        }

        ProfileCommands::Templates => {
            show_templates(output)?;
        }
    }

    Ok(())
}

fn show_templates(output: &Output) -> Result<()> {
    use crate::profile::get_profile_templates;

    output.section("Available Profile Templates");

    let templates = get_profile_templates();

    for (name, profile) in templates {
        output.info(&format!("{} - {}", name, profile.description));

        // Show some packages as examples
        let pkg_count = profile.packages.system.len() +
                       profile.packages.languages.values().map(|v| v.len()).sum::<usize>() +
                       profile.packages.binaries.len();

        if pkg_count > 0 {
            output.info(&format!("  Contains {} packages", pkg_count));
        }
    }

    output.info("\nCreate a profile from template with:");
    output.info("  pkmgr profile create <name> --from-template <template>");

    Ok(())
}
