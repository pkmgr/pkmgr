use anyhow::Result;
use clap::Subcommand;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;

#[derive(Debug, Subcommand, Clone)]
pub enum ConfigCommands {
    List,
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
    Reset,
}

pub async fn execute(cmd: ConfigCommands, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    output.info("⚙️ Configuration management");
    Ok(())
}
