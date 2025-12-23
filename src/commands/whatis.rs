use anyhow::{Result, Context};
use std::process::Command;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

pub async fn execute(package: String, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("📚 Package Description: {}", package));

    let mut found_info = false;

    // Check system man pages first
    output.info("🔍 Checking system documentation...");
    if let Some(man_desc) = get_man_description(&package) {
        output.success("✅ Found in system documentation:");
        output.info(&format!("📖 {}", man_desc));
        found_info = true;
    }

    // Check package manager information
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.info(&format!("🔍 Checking {} for package info...", package_manager.name()));

    match package_manager.info(&package).await {
        Ok(Some(info)) => {
            found_info = true;
            output.success(&format!("✅ Found in {}", package_manager.name()));

            output.info(&format!("📦 Name: {}", info.name));
            output.info(&format!("🏷️  Version: {}", info.version));

            if let Some(description) = &info.description {
                output.info(&format!("📚 Description: {}", description));
            } else {
                output.info("📚 Description: No description available");
            }

            output.info(&format!("📂 Source: {}", info.source));
            output.info(&format!("📥 Status: {}", if info.installed { "Installed" } else { "Available" }));
        }
        Ok(None) => {
            output.info(&format!("❌ Package '{}' not found in {}", package, package_manager.name()));
        }
        Err(e) => {
            output.warn(&format!("⚠️  Error checking package manager: {}", e));
        }
    }

    // Check if it's a command and get help text
    output.info("🔍 Checking command help...");
    if let Some(help_text) = get_command_help(&package) {
        found_info = true;
        output.success("✅ Found command help:");
        output.info(&format!("💡 {}", help_text));
    }

    // Check language-specific package managers
    output.info("🔍 Checking language package managers...");
    check_language_packages(&package, output, &mut found_info).await;

    // If nothing found, try searching
    if !found_info {
        output.warn(&format!("⚠️  No description found for '{}'", package));
        output.info("🔍 Searching for similar packages...");

        match package_manager.search(&package).await {
            Ok(search_result) => {
                if !search_result.packages.is_empty() {
                    output.info("📋 Similar packages found:");
                    for (i, pkg) in search_result.packages.iter().take(5).enumerate() {
                        let desc = pkg.description.as_deref().unwrap_or("No description");
                        output.info(&format!("  {}. {} - {}", i + 1, pkg.name, desc));
                    }
                } else {
                    output.info("❌ No similar packages found");
                }
            }
            Err(e) => {
                output.warn(&format!("⚠️  Search failed: {}", e));
            }
        }

        output.info(&format!("💡 Try: pkmgr search {} to find packages", package));
    }

    Ok(())
}

fn get_man_description(package: &str) -> Option<String> {
    // Try to get one-line description from whatis command
    #[cfg(unix)]
    {
        if let Ok(output) = Command::new("whatis").arg(package).output() {
            if output.status.success() {
                if let Ok(result) = String::from_utf8(output.stdout) {
                    let first_line = result.lines().next().unwrap_or("").trim();
                    if !first_line.is_empty() && !first_line.contains("nothing appropriate") {
                        return Some(first_line.to_string());
                    }
                }
            }
        }

        // Try man -f as fallback
        if let Ok(output) = Command::new("man").args(["-f", package]).output() {
            if output.status.success() {
                if let Ok(result) = String::from_utf8(output.stdout) {
                    let first_line = result.lines().next().unwrap_or("").trim();
                    if !first_line.is_empty() {
                        return Some(first_line.to_string());
                    }
                }
            }
        }
    }

    None
}

fn get_command_help(package: &str) -> Option<String> {
    // Try common help flags
    let help_flags = ["--help", "-h", "help", "/help", "/?"];

    for flag in &help_flags {
        if let Ok(output) = Command::new(package).arg(flag).output() {
            if output.status.success() {
                if let Ok(help_text) = String::from_utf8(output.stdout) {
                    // Get first meaningful line (usually a description)
                    for line in help_text.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty()
                            && !trimmed.starts_with("Usage:")
                            && !trimmed.starts_with("usage:")
                            && !trimmed.starts_with(&package.to_uppercase())
                            && !trimmed.starts_with(&package)
                            && trimmed.len() > 10
                        {
                            return Some(trimmed.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

async fn check_language_packages(package: &str, output: &Output, found_info: &mut bool) {
    // Check PyPI
    if let Ok(pip_output) = Command::new("pip")
        .args(["show", package])
        .output()
    {
        if pip_output.status.success() {
            let output_str = String::from_utf8_lossy(&pip_output.stdout);
            if let Some(summary_line) = output_str.lines().find(|line| line.starts_with("Summary:")) {
                let summary = summary_line.strip_prefix("Summary:").unwrap_or("").trim();
                if !summary.is_empty() {
                    *found_info = true;
                    output.success("✅ Found Python package:");
                    output.info(&format!("🐍 {}", summary));
                }
            }
        }
    }

    // Check npm
    if let Ok(npm_output) = Command::new("npm")
        .args(["info", package, "description", "--silent"])
        .output()
    {
        if npm_output.status.success() {
            let description = String::from_utf8_lossy(&npm_output.stdout).trim().to_string();
            if !description.is_empty() && description != "undefined" {
                *found_info = true;
                output.success("✅ Found npm package:");
                output.info(&format!("📦 {}", description));
            }
        }
    }

    // Check Ruby gems
    if let Ok(gem_output) = Command::new("gem")
        .args(["specification", package, "--remote"])
        .output()
    {
        if gem_output.status.success() {
            let output_str = String::from_utf8_lossy(&gem_output.stdout);
            if let Some(summary_line) = output_str.lines().find(|line| line.trim().starts_with("summary:")) {
                let summary = summary_line.split(':').nth(1).unwrap_or("").trim();
                if !summary.is_empty() {
                    *found_info = true;
                    output.success("✅ Found Ruby gem:");
                    output.info(&format!("💎 {}", summary));
                }
            }
        }
    }

    // Check Cargo crates
    if let Ok(cargo_output) = Command::new("cargo")
        .args(["search", package, "--limit", "1"])
        .output()
    {
        if cargo_output.status.success() {
            let output_str = String::from_utf8_lossy(&cargo_output.stdout);
            if let Some(first_line) = output_str.lines().next() {
                if first_line.contains(&package) {
                    *found_info = true;
                    output.success("✅ Found Rust crate:");
                    output.info(&format!("🦀 {}", first_line.trim()));
                }
            }
        }
    }
}