use anyhow::Result;
use clap::Subcommand;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;

#[derive(Debug, Subcommand, Clone)]
pub enum IsoCommands {
    /// Show all supported distributions or specific distribution versions
    List {
        /// Distribution name (optional)
        distro: Option<String>,
        /// Show downloaded ISOs only
        #[arg(long)]
        downloaded: bool,
    },
    /// Download ISO
    Install {
        /// Distribution name
        distro: String,
        /// Version (optional, uses current if not specified)
        version: Option<String>,
    },
    /// Delete downloaded ISO file
    Remove {
        /// ISO filename to remove
        iso_file: String,
    },
    /// Show distribution information
    Info {
        /// Distribution name
        distro: String,
    },
    /// Verify ISO checksums and signatures
    Verify {
        /// ISO file to verify (optional, verifies all if not specified)
        iso_file: Option<String>,
    },
    /// Remove old/duplicate ISO files
    Clean,
}

pub async fn execute(cmd: IsoCommands, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    use crate::iso::manager::IsoManager;
    
    let manager = IsoManager::new(config.clone(), output.clone())?;
    
    match cmd {
        IsoCommands::List { distro, downloaded } => {
            if downloaded {
                manager.list_downloaded().await
            } else {
                manager.list(distro).await
            }
        }
        IsoCommands::Install { distro, version } => {
            manager.install(distro, version).await
        }
        IsoCommands::Remove { iso_file } => {
            manager.remove(iso_file).await
        }
        IsoCommands::Info { distro } => {
            manager.info(distro).await
        }
        IsoCommands::Verify { iso_file } => {
            manager.verify(iso_file).await
        }
        IsoCommands::Clean => {
            manager.clean().await
        }
    }
}