// Allow common clippy warnings that are intentional in this codebase
#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::module_inception)]

use std::env;
use std::path::Path;

mod commands;
mod core;
mod languages;
mod managers;
mod ui;
mod utils;
mod iso;
mod usb;
mod repos;
mod profile;
mod recovery;
mod shell;
mod cache;
mod doctor;
mod binary;
mod update;

use anyhow::Result;
use clap::Parser;

use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::detector::SymlinkDetector;
use crate::ui::output::Output;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging and error handling
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("❌ Fatal error: {}", panic_info);
        std::process::exit(1);
    }));

    // Handle signals for clean shutdown
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        tokio::spawn(async {
            let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
            let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

            tokio::select! {
                _ = sigint.recv() => {
                    eprintln!("\n⚠️ Interrupted by user");
                    std::process::exit(130);
                }
                _ = sigterm.recv() => {
                    eprintln!("\n⚠️ Terminated");
                    std::process::exit(143);
                }
            }
        });
    }

    // Get the command name from argv[0] for symlink detection
    let program_name = env::args()
        .next()
        .map(|arg0| {
            Path::new(&arg0)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("pkmgr")
                .to_string()
        })
        .unwrap_or_else(|| "pkmgr".to_string());

    // Initialize configuration
    let config = Config::load().await?;
    let output = Output::new(config.defaults.color_output.clone(), config.defaults.emoji_enabled);

    // Check if we were called as a language command (symlink)
    let detector = SymlinkDetector::new();
    if let Some(language) = detector.detect_language(&program_name) {
        // Handle language command invocation
        return languages::handle_language_command(language, &config, &output).await;
    }

    // Parse CLI arguments for normal pkmgr invocation
    let cli = Cli::parse();

    // If no command provided, show help
    if cli.command.is_none() {
        use clap::CommandFactory;
        Cli::command().print_help()?;
        return Ok(());
    }

    // Execute the command
    commands::execute(cli, config, output).await
}