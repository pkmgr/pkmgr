use anyhow::{Result, Context};
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

pub async fn execute(packages: Option<Vec<String>>, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    // Detect platform and get package manager
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.debug(&format!("Using package manager: {}", package_manager.name()));

    match packages {
        Some(packages) if packages.len() == 1 && packages[0] == "all" => {
            output.print_header("🔄 Updating All Packages");
            output.update_start("all packages");

            // First update package lists/metadata
            output.info("📥 Updating package lists...");
            package_manager.update().await?;

            // Then upgrade all packages
            output.info("⬆️  Upgrading packages...");
            match package_manager.upgrade(None).await {
                Ok(result) => {
                    if result.success {
                        output.success(&format!("✅ {}", result.message));
                    } else {
                        output.error(&format!("❌ {}", result.message));
                        return Err(anyhow::anyhow!("Update failed"));
                    }
                }
                Err(e) => {
                    output.error(&format!("❌ Update failed: {}", e));
                    return Err(e);
                }
            }
        }
        Some(packages) => {
            output.print_header("🔄 Updating Specific Packages");

            // Update package lists first
            output.info("📥 Updating package lists...");
            package_manager.update().await?;

            for package in &packages {
                output.update_start(package);

                match package_manager.upgrade(Some(&[package.clone()])).await {
                    Ok(result) => {
                        if result.success {
                            output.success(&format!("✅ Updated {}", package));
                        } else {
                            output.error(&format!("❌ Failed to update {}: {}", package, result.message));
                        }
                    }
                    Err(e) => {
                        output.error(&format!("❌ Error updating {}: {}", package, e));
                    }
                }
            }
        }
        None => {
            output.print_header("🔄 Updating All Packages");
            output.update_start("all packages");

            // Update package lists
            output.info("📥 Updating package lists...");
            package_manager.update().await?;

            // Upgrade all packages
            output.info("⬆️  Upgrading packages...");
            match package_manager.upgrade(None).await {
                Ok(result) => {
                    if result.success {
                        output.success(&format!("✅ {}", result.message));
                    } else {
                        output.error(&format!("❌ {}", result.message));
                        return Err(anyhow::anyhow!("Update failed"));
                    }
                }
                Err(e) => {
                    output.error(&format!("❌ Update failed: {}", e));
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}