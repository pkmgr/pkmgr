use anyhow::Result;
use clap::Subcommand;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;
use crate::cache::manager::CacheManager;
use crate::cache::cleaner::CacheCleaner;
use crate::cache::stats::CacheStatistics;

#[derive(Debug, Subcommand, Clone)]
pub enum CacheCommands {
    /// Show cache contents and usage
    List,
    /// Clean cache (all or specific types)
    Clean {
        /// Clean specific cache type
        #[arg(value_enum)]
        cache_type: Option<CleanType>,
        /// Force cleanup without confirmation
        #[arg(long)]
        force: bool,
        /// Only clean expired entries
        #[arg(long)]
        expired: bool,
        /// Only clean stale entries
        #[arg(long)]
        stale: bool,
        /// Clean orphaned files
        #[arg(long)]
        orphaned: bool,
    },
    /// Show cache usage and locations
    Info,
    /// Force refresh all cached data
    Refresh,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum CleanType {
    All,
    Packages,
    Metadata,
    Repos,
    Binaries,
    Languages,
    Build,
    Temp,
    Isos,
}

pub async fn execute(cmd: CacheCommands, cli: &Cli, _config: &Config, output: &Output) -> Result<()> {
    match cmd {
        CacheCommands::List => {
            list_cache(output).await
        }
        CacheCommands::Clean { cache_type, force, expired, stale, orphaned } => {
            clean_cache(cache_type, force || cli.yes, expired, stale, orphaned, cli.dry_run, output).await
        }
        CacheCommands::Info => {
            show_cache_info(output).await
        }
        CacheCommands::Refresh => {
            refresh_cache(output).await
        }
    }
}

async fn list_cache(output: &Output) -> Result<()> {
    let manager = CacheManager::new(output.clone())?;
    manager.list()?;
    Ok(())
}

async fn clean_cache(
    cache_type: Option<CleanType>,
    force: bool,
    expired: bool,
    stale: bool,
    orphaned: bool,
    dry_run: bool,
    output: &Output,
) -> Result<()> {
    let mut cleaner = CacheCleaner::new(output.clone(), dry_run)?;

    if expired {
        cleaner.clean_expired().await?;
    } else if stale {
        cleaner.clean_stale().await?;
    } else if orphaned {
        cleaner.clean_orphaned().await?;
    } else if let Some(clean_type) = cache_type {
        match clean_type {
            CleanType::All => {
                cleaner.clean_all(force).await?;
            }
            CleanType::Packages => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::PackageDownload
                );
                cleaner.clean_type(&crate::cache::CacheType::PackageDownload, entries).await?;
            }
            CleanType::Metadata => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::PackageMetadata
                );
                cleaner.clean_type(&crate::cache::CacheType::PackageMetadata, entries).await?;
            }
            CleanType::Repos => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::RepositoryIndex
                );
                cleaner.clean_type(&crate::cache::CacheType::RepositoryIndex, entries).await?;
            }
            CleanType::Binaries => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::BinaryDownload
                );
                cleaner.clean_type(&crate::cache::CacheType::BinaryDownload, entries).await?;
            }
            CleanType::Languages => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::LanguageVersion
                );
                cleaner.clean_type(&crate::cache::CacheType::LanguageVersion, entries).await?;
            }
            CleanType::Build => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::BuildArtifact
                );
                cleaner.clean_type(&crate::cache::CacheType::BuildArtifact, entries).await?;
            }
            CleanType::Temp => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::Temporary
                );
                cleaner.clean_type(&crate::cache::CacheType::Temporary, entries).await?;
            }
            CleanType::Isos => {
                let entries = get_entries_by_type(
                    &cleaner.manager,
                    crate::cache::CacheType::IsoDownload
                );
                cleaner.clean_type(&crate::cache::CacheType::IsoDownload, entries).await?;
            }
        }
    } else {
        // Default to cleaning all
        cleaner.clean_all(force).await?;
    }

    Ok(())
}

async fn show_cache_info(output: &Output) -> Result<()> {
    let manager = CacheManager::new(output.clone())?;
    manager.info()?;
    Ok(())
}

async fn refresh_cache(output: &Output) -> Result<()> {
    let mut manager = CacheManager::new(output.clone())?;
    manager.refresh()?;
    Ok(())
}

fn get_entries_by_type(
    manager: &CacheManager,
    cache_type: crate::cache::CacheType,
) -> Vec<crate::cache::CacheEntry> {
    manager.index
        .values()
        .filter(|e| e.cache_type == cache_type)
        .cloned()
        .collect()
}
