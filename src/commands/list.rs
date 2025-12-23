use anyhow::{Result, Context};
use clap::ValueEnum;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

#[derive(Debug, Clone, ValueEnum)]
pub enum ListType {
    Installed,
    Available,
}

pub async fn execute(list_type: Option<ListType>, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    let list_type = list_type.unwrap_or(ListType::Installed);

    // Detect platform and get package manager
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    match list_type {
        ListType::Installed => {
            output.print_header("📦 Installed Packages");
            output.info(&format!("🔍 Listing packages from {}...", package_manager.name()));

            match package_manager.list_installed().await {
                Ok(packages) => {
                    if packages.is_empty() {
                        output.warn("⚠️  No packages found");
                    } else {
                        output.success(&format!("✅ Found {} installed packages:", packages.len()));

                        // Display packages in a formatted list
                        for (i, pkg) in packages.iter().enumerate() {
                            // Limit to first 50 packages unless verbose
                            if !cli.verbose && i >= 50 {
                                output.info(&format!("... and {} more packages", packages.len() - 50));
                                output.info("💡 Use --verbose to see all packages");
                                break;
                            }

                            let desc = pkg.description.as_deref().unwrap_or("");
                            if desc.is_empty() {
                                output.info(&format!("  📦 {} ({})", pkg.name, pkg.version));
                            } else {
                                // Truncate long descriptions
                                let desc_short = if desc.len() > 60 {
                                    format!("{}...", &desc[..57])
                                } else {
                                    desc.to_string()
                                };
                                output.info(&format!("  📦 {} ({}) - {}", pkg.name, pkg.version, desc_short));
                            }
                        }

                        output.info("");
                        output.info(&format!("📊 Total: {} packages", packages.len()));
                    }
                }
                Err(e) => {
                    output.error(&format!("❌ Failed to list packages: {}", e));
                    return Err(e);
                }
            }
        }
        ListType::Available => {
            output.print_header("📋 Available Packages");
            output.info(&format!("Package manager: {}", package_manager.name()));
            output.info("");
            output.info("⚠️  Listing all available packages can be very large");
            output.info("💡 Use 'pkmgr search <query>' to find specific packages");
            output.info("💡 Examples:");
            output.info("     pkmgr search python");
            output.info("     pkmgr search web");
            output.info("     pkmgr search editor");
        }
    }

    Ok(())
}