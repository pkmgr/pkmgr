use anyhow::Result;
use clap::Subcommand;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::Platform;
use crate::ui::output::Output;
use crate::repos::manager::RepositoryManager;

#[derive(Debug, Subcommand, Clone)]
pub enum ReposCommands {
    /// List all configured repositories
    List {
        /// Show specific repository details
        #[arg(long)]
        name: Option<String>,
    },
    /// Add a repository
    Add {
        /// Repository to add (URL, PPA, or package name)
        repo: String,
    },
    /// Remove a repository
    Remove {
        /// Repository name to remove
        repo: String,
    },
    /// Update repository metadata
    Update,
    /// Show repository information
    Info {
        /// Repository name
        repo: String,
    },
}

pub async fn execute(cmd: ReposCommands, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    let platform = Platform::detect()?;
    let manager = RepositoryManager::new(output.clone(), platform);

    match cmd {
        ReposCommands::List { name } => {
            if let Some(name) = name {
                show_repository_details(&manager, &name, output)?;
            } else {
                list_repositories(&manager, output)?;
            }
        }
        ReposCommands::Add { repo } => {
            output.section("Adding Repository");
            manager.add(&repo).await?;
        }
        ReposCommands::Remove { repo } => {
            output.section("Removing Repository");
            manager.remove(&repo).await?;
        }
        ReposCommands::Update => {
            output.section("Updating Repository Metadata");
            manager.update_cache().await?;
        }
        ReposCommands::Info { repo } => {
            show_repository_details(&manager, &repo, output)?;
        }
    }

    Ok(())
}

fn list_repositories(manager: &RepositoryManager, output: &Output) -> Result<()> {
    output.section("Configured Repositories");

    let repos = manager.list()?;

    if repos.is_empty() {
        output.info("No additional repositories configured");
        return Ok(());
    }

    // Group by trust level
    let mut official = Vec::new();
    let mut verified = Vec::new();
    let mut community = Vec::new();
    let mut unknown = Vec::new();

    for repo in repos {
        match repo.metadata.trust_level {
            crate::repos::TrustLevel::Official => official.push(repo),
            crate::repos::TrustLevel::Verified => verified.push(repo),
            crate::repos::TrustLevel::Community => community.push(repo),
            _ => unknown.push(repo),
        }
    }

    if !official.is_empty() {
        output.info("Official Repositories:");
        for repo in official {
            let status = if repo.enabled { "enabled" } else { "disabled" };
            output.info(&format!("  {} - {} [{}]", repo.name, repo.url, status));
        }
    }

    if !verified.is_empty() {
        output.info("\nVerified Vendor Repositories:");
        for repo in verified {
            let status = if repo.enabled { "enabled" } else { "disabled" };
            let vendor = repo.metadata.vendor.as_ref().unwrap_or(&repo.name);
            output.info(&format!("  {} ({}) - {} [{}]", repo.name, vendor, repo.url, status));
        }
    }

    if !community.is_empty() {
        output.info("\nCommunity Repositories:");
        for repo in community {
            let status = if repo.enabled { "enabled" } else { "disabled" };
            output.info(&format!("  {} - {} [{}]", repo.name, repo.url, status));
        }
    }

    if !unknown.is_empty() {
        output.info("\nUser-Added Repositories:");
        for repo in unknown {
            let status = if repo.enabled { "enabled" } else { "disabled" };
            output.info(&format!("  {} - {} [{}]", repo.name, repo.url, status));
        }
    }

    Ok(())
}

fn show_repository_details(manager: &RepositoryManager, name: &str, output: &Output) -> Result<()> {
    let repos = manager.list()?;

    if let Some(repo) = repos.iter().find(|r| r.name == name) {
        output.section(&format!("Repository: {}", repo.name));

        output.info(&format!("URL: {}", repo.url));
        output.info(&format!("Type: {}", repo.repo_type));
        output.info(&format!("Status: {}", if repo.enabled { "Enabled" } else { "Disabled" }));
        output.info(&format!("Trust Level: {}", repo.metadata.trust_level));

        if let Some(ref vendor) = repo.metadata.vendor {
            output.info(&format!("Vendor: {}", vendor));
        }

        if let Some(ref desc) = repo.metadata.description {
            output.info(&format!("Description: {}", desc));
        }

        if !repo.suites.is_empty() {
            output.info(&format!("Suites: {}", repo.suites.join(", ")));
        }

        if !repo.components.is_empty() {
            output.info(&format!("Components: {}", repo.components.join(", ")));
        }

        if let Some(ref key) = repo.gpg_key {
            output.info("\nGPG Key Information:");
            if !key.fingerprint.is_empty() {
                output.info(&format!("  Fingerprint: {}", key.fingerprint));
            }
            if let Some(ref url) = key.key_url {
                output.info(&format!("  Key URL: {}", url));
            }
            if let Some(expires) = key.expires {
                output.info(&format!("  Expires: {}", expires.format("%Y-%m-%d")));
            }

            if repo.is_expired() {
                output.error("  WARNING: GPG key is expired!");
            } else if repo.needs_refresh() {
                output.warn("  GPG key needs refresh (older than 30 days)");
            }
        }

        if let Some(count) = repo.metadata.package_count {
            output.info(&format!("\nPackages: {} available", count));
        }
    } else {
        output.error(&format!("Repository '{}' not found", name));
    }

    Ok(())
}
