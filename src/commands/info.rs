use anyhow::{Result, Context};
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

pub async fn execute(package: String, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("📌 Package Information: {}", package));

    // Detect platform and get package manager
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.info(&format!("🔍 Searching for package info using {}", package_manager.name()));

    // Try to get package info
    match package_manager.info(&package).await {
        Ok(Some(info)) => {
            output.success(&format!("✅ Found package: {}", info.name));

            // Display package details
            output.info(&format!("📦 Name: {}", info.name));
            output.info(&format!("🏷️  Version: {}", info.version));

            if let Some(description) = &info.description {
                output.info(&format!("📚 Description: {}", description));
            }

            if let Some(size) = info.size {
                let size_mb = size as f64 / 1024.0 / 1024.0;
                output.info(&format!("💾 Size: {:.2} MB", size_mb));
            }

            output.info(&format!("📂 Source: {}", info.source));
            output.info(&format!("📥 Installed: {}", if info.installed { "✅ Yes" } else { "❌ No" }));
        }
        Ok(None) => {
            output.warn(&format!("⚠️  Package '{}' not found in {}", package, package_manager.name()));

            // Try to search for similar packages
            output.info("🔍 Searching for similar packages...");
            match package_manager.search(&package).await {
                Ok(search_result) => {
                    if !search_result.packages.is_empty() {
                        output.info("📋 Similar packages found:");
                        for pkg in search_result.packages.iter().take(5) {
                            let desc = pkg.description.as_deref().unwrap_or("No description");
                            output.info(&format!("  • {} - {}", pkg.name, desc));
                        }
                    } else {
                        output.info("❌ No similar packages found");
                    }
                }
                Err(e) => {
                    output.warn(&format!("⚠️  Search failed: {}", e));
                }
            }
        }
        Err(e) => {
            output.error(&format!("❌ Failed to get package info: {}", e));
        }
    }

    Ok(())
}