use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::doctor::{CheckCategory, Finding, HealthReport, Severity};
use crate::ui::output::Output;
use crate::cache::format_size;

pub struct ReportFormatter {
    output: Output,
}

impl ReportFormatter {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Display health report
    pub fn display(&self, report: &HealthReport) {
        // Header
        self.display_header(report);

        // Summary
        self.display_summary(report);

        // System Info
        self.display_system_info(report);

        // Findings by category
        self.display_findings(report);

        // Recommendations
        self.display_recommendations(report);

        // Footer
        self.display_footer(report);
    }

    /// Display report header
    fn display_header(&self, report: &HealthReport) {
        let status = report.overall_status();
        let emoji = status.emoji();
        let title = match status {
            Severity::Ok => "System Health: Excellent",
            Severity::Info => "System Health: Good",
            Severity::Warning => "System Health: Fair",
            Severity::Error => "System Health: Poor",
            Severity::Critical => "System Health: Critical",
        };

        self.output.section(&format!("{} {}", emoji, title));
    }

    /// Display summary statistics
    fn display_summary(&self, report: &HealthReport) {
        self.output.info(&format!("📊 Health Check Summary:"));
        self.output.info(&format!("   • Total checks: {}", report.stats.total()));

        if report.stats.ok_count > 0 {
            self.output.info(&format!("   {} {} passed",
                Severity::Ok.emoji(),
                report.stats.ok_count
            ));
        }

        if report.stats.info_count > 0 {
            self.output.info(&format!("   {} {} informational",
                Severity::Info.emoji(),
                report.stats.info_count
            ));
        }

        if report.stats.warning_count > 0 {
            self.output.warn(&format!("   {} {} warnings",
                Severity::Warning.emoji(),
                report.stats.warning_count
            ));
        }

        if report.stats.error_count > 0 {
            self.output.error(&format!("   {} {} errors",
                Severity::Error.emoji(),
                report.stats.error_count
            ));
        }

        if report.stats.critical_count > 0 {
            self.output.error(&format!("   {} {} critical",
                Severity::Critical.emoji(),
                report.stats.critical_count
            ));
        }

        if report.stats.fixable_count > 0 {
            self.output.info(&format!("   🔧 {} auto-fixable",
                report.stats.fixable_count
            ));
        }
    }

    /// Display system information
    fn display_system_info(&self, report: &HealthReport) {
        self.output.section("System Information");

        let info = &report.system_info;

        self.output.info(&format!("🖥️  System: {} {} ({})",
            info.distribution,
            info.version,
            info.architecture
        ));

        self.output.info(&format!("🐧 Kernel: {}",
            info.kernel
        ));

        self.output.info(&format!("🏠 Hostname: {}",
            info.hostname
        ));

        self.output.info(&format!("⏰ Uptime: {}",
            info.uptime
        ));

        self.output.info(&format!("🧠 CPU: {} cores",
            info.cpu_count
        ));

        let mem_used_gb = (info.memory_total - info.memory_available) / (1024 * 1024 * 1024);
        let mem_total_gb = info.memory_total / (1024 * 1024 * 1024);
        self.output.info(&format!("💾 Memory: {} GB / {} GB",
            mem_used_gb,
            mem_total_gb
        ));

        let disk_used_gb = (info.disk_total - info.disk_available) / (1024 * 1024 * 1024);
        let disk_total_gb = info.disk_total / (1024 * 1024 * 1024);
        self.output.info(&format!("💿 Disk: {} GB / {} GB",
            disk_used_gb,
            disk_total_gb
        ));

        self.output.info(&format!("📦 pkmgr: v{}",
            info.pkmgr_version
        ));
    }

    /// Display findings grouped by category
    fn display_findings(&self, report: &HealthReport) {
        // Group findings by category
        let mut by_category: std::collections::HashMap<String, Vec<&Finding>> = std::collections::HashMap::new();

        for finding in &report.findings {
            by_category.entry(finding.category.clone())
                .or_default()
                .push(finding);
        }

        // Sort categories
        let mut categories: Vec<_> = by_category.keys().cloned().collect();
        categories.sort();

        for category in categories {
            let findings = &by_category[&category];

            // Skip if all findings are OK
            if findings.iter().all(|f| f.severity == Severity::Ok) && !self.output.verbose {
                continue;
            }

            // Get category emoji
            let emoji = match category.as_str() {
                "System" => CheckCategory::System.emoji(),
                "Packages" => CheckCategory::Packages.emoji(),
                "Languages" => CheckCategory::Languages.emoji(),
                "Network" => CheckCategory::Network.emoji(),
                "Storage" => CheckCategory::Storage.emoji(),
                "Security" => CheckCategory::Security.emoji(),
                "Configuration" => CheckCategory::Configuration.emoji(),
                "Cache" => CheckCategory::Cache.emoji(),
                "Repository" => CheckCategory::Repository.emoji(),
                "USB" => CheckCategory::USB.emoji(),
                "Binary" => CheckCategory::Binary.emoji(),
                "Shell" => CheckCategory::Shell.emoji(),
                _ => "📋",
            };

            self.output.section(&format!("{} {}", emoji, category));

            for finding in findings {
                self.display_finding(finding);
            }
        }
    }

    /// Display a single finding
    fn display_finding(&self, finding: &Finding) {
        let emoji = finding.severity.emoji();
        let message = format!("{} {}", emoji, finding.message);

        match finding.severity {
            Severity::Ok => {
                if self.output.verbose {
                    self.output.success(&message);
                }
            }
            Severity::Info => self.output.info(&message),
            Severity::Warning => self.output.warn(&message),
            Severity::Error => self.output.error(&message),
            Severity::Critical => self.output.error(&message),
        }

        if let Some(details) = &finding.details {
            self.output.info(&format!("     {}", details));
        }

        if let Some(fix) = &finding.fix_hint {
            let fix_emoji = if finding.auto_fixable { "🔧" } else { "💡" };
            self.output.info(&format!("     {} {}", fix_emoji, fix));
        }
    }

    /// Display recommendations
    fn display_recommendations(&self, report: &HealthReport) {
        if report.recommendations.is_empty() {
            return;
        }

        self.output.section("💡 Recommendations");

        for recommendation in &report.recommendations {
            self.output.info(&format!("• {}", recommendation));
        }
    }

    /// Display footer
    fn display_footer(&self, report: &HealthReport) {
        self.output.section("Next Steps");

        if report.stats.fixable_count > 0 {
            self.output.info("🔧 Run 'pkmgr doctor --fix' to apply automatic fixes");
        }

        if report.stats.has_issues() {
            self.output.info("📖 Review findings above and apply suggested fixes");
        } else {
            self.output.success("✅ Your system is healthy!");
        }

        self.output.info(&format!("🕐 Check completed at {}",
            report.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        ));
    }

    /// Export report to file
    pub fn export(&self, report: &HealthReport, format: ExportFormat, path: Option<PathBuf>) -> Result<()> {
        let content = match format {
            ExportFormat::Text => self.export_text(report),
            ExportFormat::Markdown => self.export_markdown(report),
            ExportFormat::Json => self.export_json(report)?,
            ExportFormat::Html => self.export_html(report),
        };

        let filename = path.unwrap_or_else(|| {
            let timestamp = report.timestamp.format("%Y%m%d_%H%M%S");
            let extension = match format {
                ExportFormat::Text => "txt",
                ExportFormat::Markdown => "md",
                ExportFormat::Json => "json",
                ExportFormat::Html => "html",
            };
            PathBuf::from(format!("pkmgr_health_report_{}.{}", timestamp, extension))
        });

        fs::write(&filename, content)?;
        self.output.success(&format!("📄 Report saved to: {}", filename.display()));

        Ok(())
    }

    /// Export as plain text
    fn export_text(&self, report: &HealthReport) -> String {
        let mut content = String::new();

        content.push_str("pkmgr System Health Report\n");
        content.push_str(&format!("Generated: {}\n\n", report.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));

        content.push_str("SYSTEM INFORMATION\n");
        content.push_str(&format!("  OS: {} {} ({})\n",
            report.system_info.distribution,
            report.system_info.version,
            report.system_info.architecture
        ));
        content.push_str(&format!("  Kernel: {}\n", report.system_info.kernel));
        content.push_str(&format!("  Hostname: {}\n", report.system_info.hostname));
        content.push_str(&format!("  Uptime: {}\n", report.system_info.uptime));
        content.push_str(&format!("  CPU: {} cores\n", report.system_info.cpu_count));
        content.push_str(&format!("  Memory: {} / {} GB\n",
            (report.system_info.memory_total - report.system_info.memory_available) / (1024 * 1024 * 1024),
            report.system_info.memory_total / (1024 * 1024 * 1024)
        ));
        content.push_str(&format!("  Disk: {} / {} GB\n\n",
            (report.system_info.disk_total - report.system_info.disk_available) / (1024 * 1024 * 1024),
            report.system_info.disk_total / (1024 * 1024 * 1024)
        ));

        content.push_str("HEALTH CHECK RESULTS\n");
        content.push_str(&format!("  Total checks: {}\n", report.stats.total()));
        content.push_str(&format!("  Passed: {}\n", report.stats.ok_count));
        content.push_str(&format!("  Warnings: {}\n", report.stats.warning_count));
        content.push_str(&format!("  Errors: {}\n", report.stats.error_count));
        content.push_str(&format!("  Critical: {}\n", report.stats.critical_count));
        content.push_str(&format!("  Auto-fixable: {}\n\n", report.stats.fixable_count));

        content.push_str("FINDINGS\n");
        for finding in &report.findings {
            content.push_str(&format!("  [{:?}] {}: {}\n",
                finding.severity,
                finding.name,
                finding.message
            ));

            if let Some(details) = &finding.details {
                content.push_str(&format!("    Details: {}\n", details));
            }

            if let Some(fix) = &finding.fix_hint {
                content.push_str(&format!("    Fix: {}\n", fix));
            }
        }

        if !report.recommendations.is_empty() {
            content.push_str("\nRECOMMENDATIONS\n");
            for rec in &report.recommendations {
                content.push_str(&format!("  • {}\n", rec));
            }
        }

        content
    }

    /// Export as Markdown
    fn export_markdown(&self, report: &HealthReport) -> String {
        let mut content = String::new();

        content.push_str("# pkmgr System Health Report\n\n");
        content.push_str(&format!("**Generated:** {}\n\n", report.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));

        content.push_str("## System Information\n\n");
        content.push_str("| Property | Value |\n");
        content.push_str("|----------|-------|\n");
        content.push_str(&format!("| OS | {} {} ({}) |\n",
            report.system_info.distribution,
            report.system_info.version,
            report.system_info.architecture
        ));
        content.push_str(&format!("| Kernel | {} |\n", report.system_info.kernel));
        content.push_str(&format!("| Hostname | {} |\n", report.system_info.hostname));
        content.push_str(&format!("| Uptime | {} |\n", report.system_info.uptime));
        content.push_str(&format!("| CPU | {} cores |\n", report.system_info.cpu_count));
        content.push_str(&format!("| Memory | {} / {} GB |\n",
            (report.system_info.memory_total - report.system_info.memory_available) / (1024 * 1024 * 1024),
            report.system_info.memory_total / (1024 * 1024 * 1024)
        ));
        content.push_str(&format!("| Disk | {} / {} GB |\n\n",
            (report.system_info.disk_total - report.system_info.disk_available) / (1024 * 1024 * 1024),
            report.system_info.disk_total / (1024 * 1024 * 1024)
        ));

        content.push_str("## Health Check Summary\n\n");
        content.push_str("| Status | Count |\n");
        content.push_str("|--------|-------|\n");
        content.push_str(&format!("| ✅ Passed | {} |\n", report.stats.ok_count));
        content.push_str(&format!("| ℹ️ Info | {} |\n", report.stats.info_count));
        content.push_str(&format!("| ⚠️ Warning | {} |\n", report.stats.warning_count));
        content.push_str(&format!("| ❌ Error | {} |\n", report.stats.error_count));
        content.push_str(&format!("| 🔴 Critical | {} |\n", report.stats.critical_count));
        content.push_str(&format!("| 🔧 Auto-fixable | {} |\n\n", report.stats.fixable_count));

        content.push_str("## Findings\n\n");

        // Group by category
        let mut by_category: std::collections::HashMap<String, Vec<&Finding>> = std::collections::HashMap::new();
        for finding in &report.findings {
            by_category.entry(finding.category.clone()).or_default().push(finding);
        }

        for (category, findings) in by_category {
            content.push_str(&format!("### {}\n\n", category));

            for finding in findings {
                let emoji = finding.severity.emoji();
                content.push_str(&format!("- {} **{}**: {}\n", emoji, finding.name, finding.message));

                if let Some(details) = &finding.details {
                    content.push_str(&format!("  - Details: {}\n", details));
                }

                if let Some(fix) = &finding.fix_hint {
                    content.push_str(&format!("  - Fix: {}\n", fix));
                }
            }
            content.push_str("\n");
        }

        if !report.recommendations.is_empty() {
            content.push_str("## Recommendations\n\n");
            for rec in &report.recommendations {
                content.push_str(&format!("- {}\n", rec));
            }
        }

        content
    }

    /// Export as JSON
    fn export_json(&self, report: &HealthReport) -> Result<String> {
        serde_json::to_string_pretty(report)
            .context("Failed to serialize report to JSON")
    }

    /// Export as HTML
    fn export_html(&self, report: &HealthReport) -> String {
        let markdown = self.export_markdown(report);

        // Basic HTML wrapper with inline CSS
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>pkmgr System Health Report</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; line-height: 1.6; margin: 40px auto; max-width: 900px; padding: 0 20px; }}
        h1 {{ color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }}
        h2 {{ color: #555; margin-top: 30px; }}
        h3 {{ color: #666; }}
        table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}
        th, td {{ text-align: left; padding: 12px; border: 1px solid #ddd; }}
        th {{ background-color: #4CAF50; color: white; }}
        tr:nth-child(even) {{ background-color: #f2f2f2; }}
        ul {{ margin: 20px 0; }}
        li {{ margin: 10px 0; }}
        code {{ background: #f4f4f4; padding: 2px 6px; border-radius: 3px; }}
    </style>
</head>
<body>
{}
</body>
</html>"#, markdown_to_html(&markdown))
    }
}

/// Export format options
pub enum ExportFormat {
    Text,
    Markdown,
    Json,
    Html,
}

/// Basic markdown to HTML conversion
fn markdown_to_html(markdown: &str) -> String {
    markdown
        .replace("# ", "<h1>")
        .replace("\n## ", "</h1>\n<h2>")
        .replace("\n### ", "</h2>\n<h3>")
        .replace("\n\n", "</p>\n<p>")
        .replace("**", "<strong>")
        .replace("*", "<em>")
        .replace("\n- ", "\n<li>")
        .replace("\n|", "\n<tr><td>")
        .replace("|", "</td><td>")
        + "</p>"
}