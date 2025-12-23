use anyhow::{Context, Result};
use clap::Subcommand;
use std::path::PathBuf;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::ui::output::Output;
use crate::utils::download::{Downloader, GitHubClient};
use crate::utils::archive::Extractor;

#[derive(Debug, Subcommand, Clone)]
pub enum BinaryCommands {
    /// Search for binary releases
    Search { query: String },
    /// Install from GitHub/GitLab
    Install { repo: String },
    /// List installed binaries
    List,
    /// Update binaries
    Update { name: Option<String> },
    /// Remove binary
    Remove { name: String },
    /// Show repository information
    Info { repo: String },
}

pub async fn execute(cmd: BinaryCommands, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    match cmd {
        BinaryCommands::Search { query } => {
            search_binaries(query, config, output).await
        }
        BinaryCommands::Install { repo } => {
            install_binary(repo, config, output).await
        }
        BinaryCommands::List => {
            list_binaries(config, output).await
        }
        BinaryCommands::Update { name } => {
            update_binaries(name, config, output).await
        }
        BinaryCommands::Remove { name } => {
            remove_binary(name, config, output).await
        }
        BinaryCommands::Info { repo } => {
            show_binary_info(repo, config, output).await
        }
    }
}

async fn search_binaries(query: String, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("🔍 Searching for binaries: {}", query));

    // TODO: Implement GitHub/GitLab search API
    output.info("Search feature coming soon. Use 'pkmgr binary install user/repo' to install directly.");

    Ok(())
}

async fn install_binary(repo: String, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("📦 Installing binary from: {}", repo));

    // Parse repository format (user/repo[@version])
    let (repo_path, version) = if let Some(at_pos) = repo.find('@') {
        let (r, v) = repo.split_at(at_pos);
        (r.to_string(), Some(v[1..].to_string()))
    } else {
        (repo.clone(), None)
    };

    // Split owner and repo name
    let parts: Vec<&str> = repo_path.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid repository format. Use: owner/repo or owner/repo@version");
    }

    let owner = parts[0];
    let repo_name = parts[1];

    // Detect platform
    let platform_info = PlatformInfo::detect();
    let platform_str = match platform_info.platform {
        crate::core::platform::Platform::Linux => "linux",
        crate::core::platform::Platform::MacOs => "darwin",
        crate::core::platform::Platform::Windows => "windows",
        _ => "unknown",
    };

    let arch_str = match platform_info.architecture {
        crate::core::platform::Architecture::X86_64 => "x86_64",
        crate::core::platform::Architecture::Aarch64 => "aarch64",
        _ => "x86_64",
    };

    output.progress(&format!("Fetching release information for {}/{}", owner, repo_name));

    // Get release information
    let github_client = GitHubClient::new()?;
    let release = if let Some(ver) = version {
        // Get specific version
        let releases = github_client.get_releases(owner, repo_name).await?;
        releases.into_iter()
            .find(|r| r.tag_name == ver || r.tag_name == format!("v{}", ver))
            .ok_or_else(|| anyhow::anyhow!("Version {} not found", ver))?
    } else {
        // Get latest release
        github_client.get_latest_release(owner, repo_name).await?
    };

    output.info(&format!("Found release: {} {}", release.name, release.tag_name));

    // Select appropriate asset
    let asset = github_client.select_asset(&release, platform_str, arch_str)
        .ok_or_else(|| anyhow::anyhow!("No suitable binary found for {}/{}", platform_str, arch_str))?;

    output.progress(&format!("Selected asset: {} ({:.2} MB)", asset.name, asset.size as f64 / 1_000_000.0));

    // Download the asset
    let cache_dir = config.get_cache_dir()?;
    let download_path = cache_dir.join(&asset.name);

    let downloader = Downloader::new(config.defaults.emoji_enabled)?;

    output.download_start(&asset.name, Some(asset.size));
    downloader.download_file(&asset.browser_download_url, &download_path).await?;

    // Extract if needed
    let install_dir = config.get_install_dir()?.join("bin");
    tokio::fs::create_dir_all(&install_dir).await?;

    let binary_path = install_dir.join(repo_name);

    if asset.name.ends_with(".tar.gz") || asset.name.ends_with(".zip") {
        output.progress("Extracting binary from archive");
        let extractor = Extractor::new();
        extractor.extract_single_binary(&download_path, repo_name, &binary_path).await?;
    } else {
        // Direct binary download
        output.progress("Installing binary");
        tokio::fs::copy(&download_path, &binary_path).await?;

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = tokio::fs::metadata(&binary_path).await?.permissions();
            perms.set_mode(0o755);
            tokio::fs::set_permissions(&binary_path, perms).await?;
        }
    }

    // Save installation info
    save_binary_info(owner, repo_name, &release.tag_name, &asset.browser_download_url, config).await?;

    output.success(&format!("✅ Successfully installed {} {}", repo_name, release.tag_name));
    output.info(&format!("📁 Installed to: {}", binary_path.display()));

    Ok(())
}

async fn list_binaries(config: &Config, output: &Output) -> Result<()> {
    output.print_header("📋 Installed Binaries");

    let data_dir = config.get_data_dir()?;
    let binaries_file = data_dir.join("binaries").join("installed.toml");

    if !binaries_file.exists() {
        output.info("No binaries installed yet.");
        return Ok(());
    }

    let content = tokio::fs::read_to_string(&binaries_file).await?;
    let binaries: toml::Value = toml::from_str(&content)?;

    if let Some(table) = binaries.as_table() {
        let headers = vec!["Binary", "Version", "Source", "Installed"];
        let mut rows = Vec::new();

        for (name, info) in table {
            if let Some(info_table) = info.as_table() {
                let version = info_table.get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                let source = info_table.get("repository")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                let installed_date = info_table.get("installed_date")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                rows.push(vec![
                    name.clone(),
                    version.to_string(),
                    source.to_string(),
                    installed_date.to_string(),
                ]);
            }
        }

        if !rows.is_empty() {
            output.print_table(&headers, &rows);
        } else {
            output.info("No binaries installed yet.");
        }
    }

    Ok(())
}

async fn update_binaries(name: Option<String>, config: &Config, output: &Output) -> Result<()> {
    if let Some(name) = name {
        output.print_header(&format!("🔄 Updating binary: {}", name));
        // TODO: Implement single binary update
        output.info("Update feature coming soon");
    } else {
        output.print_header("🔄 Updating all binaries");
        // TODO: Implement all binaries update
        output.info("Update feature coming soon");
    }

    Ok(())
}

async fn remove_binary(name: String, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("🗑️ Removing binary: {}", name));

    let install_dir = config.get_install_dir()?.join("bin");
    let binary_path = install_dir.join(&name);

    if binary_path.exists() {
        tokio::fs::remove_file(&binary_path).await?;
        output.success(&format!("✅ Removed {}", name));

        // Remove from tracking
        remove_binary_info(&name, config).await?;
    } else {
        output.error(&format!("Binary '{}' not found", name));
    }

    Ok(())
}

async fn show_binary_info(repo: String, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("ℹ️ Binary info: {}", repo));

    // Parse repository
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid repository format. Use: owner/repo");
    }

    let owner = parts[0];
    let repo_name = parts[1];

    // Get release information
    let github_client = GitHubClient::new()?;
    let release = github_client.get_latest_release(owner, repo_name).await?;

    output.print_section("Release Information");
    output.info(&format!("📦 Repository: {}/{}", owner, repo_name));
    output.info(&format!("🏷️ Latest Version: {}", release.tag_name));
    output.info(&format!("📝 Release Name: {}", release.name));
    output.info(&format!("📅 Pre-release: {}", if release.prerelease { "Yes" } else { "No" }));

    output.print_section("Available Assets");
    for asset in &release.assets {
        output.info(&format!("  📎 {} ({:.2} MB)", asset.name, asset.size as f64 / 1_000_000.0));
    }

    Ok(())
}

async fn save_binary_info(owner: &str, name: &str, version: &str, url: &str, config: &Config) -> Result<()> {
    let data_dir = config.get_data_dir()?;
    let binaries_dir = data_dir.join("binaries");
    tokio::fs::create_dir_all(&binaries_dir).await?;

    let binaries_file = binaries_dir.join("installed.toml");

    let mut binaries: toml::Value = if binaries_file.exists() {
        let content = tokio::fs::read_to_string(&binaries_file).await?;
        toml::from_str(&content)?
    } else {
        toml::Value::Table(toml::map::Map::new())
    };

    if let Some(table) = binaries.as_table_mut() {
        let mut info = toml::map::Map::new();
        info.insert("repository".to_string(), toml::Value::String(format!("{}/{}", owner, name)));
        info.insert("version".to_string(), toml::Value::String(version.to_string()));
        info.insert("download_url".to_string(), toml::Value::String(url.to_string()));
        info.insert("installed_date".to_string(), toml::Value::String(chrono::Utc::now().to_rfc3339()));

        table.insert(name.to_string(), toml::Value::Table(info));
    }

    let content = toml::to_string_pretty(&binaries)?;
    tokio::fs::write(&binaries_file, content).await?;

    Ok(())
}

async fn remove_binary_info(name: &str, config: &Config) -> Result<()> {
    let data_dir = config.get_data_dir()?;
    let binaries_file = data_dir.join("binaries").join("installed.toml");

    if !binaries_file.exists() {
        return Ok(());
    }

    let content = tokio::fs::read_to_string(&binaries_file).await?;
    let mut binaries: toml::Value = toml::from_str(&content)?;

    if let Some(table) = binaries.as_table_mut() {
        table.remove(name);
    }

    let content = toml::to_string_pretty(&binaries)?;
    tokio::fs::write(&binaries_file, content).await?;

    Ok(())
}