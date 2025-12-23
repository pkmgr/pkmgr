use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

pub mod patterns;
pub mod analyzer;
pub mod fixer;
pub mod strategies;

// Re-export main types for easier access
pub use analyzer::ErrorAnalyzer;
pub use fixer::ErrorFixer;
pub use strategies::RecoveryStrategies;

/// Error pattern that can be matched and fixed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
    pub patterns: Vec<PatternMatcher>,
    pub fix_strategy: FixStrategy,
    pub success_rate: f32,
    pub platforms: Vec<String>,
    pub package_managers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorCategory {
    Dependency,
    Permission,
    Network,
    DiskSpace,
    Configuration,
    Package,
    Repository,
    Build,
    Signature,
    Lock,
    Library,
    Keyring,
    Database,
    Environment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorSeverity {
    Critical,   // System breaking
    High,       // Operation failure
    Medium,     // Degraded functionality
    Low,        // Minor issue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatcher {
    pub regex: String,
    pub location: MatchLocation,
    pub capture_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchLocation {
    Stdout,
    Stderr,
    ExitCode(i32),
    Combined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixStrategy {
    /// Simple command to run
    Command(Vec<String>),

    /// Multiple commands in sequence
    CommandSequence(Vec<Vec<String>>),

    /// Built-in fix function
    BuiltIn(String),

    /// Rebuild package
    Rebuild { package: String },

    /// Force overwrite files
    ForceOverwrite { patterns: Vec<String> },

    /// Clean and retry
    CleanRetry {
        clean_commands: Vec<Vec<String>>,
        retry_original: bool,
    },

    /// Update system component
    UpdateComponent { component: String },

    /// Reconfigure service
    Reconfigure { service: String },

    /// Environment fix
    EnvironmentFix {
        variables: HashMap<String, String>,
        permanent: bool,
    },

    /// Custom Rust function
    Custom(String),
}

/// Result of analyzing an error
#[derive(Debug, Clone)]
pub struct ErrorAnalysis {
    pub matched_pattern: ErrorPattern,
    pub confidence: f32,
    pub extracted_data: HashMap<String, String>,
    pub suggested_fixes: Vec<FixSuggestion>,
}

/// A suggested fix for an error
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub description: String,
    pub strategy: FixStrategy,
    pub estimated_success: f32,
    pub requires_sudo: bool,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Safe,       // No risk
    Low,        // Minimal risk
    Medium,     // Some risk, reversible
    High,       // Significant risk
}

impl ErrorPattern {
    /// Check if this pattern matches the given error
    pub fn matches(&self, stdout: &str, stderr: &str, exit_code: i32) -> Option<HashMap<String, String>> {
        let mut captured_data = HashMap::new();

        for matcher in &self.patterns {
            let text = match matcher.location {
                MatchLocation::Stdout => stdout,
                MatchLocation::Stderr => stderr,
                MatchLocation::ExitCode(code) => {
                    if code == exit_code {
                        continue;
                    } else {
                        return None;
                    }
                }
                MatchLocation::Combined => &format!("{}\n{}", stdout, stderr),
            };

            if let Ok(re) = Regex::new(&matcher.regex) {
                if let Some(captures) = re.captures(text) {
                    // Extract named capture groups
                    for (i, name) in matcher.capture_groups.iter().enumerate() {
                        if let Some(matched) = captures.get(i + 1) {
                            captured_data.insert(name.clone(), matched.as_str().to_string());
                        }
                    }
                } else {
                    return None; // Pattern didn't match
                }
            }
        }

        Some(captured_data)
    }

    /// Generate fix suggestions for this error
    pub fn generate_suggestions(&self, data: &HashMap<String, String>) -> Vec<FixSuggestion> {
        let mut suggestions = Vec::new();

        // Primary fix strategy
        suggestions.push(FixSuggestion {
            description: self.description.clone(),
            strategy: self.fix_strategy.clone(),
            estimated_success: self.success_rate,
            requires_sudo: self.requires_sudo(),
            risk_level: self.risk_level(),
        });

        // Add alternative strategies based on category
        match self.category {
            ErrorCategory::Dependency => {
                suggestions.push(FixSuggestion {
                    description: "Update all packages and retry".to_string(),
                    strategy: FixStrategy::CommandSequence(vec![
                        vec!["pkmgr".to_string(), "update".to_string(), "all".to_string()],
                        vec!["pkmgr".to_string(), "fix".to_string()],
                    ]),
                    estimated_success: 0.7,
                    requires_sudo: true,
                    risk_level: RiskLevel::Low,
                });
            }
            ErrorCategory::Permission => {
                suggestions.push(FixSuggestion {
                    description: "Retry with elevated privileges".to_string(),
                    strategy: FixStrategy::BuiltIn("retry_with_sudo".to_string()),
                    estimated_success: 0.9,
                    requires_sudo: true,
                    risk_level: RiskLevel::Safe,
                });
            }
            ErrorCategory::Lock => {
                suggestions.push(FixSuggestion {
                    description: "Force remove lock files and retry".to_string(),
                    strategy: FixStrategy::BuiltIn("clear_locks".to_string()),
                    estimated_success: 0.8,
                    requires_sudo: true,
                    risk_level: RiskLevel::Medium,
                });
            }
            _ => {}
        }

        suggestions
    }

    fn requires_sudo(&self) -> bool {
        matches!(
            self.category,
            ErrorCategory::Permission |
            ErrorCategory::Package |
            ErrorCategory::Repository |
            ErrorCategory::Lock
        )
    }

    fn risk_level(&self) -> RiskLevel {
        match &self.fix_strategy {
            FixStrategy::ForceOverwrite { .. } => RiskLevel::High,
            FixStrategy::CleanRetry { .. } => RiskLevel::Medium,
            FixStrategy::Rebuild { .. } => RiskLevel::Low,
            FixStrategy::Command(_) => RiskLevel::Low,
            _ => RiskLevel::Safe,
        }
    }
}

/// Common error patterns database
pub fn get_error_patterns() -> Vec<ErrorPattern> {
    let mut patterns = Vec::new();

    // Arch Linux patterns
    patterns.extend(patterns::arch::get_patterns());

    // Debian/Ubuntu patterns
    patterns.extend(patterns::debian::get_patterns());

    // Fedora/RHEL patterns
    patterns.extend(patterns::fedora::get_patterns());

    // Cross-platform patterns
    patterns.extend(patterns::common::get_patterns());

    patterns
}

/// Find matching error patterns
pub fn analyze_error(
    stdout: &str,
    stderr: &str,
    exit_code: i32,
    platform: Option<&str>,
) -> Vec<ErrorAnalysis> {
    let patterns = get_error_patterns();
    let mut analyses = Vec::new();

    for pattern in patterns {
        // Filter by platform if specified
        if let Some(platform) = platform {
            if !pattern.platforms.is_empty() && !pattern.platforms.contains(&platform.to_string()) {
                continue;
            }
        }

        if let Some(captured_data) = pattern.matches(stdout, stderr, exit_code) {
            let suggestions = pattern.generate_suggestions(&captured_data);

            analyses.push(ErrorAnalysis {
                matched_pattern: pattern.clone(),
                confidence: calculate_confidence(&pattern, &captured_data),
                extracted_data: captured_data,
                suggested_fixes: suggestions,
            });
        }
    }

    // Sort by confidence
    analyses.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    analyses
}

fn calculate_confidence(pattern: &ErrorPattern, _data: &HashMap<String, String>) -> f32 {
    // Base confidence on pattern success rate
    let mut confidence = pattern.success_rate;

    // Adjust based on severity (more severe = more likely to be correct)
    confidence *= match pattern.severity {
        ErrorSeverity::Critical => 1.0,
        ErrorSeverity::High => 0.95,
        ErrorSeverity::Medium => 0.9,
        ErrorSeverity::Low => 0.85,
    };

    confidence.min(1.0)
}