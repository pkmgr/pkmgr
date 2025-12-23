use anyhow::{Result, Context};
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

pub async fn execute(query: String, _cli: &Cli, _config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("🔍 Searching for: {}", query));

    // Get platform-appropriate package manager
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.info(&format!("🔍 Searching in {} repositories...", package_manager.name()));

    // Perform search
    match package_manager.search(&query).await {
        Ok(search_result) => {
            if search_result.packages.is_empty() {
                output.warn(&format!("⚠️  No packages found matching '{}'", query));
                output.info("💡 Try using a different search term or check the package name");
            } else {
                output.success(&format!("✅ Found {} packages:", search_result.total_count));

                for (i, package) in search_result.packages.iter().enumerate() {
                    if i >= 10 { // Limit display to first 10 results
                        output.info(&format!("... and {} more packages", search_result.total_count - 10));
                        break;
                    }

                    let desc = package.description.as_deref().unwrap_or("No description available");
                    let status = if package.installed { " [installed]" } else { "" };

                    output.info(&format!("  📦 {} ({}){} - {}",
                        package.name,
                        package.version,
                        status,
                        desc
                    ));
                }

                if search_result.total_count > 10 {
                    output.info(&format!("💡 Use 'pkmgr info <package>' for detailed information"));
                }
            }
        }
        Err(e) => {
            output.error(&format!("❌ Search failed: {}", e));
            output.info("💡 Check your network connection and try again");
        }
    }

    Ok(())
}