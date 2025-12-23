use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::PlatformInfo;
use crate::managers::PackageManagerFactory;
use crate::ui::output::Output;

pub async fn execute(package: String, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    output.print_header(&format!("📁 Package Location: {}", package));

    // Check if it's installed as a binary first
    output.info("🔍 Searching in PATH...");
    let path_locations = find_in_path(&package);

    if !path_locations.is_empty() {
        output.success("✅ Found in PATH:");
        for location in &path_locations {
            output.info(&format!("  📂 {}", location));
        }
    } else {
        output.info("❌ Not found in PATH");
    }

    // Check package manager information
    let platform_info = PlatformInfo::detect_async().await?;
    let package_manager = PackageManagerFactory::create(&platform_info)
        .context("Failed to create package manager")?;

    output.info(&format!("🔍 Checking {} for package info...", package_manager.name()));

    // Check if package is installed via package manager
    let installed_check = package_manager.is_installed(&[package.clone()]).await?;

    if let Some(is_installed) = installed_check.get(&package) {
        if *is_installed {
            output.success(&format!("✅ Package '{}' is installed via {}", package, package_manager.name()));

            // Try to get package info for additional details
            if let Ok(Some(info)) = package_manager.info(&package).await {
                output.info(&format!("📦 Package: {}", info.name));
                output.info(&format!("🏷️  Version: {}", info.version));
                output.info(&format!("📂 Managed by: {}", info.source));
            }
        } else {
            output.info(&format!("❌ Package '{}' not installed via {}", package, package_manager.name()));
        }
    }

    // Check common installation directories
    output.info("🔍 Checking common installation directories...");
    let common_paths = get_common_paths();
    let mut found_in_common = false;

    for path in common_paths {
        let package_path = format!("{}/{}", path, package);
        if Path::new(&package_path).exists() {
            output.info(&format!("  📂 {}", package_path));
            found_in_common = true;
        }
    }

    if !found_in_common {
        output.info("❌ Not found in common directories");
    }

    // Check language-specific locations
    output.info("🔍 Checking language-specific locations...");
    check_language_locations(&package, output);

    // Summary
    if path_locations.is_empty() && !found_in_common {
        output.warn(&format!("⚠️  Package '{}' not found in standard locations", package));
        output.info(&format!("💡 Try: pkmgr search {} to find similar packages", package));
    }

    Ok(())
}

fn find_in_path(package: &str) -> Vec<String> {
    let mut locations = Vec::new();

    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(':') {
            let package_path = format!("{}/{}", dir, package);
            if Path::new(&package_path).exists() {
                locations.push(package_path);
            }
        }
    }

    // Also check with `which` or `where` command
    #[cfg(unix)]
    {
        if let Ok(output) = Command::new("which").arg(package).output() {
            if output.status.success() {
                if let Ok(which_result) = String::from_utf8(output.stdout) {
                    let trimmed = which_result.trim();
                    if !trimmed.is_empty() && !locations.contains(&trimmed.to_string()) {
                        locations.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    #[cfg(windows)]
    {
        if let Ok(output) = Command::new("where").arg(package).output() {
            if output.status.success() {
                if let Ok(where_result) = String::from_utf8(output.stdout) {
                    for line in where_result.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() && !locations.contains(&trimmed.to_string()) {
                            locations.push(trimmed.to_string());
                        }
                    }
                }
            }
        }
    }

    locations
}

fn get_common_paths() -> Vec<&'static str> {
    vec![
        "/usr/bin",
        "/usr/local/bin",
        "/opt",
        "/usr/share",
        "/usr/local/share",
        "/Applications", // macOS
        "C:\\Program Files", // Windows
        "C:\\Program Files (x86)", // Windows
    ]
}

fn check_language_locations(package: &str, output: &Output) {
    // Check Python packages
    if let Ok(python_output) = Command::new("python3")
        .args(["-c", &format!("import {}; print({}.__file__)", package, package)])
        .output()
    {
        if python_output.status.success() {
            if let Ok(location) = String::from_utf8(python_output.stdout) {
                output.info(&format!("🐍 Python package: {}", location.trim()));
            }
        }
    }

    // Check npm packages
    if let Ok(npm_output) = Command::new("npm")
        .args(["list", "-g", package])
        .output()
    {
        if npm_output.status.success() {
            let output_str = String::from_utf8_lossy(&npm_output.stdout);
            if !output_str.contains("(empty)") && output_str.contains(package) {
                output.info(&format!("📦 npm global package: {}", package));
            }
        }
    }

    // Check Ruby gems
    if let Ok(gem_output) = Command::new("gem")
        .args(["which", package])
        .output()
    {
        if gem_output.status.success() {
            if let Ok(location) = String::from_utf8(gem_output.stdout) {
                let trimmed = location.trim();
                if !trimmed.is_empty() {
                    output.info(&format!("💎 Ruby gem: {}", trimmed));
                }
            }
        }
    }
}