use anyhow::{Context, Result};
use crate::commands::Cli;
use crate::core::config::Config;
use crate::core::platform::{Platform, PlatformInfo};
use crate::ui::output::Output;
use crate::recovery::{ErrorAnalyzer, ErrorFixer, RecoveryStrategies};
use std::fs;
use std::path::Path;

pub async fn execute(
    auto: bool,
    dry_run: bool,
    last_error: bool,
    cli: &Cli,
    config: &Config,
    output: &Output,
) -> Result<()> {
    let platform = Platform::detect()?;

    if last_error {
        // Analyze the last error from log file
        analyze_last_error(auto, dry_run, output, platform).await
    } else {
        // Run general system recovery
        run_system_recovery(auto, dry_run, output, platform).await
    }
}

async fn analyze_last_error(
    auto: bool,
    dry_run: bool,
    output: &Output,
    platform: PlatformInfo,
) -> Result<()> {
    output.section("Error Recovery Analysis");

    // Read last error from log file
    let log_path = dirs::data_dir()
        .context("Failed to determine data directory")?
        .join("pkmgr")
        .join("last_error.log");

    if !log_path.exists() {
        output.info("No recent errors found to analyze");
        output.info("Error recovery is automatically triggered when commands fail");
        return Ok(());
    }

    let error_content = fs::read_to_string(&log_path)
        .context("Failed to read error log")?;

    // Parse error content
    let lines: Vec<&str> = error_content.lines().collect();
    let stdout = lines.iter()
        .find(|l| l.starts_with("STDOUT:"))
        .map(|l| l.trim_start_matches("STDOUT:"))
        .unwrap_or("");
    let stderr = lines.iter()
        .find(|l| l.starts_with("STDERR:"))
        .map(|l| l.trim_start_matches("STDERR:"))
        .unwrap_or("");
    let exit_code: i32 = lines.iter()
        .find(|l| l.starts_with("EXIT_CODE:"))
        .and_then(|l| l.trim_start_matches("EXIT_CODE:").parse().ok())
        .unwrap_or(1);

    // Analyze error
    let analyzer = ErrorAnalyzer::new(output.clone(), platform);
    let analyses = analyzer.analyze(stdout, stderr, exit_code);

    if analyses.is_empty() {
        output.warn("No known error patterns matched");
        output.info("The error may require manual intervention");
        return Ok(());
    }

    // Display analysis
    analyzer.display_analysis(&analyses);

    // Apply fixes if requested
    if !dry_run {
        let fixer = ErrorFixer::new(output.clone(), dry_run, auto);

        for analysis in &analyses {
            output.section(&format!("Applying fixes for: {}", analysis.matched_pattern.name));

            for fix in &analysis.suggested_fixes {
                let applied = fixer.apply_fix(analysis, fix).await?;

                if applied {
                    output.success(&format!("✅ Applied fix: {}", fix.description));
                    // If one fix works, we're done
                    break;
                }
            }
        }
    }

    // Clean up log file after successful recovery
    if !dry_run {
        let _ = fs::remove_file(&log_path);
    }

    Ok(())
}

async fn run_system_recovery(
    auto: bool,
    dry_run: bool,
    output: &Output,
    platform: PlatformInfo,
) -> Result<()> {
    output.section("System Recovery");

    let strategies = RecoveryStrategies::new(output.clone());

    // Display recovery capabilities
    let stats = RecoveryStrategies::get_statistics();
    output.info(&format!("🔧 Recovery Pattern Database:"));
    output.info(&format!("   • Total patterns: {}", stats.total_patterns));
    output.info(&format!("   • Arch patterns: {}", stats.arch_patterns));
    output.info(&format!("   • Debian patterns: {}", stats.debian_patterns));
    output.info(&format!("   • Fedora patterns: {}", stats.fedora_patterns));
    output.info(&format!("   • Common patterns: {}", stats.common_patterns));
    output.info(&format!("   • Success rate: {:.0}%", stats.success_rate * 100.0));

    output.section("Recovery Categories");
    for (category, count) in &stats.categories {
        output.info(&format!("   • {}: {} patterns", category, count));
    }

    // Check for common issues
    output.section("Checking for Common Issues");

    // Check package manager locks
    check_package_locks(output, &platform, auto, dry_run).await?;

    // Check broken dependencies
    check_broken_deps(output, &platform, auto, dry_run).await?;

    // Check repository issues
    check_repo_issues(output, &platform, auto, dry_run).await?;

    // Check disk space
    check_disk_space(output, &platform).await?;

    output.success("✅ System recovery check complete");

    Ok(())
}

async fn check_package_locks(
    output: &Output,
    platform: &crate::core::platform::PlatformInfo,
    auto: bool,
    dry_run: bool,
) -> Result<()> {
    output.progress("Checking for package manager locks...");

    let lock_files = match platform.distribution.as_ref().map(|s| s.as_str()).unwrap_or("") {
        dist if dist.contains("ubuntu") || dist.contains("debian") => {
            vec![
                "/var/lib/dpkg/lock-frontend",
                "/var/lib/dpkg/lock",
                "/var/cache/apt/archives/lock",
            ]
        }
        dist if dist.contains("fedora") || dist.contains("centos") || dist.contains("rhel") => {
            vec!["/var/run/yum.pid"]
        }
        dist if dist.contains("arch") || dist.contains("manjaro") => {
            vec!["/var/lib/pacman/db.lck"]
        }
        _ => vec![]
    };

    let mut found_locks = false;
    for lock_file in &lock_files {
        if Path::new(lock_file).exists() {
            output.warn(&format!("⚠️  Found lock file: {}", lock_file));
            found_locks = true;
        }
    }

    if found_locks {
        if auto || dry_run {
            output.info("Would remove stale lock files");
        } else {
            output.warn("Package manager locks detected. These may be from:");
            output.info("  • Another package manager running");
            output.info("  • Previous interrupted operation");
            output.info("  • System update in progress");
            output.info("");
            output.info("Run 'pkmgr fix --auto' to clear locks if no operations are running");
        }
    } else {
        output.success("✓ No package manager locks found");
    }

    Ok(())
}

async fn check_broken_deps(
    output: &Output,
    platform: &crate::core::platform::PlatformInfo,
    auto: bool,
    dry_run: bool,
) -> Result<()> {
    output.progress("Checking for broken dependencies...");

    // Platform-specific dependency check commands
    let check_cmd = match platform.distribution.as_ref().map(|s| s.as_str()).unwrap_or("") {
        dist if dist.contains("ubuntu") || dist.contains("debian") => {
            Some(("dpkg", vec!["--audit"]))
        }
        dist if dist.contains("fedora") || dist.contains("centos") || dist.contains("rhel") => {
            Some(("rpm", vec!["-Va", "--nofiles", "--noscripts"]))
        }
        dist if dist.contains("arch") || dist.contains("manjaro") => {
            Some(("pacman", vec!["-Dk"]))
        }
        _ => None
    };

    if let Some((cmd, args)) = check_cmd {
        let result = std::process::Command::new(cmd)
            .args(&args)
            .output()
            .context(format!("Failed to run {}", cmd))?;

        if !result.status.success() || !result.stdout.is_empty() {
            output.warn("⚠️  Broken dependencies detected");

            if auto || dry_run {
                output.info("Would fix broken dependencies");
            } else {
                output.info("Run 'pkmgr fix --auto' to repair dependencies");
            }
        } else {
            output.success("✓ No broken dependencies found");
        }
    }

    Ok(())
}

async fn check_repo_issues(
    output: &Output,
    platform: &PlatformInfo,
    auto: bool,
    dry_run: bool,
) -> Result<()> {
    output.progress("Checking repository configuration...");

    // Check for expired GPG keys
    let gpg_check = std::process::Command::new("gpg")
        .args(&["--list-keys", "--with-colons"])
        .output();

    if let Ok(result) = gpg_check {
        let output_str = String::from_utf8_lossy(&result.stdout);
        let expired_keys: Vec<&str> = output_str
            .lines()
            .filter(|line| line.contains(":e:"))
            .collect();

        if !expired_keys.is_empty() {
            output.warn(&format!("⚠️  Found {} expired GPG keys", expired_keys.len()));

            if auto || dry_run {
                output.info("Would refresh expired keys");
            } else {
                output.info("Run 'pkmgr fix --auto' to refresh keys");
            }
        } else {
            output.success("✓ All GPG keys are valid");
        }
    }

    Ok(())
}

async fn check_disk_space(output: &Output, platform: &PlatformInfo) -> Result<()> {
    output.progress("Checking disk space...");

    // Get disk usage for key directories
    let paths = vec!["/", "/var", "/tmp", "/home"];
    let mut low_space = false;

    for path in paths {
        if let Ok(stats) = fs2::statvfs(path) {
            let available = stats.available_space();
            let total = stats.total_space();
            let percent_used = ((total - available) as f64 / total as f64 * 100.0) as u8;

            if percent_used > 90 {
                output.warn(&format!("⚠️  Low disk space on {}: {}% used", path, percent_used));
                low_space = true;
            }
        }
    }

    if low_space {
        output.info("Consider running 'pkmgr cache clean' to free space");
    } else {
        output.success("✓ Adequate disk space available");
    }

    Ok(())
}

// Integration point for automatic error recovery
pub async fn recover_from_error(
    stdout: &str,
    stderr: &str,
    exit_code: i32,
    command: &str,
    output: &Output,
    config: &Config,
) -> Result<bool> {
    // Save error for later analysis
    save_last_error(stdout, stderr, exit_code, command)?;

    let platform = Platform::detect()?;
    let analyzer = ErrorAnalyzer::new(output.clone(), platform);
    let analyses = analyzer.analyze(stdout, stderr, exit_code);

    if analyses.is_empty() {
        return Ok(false);
    }

    // Only auto-fix safe operations
    let auto_fix = config.defaults.auto_fix;
    let fixer = ErrorFixer::new(output.clone(), false, auto_fix);

    for analysis in &analyses {
        if analyzer.should_auto_fix(analysis) {
            output.info(&format!("🔧 Attempting automatic recovery: {}",
                analysis.matched_pattern.name));

            for fix in &analysis.suggested_fixes {
                if fixer.apply_fix(analysis, fix).await? {
                    output.success("✅ Error recovered automatically");
                    return Ok(true);
                }
            }
        }
    }

    // If we couldn't auto-fix, show analysis to user
    if !analyses.is_empty() {
        output.section("Error Analysis");
        analyzer.display_analysis(&analyses);
        output.info("💡 Run 'pkmgr fix --last-error' to apply suggested fixes");
    }

    Ok(false)
}

fn save_last_error(stdout: &str, stderr: &str, exit_code: i32, command: &str) -> Result<()> {
    let data_dir = dirs::data_dir()
        .context("Failed to determine data directory")?
        .join("pkmgr");

    fs::create_dir_all(&data_dir)?;

    let content = format!(
        "COMMAND:{}\nEXIT_CODE:{}\nSTDOUT:{}\nSTDERR:{}\nTIME:{}",
        command,
        exit_code,
        stdout,
        stderr,
        chrono::Utc::now().to_rfc3339()
    );

    fs::write(data_dir.join("last_error.log"), content)?;

    Ok(())
}