use anyhow::Result;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;
use crate::doctor::checker::HealthChecker;
use crate::doctor::diagnostics::Diagnostics;
use crate::doctor::report::{ReportFormatter, ExportFormat};

pub async fn execute(
    full: bool,
    packages: bool,
    usb: bool,
    security: bool,
    fix: bool,
    cli: &Cli,
    _config: &Config,
    output: &Output,
) -> Result<()> {
    // Create health checker
    let checker = HealthChecker::new(output.clone(), fix)?;

    // Run appropriate checks
    let report = if full {
        output.section("🏥 Running Full System Health Check");
        checker.check_all().await?
    } else if packages {
        output.section("📦 Checking Package Management Health");
        checker.check_packages_only().await?
    } else if usb {
        output.section("💾 Checking USB Device Health");
        checker.check_usb_only().await?
    } else if security {
        output.section("🔐 Checking Security Status");
        checker.check_security_only().await?
    } else {
        // Default: run standard checks
        output.section("🏥 Running System Health Check");
        checker.check_all().await?
    };

    // Display report
    let formatter = ReportFormatter::new(output.clone());
    formatter.display(&report);

    // Run diagnostics if requested
    if full && output.verbose {
        let diagnostics = Diagnostics::new(output.clone(), fix, cli.dry_run);
        diagnostics.run_diagnostics(&report).await?;
    }

    // Apply fixes if requested
    if fix {
        let diagnostics = Diagnostics::new(output.clone(), fix, cli.dry_run);
        diagnostics.apply_fixes(&report).await?;
    }

    // Export report if requested (could add --export flag)
    if std::env::var("PKMGR_EXPORT_REPORT").is_ok() {
        formatter.export(&report, ExportFormat::Markdown, None)?;
    }

    Ok(())
}
