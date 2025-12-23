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

    output.print_header("📦 Installing Packages");

    // Detect platform and get package manager
    let platform_info = PlatformInfo::detect_async().await?;
    output.debug(&format!("Detected platform: {} - {:?}", platform_info.os(), platform_info.distribution));

    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.debug(&format!("Using package manager: {}", package_manager.name()));

    // Get the package manager type for normalization
    let pm_type = platform_info.primary_package_manager()
        .context("No package manager available")?;

    // Initialize normalizer for package name mapping
    let normalizer = PackageNormalizer::new();

    // Track successful and failed installations
    let mut installed = Vec::new();
    let mut failed = Vec::new();

    for package in &packages {
        output.install_start(package);

        // Normalize package name for this platform
        let normalized_names = normalizer.normalize(package, pm_type)?;
        
        if normalized_names.len() > 1 || (normalized_names.len() == 1 && &normalized_names[0] != package) {
            output.debug(&format!("Normalized '{}' to {:?}", package, normalized_names));
        }

        // Use the normalized packages (could be multiple)
        let packages_to_use = if normalized_names.is_empty() {
            vec![package.to_string()]
        } else {
            normalized_names
        };

        // Check if already installed
        let is_installed_map = package_manager.is_installed(&packages_to_use).await?;
        let all_installed = packages_to_use.iter().all(|p| is_installed_map.get(p) == Some(&true));
        
        if all_installed {
            output.info(&format!("📦 {} is already installed", package));
            installed.push(package.clone());
            continue;
        }

        // Attempt installation
        match package_manager.install(&packages_to_use).await {
            Ok(result) => {
                if result.success {
                    output.success(&format!("✅ Installed {}", package));
                    installed.push(package.clone());
                } else {
                    output.error(&format!("❌ Failed to install {}: {}", package, result.message));
                    failed.push(package.clone());
                }
            }
            Err(e) => {
                output.error(&format!("❌ Error installing {}: {}", package, e));
                failed.push(package.clone());
            }
        }
    }

    // Summary
    output.print_header("📊 Installation Summary");
    
    if !installed.is_empty() {
        output.success(&format!("✅ Installed {} packages: {}", installed.len(), installed.join(", ")));
    }
    
    if !failed.is_empty() {
        output.error(&format!("❌ Failed to install {} packages: {}", failed.len(), failed.join(", ")));
        return Err(anyhow::anyhow!("Some packages failed to install"));
    }

    Ok(())
}