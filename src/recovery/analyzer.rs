use anyhow::{Context, Result};
use crate::ui::output::Output;
use crate::core::platform::PlatformInfo;
use super::{analyze_error, ErrorAnalysis, ErrorSeverity, RiskLevel};

pub struct ErrorAnalyzer {
    output: Output,
    platform: PlatformInfo,
}

impl ErrorAnalyzer {
    pub fn new(output: Output, platform: PlatformInfo) -> Self {
        Self { output, platform }
    }

    /// Analyze command output for errors and suggest fixes
    pub fn analyze(&self, stdout: &str, stderr: &str, exit_code: i32) -> Vec<ErrorAnalysis> {
        if exit_code == 0 && stderr.is_empty() {
            return Vec::new();
        }

        self.output.progress("Analyzing error output...");

        let platform_str = self.get_platform_string();
        let analyses = analyze_error(stdout, stderr, exit_code, Some(&platform_str));

        if !analyses.is_empty() {
            self.output.info(&format!("Found {} potential fixes", analyses.len()));
        }

        analyses
    }

    /// Display error analysis results
    pub fn display_analysis(&self, analyses: &[ErrorAnalysis]) {
        if analyses.is_empty() {
            self.output.info("No known error patterns matched");
            return;
        }

        self.output.section("Error Analysis");

        for (i, analysis) in analyses.iter().enumerate() {
            let pattern = &analysis.matched_pattern;

            // Show error info
            self.output.info(&format!("{}. {} (confidence: {:.0}%)",
                i + 1,
                pattern.name,
                analysis.confidence * 100.0
            ));

            // Show severity
            let severity_emoji = match pattern.severity {
                ErrorSeverity::Critical => "🔴",
                ErrorSeverity::High => "🟠",
                ErrorSeverity::Medium => "🟡",
                ErrorSeverity::Low => "🟢",
            };
            self.output.info(&format!("   {} Severity: {:?}", severity_emoji, pattern.severity));

            // Show description
            self.output.info(&format!("   Description: {}", pattern.description));

            // Show extracted data
            if !analysis.extracted_data.is_empty() {
                self.output.info("   Extracted data:");
                for (key, value) in &analysis.extracted_data {
                    self.output.info(&format!("     {}: {}", key, value));
                }
            }

            // Show suggested fixes
            if !analysis.suggested_fixes.is_empty() {
                self.output.info("   Suggested fixes:");
                for (j, fix) in analysis.suggested_fixes.iter().enumerate() {
                    let risk_emoji = match fix.risk_level {
                        RiskLevel::Safe => "✅",
                        RiskLevel::Low => "⚠️",
                        RiskLevel::Medium => "⚡",
                        RiskLevel::High => "🔥",
                    };

                    self.output.info(&format!(
                        "     {}. {} {} (success rate: {:.0}%)",
                        j + 1,
                        risk_emoji,
                        fix.description,
                        fix.estimated_success * 100.0
                    ));

                    if fix.requires_sudo {
                        self.output.info("        Requires administrator privileges");
                    }
                }
            }
        }
    }

    /// Get the best fix suggestion
    pub fn get_best_fix<'a>(&self, analyses: &'a [ErrorAnalysis]) -> Option<&'a super::FixSuggestion> {
        analyses.first()
            .and_then(|analysis| analysis.suggested_fixes.first())
    }

    /// Check if automatic fix should be attempted
    pub fn should_auto_fix(&self, analysis: &ErrorAnalysis) -> bool {
        // Auto-fix if confidence is high and risk is low
        if analysis.confidence >= 0.9 {
            if let Some(fix) = analysis.suggested_fixes.first() {
                return fix.risk_level <= RiskLevel::Low && fix.estimated_success >= 0.8;
            }
        }
        false
    }

    fn get_platform_string(&self) -> String {
        let dist = self.platform.distribution.as_ref()
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        if dist.contains("ubuntu") {
            "ubuntu".to_string()
        } else if dist.contains("debian") {
            "debian".to_string()
        } else if dist.contains("fedora") {
            "fedora".to_string()
        } else if dist.contains("arch") {
            "arch".to_string()
        } else if dist.contains("centos") {
            "centos".to_string()
        } else if dist.contains("rhel") {
            "rhel".to_string()
        } else {
            "linux".to_string()
        }
    }
}