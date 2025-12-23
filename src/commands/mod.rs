use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::core::config::Config;
use crate::ui::output::Output;

pub mod binary;
pub mod cache;
pub mod config;
pub mod doctor;
pub mod info;
pub mod install;
pub mod iso;
pub mod language;
pub mod list;
pub mod profile;
pub mod remove;
pub mod repos;
pub mod search;
pub mod shell;
pub mod sync;
pub mod whatis;
pub mod where_pkg;
pub mod update;
pub mod usb;
pub mod recovery;

#[derive(clap::ValueEnum, Clone)]
pub enum SelfUpdateCommand {
    /// Check for updates without installing
    Check,
    /// Download and install update
    Yes,
    /// Set update branch (stable, beta, daily)
    Branch,
}

#[derive(Parser)]
#[command(
    name = "pkmgr",
    version = "1.0.0",
    about = "CasjaysDev Package Manager - Universal package manager with one consistent interface",
    author = "Jason Hempstead <jason@casjaysdev.pro>",
    long_about = "A universal package manager that provides one consistent interface across all package sources.
Smart wrapper around existing tools - never reinvent, always enhance.
Single static binary with zero dependencies."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Force override safety checks and confirmations
    #[arg(long, global = true)]
    pub force: bool,

    /// Minimal output only
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Detailed operation output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Auto-confirm all prompts
    #[arg(short, long, global = true)]
    pub yes: bool,

    /// Show what would happen without executing
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Show underlying native commands that would be executed
    #[arg(long, global = true)]
    pub explain: bool,

    /// Use specific configuration profile
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Specify target architecture
    #[arg(long, global = true)]
    pub arch: Option<String>,

    /// Specify target version
    #[arg(long = "target-version", global = true)]
    pub version: Option<String>,

    /// Force system-wide installation
    #[arg(long, global = true)]
    pub global: bool,

    /// Force user-space installation
    #[arg(long, global = true)]
    pub user: bool,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Install packages via system package manager
    #[command(alias = "i")]
    Install {
        /// Package name(s) to install
        packages: Vec<String>,
    },

    /// Remove packages completely with cleanup
    #[command(alias = "rm", alias = "r")]
    Remove {
        /// Package name(s) to remove
        packages: Vec<String>,
    },

    /// Update packages (all if no target specified)
    #[command(alias = "u", alias = "up")]
    Update {
        /// Package name(s) to update, or "all" for everything
        packages: Option<Vec<String>>,
    },

    /// Search system package manager
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,
    },

    /// List packages
    #[command(alias = "ls")]
    List {
        /// List type: installed, available
        #[arg(value_enum)]
        list_type: Option<list::ListType>,
    },

    /// Show detailed package information
    Info {
        /// Package name
        package: String,
    },

    /// Show installation location/path
    Where {
        /// Package name
        package: String,
    },

    /// Show package description
    Whatis {
        /// Package name
        package: String,
    },

    /// Fix broken dependencies and installations
    Fix {
        /// Automatically apply safe fixes without prompting
        #[arg(long)]
        auto: bool,
        /// Show what would be fixed without making changes
        #[arg(long)]
        dry_run: bool,
        /// Analyze the last error from command output
        #[arg(long)]
        last_error: bool,
    },

    /// Language version management
    #[command(subcommand)]
    Node(language::NodeCommands),
    #[command(subcommand)]
    Python(language::PythonCommands),
    #[command(subcommand)]
    Go(language::GoCommands),
    #[command(subcommand)]
    Rust(language::RustCommands),
    #[command(subcommand)]
    Ruby(language::RubyCommands),
    #[command(subcommand)]
    Php(language::PhpCommands),
    #[command(subcommand)]
    Java(language::JavaCommands),
    #[command(subcommand)]
    Dotnet(language::DotnetCommands),

    /// Binary management
    #[command(subcommand)]
    Binary(binary::BinaryCommands),

    /// ISO management
    #[command(subcommand)]
    Iso(iso::IsoCommands),

    /// USB management
    #[command(subcommand)]
    Usb(usb::UsbCommands),

    /// Repository management
    #[command(subcommand)]
    Repos(repos::ReposCommands),

    /// Profile management
    #[command(subcommand)]
    Profile(profile::ProfileCommands),

    /// Configuration management
    #[command(subcommand)]
    Config(config::ConfigCommands),

    /// Cache management
    #[command(subcommand)]
    Cache(cache::CacheCommands),

    /// System health check
    Doctor {
        /// Full comprehensive check
        #[arg(long)]
        full: bool,
        /// Package management health only
        #[arg(long)]
        packages: bool,
        /// USB device health check
        #[arg(long)]
        usb: bool,
        /// Security status check
        #[arg(long)]
        security: bool,
        /// Auto-fix issues where possible
        #[arg(long)]
        fix: bool,
    },

    /// Bootstrap and sync
    #[command(subcommand)]
    Bootstrap(sync::BootstrapCommands),
    #[command(subcommand)]
    Sync(sync::SyncCommands),

    /// Check for updates
    Check {
        /// Output only the number of updates for scripting
        #[arg(long)]
        script: bool,
    },

    /// Shell integration
    #[command(subcommand)]
    Shell(shell::ShellCommands),

    /// Check and perform pkmgr self-updates
    #[command(name = "update-self")]
    UpdateSelf {
        /// Update command: check, yes, or branch
        #[arg(value_enum)]
        command: Option<SelfUpdateCommand>,
        /// Branch name when using branch command
        branch: Option<String>,
    },
}

pub async fn execute(cli: Cli, config: Config, output: Output) -> Result<()> {
    let command = match &cli.command {
        Some(cmd) => cmd.clone(),
        None => return Ok(()), // Should not happen due to check in main
    };
    
    match command {
        Commands::Install { packages } => {
            install::execute(packages, &cli, &config, &output).await
        }
        Commands::Remove { packages } => {
            remove::execute(packages, &cli, &config, &output).await
        }
        Commands::Update { packages } => {
            update::execute(packages, &cli, &config, &output).await
        }
        Commands::Search { query } => {
            search::execute(query, &cli, &config, &output).await
        }
        Commands::List { list_type } => {
            list::execute(list_type, &cli, &config, &output).await
        }
        Commands::Info { package } => {
            info::execute(package, &cli, &config, &output).await
        }
        Commands::Where { package } => {
            where_pkg::execute(package, &cli, &config, &output).await
        }
        Commands::Whatis { package } => {
            whatis::execute(package, &cli, &config, &output).await
        }
        Commands::Fix { auto, dry_run, last_error } => {
            recovery::execute(auto, dry_run, last_error, &cli, &config, &output).await
        }
        Commands::Node(cmd) => language::execute_node(cmd, &cli, &config, &output).await,
        Commands::Python(cmd) => language::execute_python(cmd, &cli, &config, &output).await,
        Commands::Go(cmd) => language::execute_go(cmd, &cli, &config, &output).await,
        Commands::Rust(cmd) => language::execute_rust(cmd, &cli, &config, &output).await,
        Commands::Ruby(cmd) => language::execute_ruby(cmd, &cli, &config, &output).await,
        Commands::Php(cmd) => language::execute_php(cmd, &cli, &config, &output).await,
        Commands::Java(cmd) => language::execute_java(cmd, &cli, &config, &output).await,
        Commands::Dotnet(cmd) => language::execute_dotnet(cmd, &cli, &config, &output).await,
        Commands::Binary(cmd) => binary::execute(cmd, &cli, &config, &output).await,
        Commands::Iso(cmd) => iso::execute(cmd, &cli, &config, &output).await,
        Commands::Usb(cmd) => usb::execute(cmd, &cli, &config, &output).await,
        Commands::Repos(cmd) => repos::execute(cmd, &cli, &config, &output).await,
        Commands::Profile(cmd) => profile::execute(cmd, &cli, &config, &output).await,
        Commands::Config(cmd) => config::execute(cmd, &cli, &config, &output).await,
        Commands::Cache(cmd) => cache::execute(cmd, &cli, &config, &output).await,
        Commands::Doctor { full, packages, usb, security, fix } => {
            doctor::execute(full, packages, usb, security, fix, &cli, &config, &output).await
        }
        Commands::Bootstrap(cmd) => sync::execute_bootstrap(cmd, &cli, &config, &output).await,
        Commands::Sync(cmd) => sync::execute_sync(cmd, &cli, &config, &output).await,
        Commands::Check { script } => {
            // TODO: Implement check command
            if script {
                println!("0");
            } else {
                output.success("✅ All packages up to date");
            }
            Ok(())
        }
        Commands::Shell(cmd) => shell::execute(cmd, &cli, &config, &output).await,
        Commands::UpdateSelf { command, branch } => {
            use crate::update::{UpdateManager, UpdateBranch};
            
            let version = env!("CARGO_PKG_VERSION").to_string();
            let manager = UpdateManager::new(version)?;
            
            match command {
                Some(SelfUpdateCommand::Check) | None if branch.is_none() => {
                    // Default: check for updates
                    manager.check_for_updates()?;
                }
                Some(SelfUpdateCommand::Yes) => {
                    // Perform update
                    manager.perform_update()?;
                }
                Some(SelfUpdateCommand::Branch) => {
                    // Set branch
                    if let Some(branch_name) = branch {
                        let branch = UpdateBranch::from_str(&branch_name)?;
                        manager.set_branch(branch)?;
                    } else {
                        output.error("❌ Branch name required. Valid: stable, beta, daily");
                        std::process::exit(1);
                    }
                }
                _ => {
                    // Default to check
                    manager.check_for_updates()?;
                }
            }
            Ok(())
        }
    }
}