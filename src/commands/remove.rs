use anyhow::{Result, Context};
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::core::normalizer::PackageNormalizer;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

pub async fn execute(packages: Vec<String>, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    if packages.is_empty() {
        output.error("No packages specified");
        return Ok(());
    }

    output.print_header("🗑️  Removing Packages");

    // Detect platform and get package manager
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.debug(&format!("Using package manager: {}", package_manager.name()));

    // Get the package manager type for normalization
    let pm_type = platform_info.primary_package_manager()
        .context("No package manager available")?;

    // Initialize normalizer
    let normalizer = PackageNormalizer::new();

    // Track successful and failed removals
    let mut removed = Vec::new();
    let mut failed = Vec::new();

    for package in &packages {
        output.remove_start(package);

        // Normalize package name
        let normalized_names = normalizer.normalize(package, pm_type)?;
        
        let packages_to_use = if normalized_names.is_empty() {
            vec![package.to_string()]
        } else {
            normalized_names
        };

        // Check if installed
        let is_installed_map = package_manager.is_installed(&packages_to_use).await?;
        let any_installed = packages_to_use.iter().any(|p| is_installed_map.get(p) == Some(&true));
        
        if !any_installed {
            output.warn(&format!("⚠️  {} is not installed", package));
            continue;
        }

        // Attempt removal
        match package_manager.remove(&packages_to_use).await {
            Ok(result) => {
                if result.success {
                    output.success(&format!("✅ Removed {}", package));
                    removed.push(package.clone());
                } else {
                    output.error(&format!("❌ Failed to remove {}: {}", package, result.message));
                    failed.push(package.clone());
                }
            }
            Err(e) => {
                output.error(&format!("❌ Error removing {}: {}", package, e));
                failed.push(package.clone());
            }
        }
    }

    // Summary
    output.print_header("📊 Removal Summary");
    
    if !removed.is_empty() {
        output.success(&format!("✅ Removed {} packages: {}", removed.len(), removed.join(", ")));
    }
    
    if !failed.is_empty() {
        output.error(&format!("❌ Failed to remove {} packages: {}", failed.len(), failed.join(", ")));
        return Err(anyhow::anyhow!("Some packages failed to remove"));
    }

    Ok(())
}