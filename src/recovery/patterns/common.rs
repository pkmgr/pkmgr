use crate::recovery::{ErrorPattern, ErrorCategory, ErrorSeverity, PatternMatcher, MatchLocation, FixStrategy};
use std::collections::HashMap;

/// Get cross-platform common error patterns
pub fn get_patterns() -> Vec<ErrorPattern> {
    vec![
        // Network timeout
        ErrorPattern {
            id: "common_network_timeout".to_string(),
            name: "Network timeout".to_string(),
            description: "Connection timed out while downloading".to_string(),
            category: ErrorCategory::Network,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"Connection timed out".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Could not connect to .+ \(.+\), connection timed out".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("retry_with_timeout".to_string()),
            success_rate: 0.8,
            platforms: vec![],
            package_managers: vec![],
        },

        // DNS failure
        ErrorPattern {
            id: "common_dns_failure".to_string(),
            name: "DNS resolution failure".to_string(),
            description: "Failed to resolve hostname".to_string(),
            category: ErrorCategory::Network,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Could not resolve host: (.+)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["host".to_string()],
                },
                PatternMatcher {
                    regex: r"Name or service not known".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["systemctl".to_string(), "restart".to_string(), "systemd-resolved".to_string()],
                vec!["echo".to_string(), "nameserver 8.8.8.8".to_string(), "|".to_string(),
                     "tee".to_string(), "/etc/resolv.conf".to_string()],
            ]),
            success_rate: 0.85,
            platforms: vec![],
            package_managers: vec![],
        },

        // Permission denied
        ErrorPattern {
            id: "common_permission_denied".to_string(),
            name: "Permission denied".to_string(),
            description: "Insufficient privileges for operation".to_string(),
            category: ErrorCategory::Permission,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Permission denied".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Operation not permitted".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("retry_with_sudo".to_string()),
            success_rate: 0.95,
            platforms: vec![],
            package_managers: vec![],
        },

        // Disk full
        ErrorPattern {
            id: "common_disk_full".to_string(),
            name: "Disk full".to_string(),
            description: "No space left on device".to_string(),
            category: ErrorCategory::DiskSpace,
            severity: ErrorSeverity::Critical,
            patterns: vec![
                PatternMatcher {
                    regex: r"No space left on device".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Disk quota exceeded".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("cleanup_disk_space".to_string()),
            success_rate: 0.7,
            platforms: vec![],
            package_managers: vec![],
        },

        // SSL certificate error
        ErrorPattern {
            id: "common_ssl_cert".to_string(),
            name: "SSL certificate error".to_string(),
            description: "SSL certificate verification failed".to_string(),
            category: ErrorCategory::Network,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"SSL certificate problem".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"certificate verify failed".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["update-ca-certificates".to_string()],
            ]),
            success_rate: 0.8,
            platforms: vec![],
            package_managers: vec![],
        },

        // Build tools missing
        ErrorPattern {
            id: "common_build_tools".to_string(),
            name: "Build tools missing".to_string(),
            description: "Required build tools not installed".to_string(),
            category: ErrorCategory::Build,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"gcc: command not found".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"make: command not found".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"error: Microsoft Visual C\+\+ 14.0 is required".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("install_build_tools".to_string()),
            success_rate: 0.95,
            platforms: vec![],
            package_managers: vec![],
        },

        // Library missing
        ErrorPattern {
            id: "common_lib_missing".to_string(),
            name: "Library missing".to_string(),
            description: "Required library not found".to_string(),
            category: ErrorCategory::Library,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"error while loading shared libraries: (.+): cannot open shared object file".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["library".to_string()],
                },
                PatternMatcher {
                    regex: r"ImportError: (.+\.so\.\d+): cannot open shared object file".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["library".to_string()],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("find_and_install_library".to_string()),
            success_rate: 0.85,
            platforms: vec![],
            package_managers: vec![],
        },

        // Environment variable missing
        ErrorPattern {
            id: "common_env_var".to_string(),
            name: "Environment variable missing".to_string(),
            description: "Required environment variable not set".to_string(),
            category: ErrorCategory::Environment,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: Environment variable '(\w+)' not set".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["variable".to_string()],
                },
                PatternMatcher {
                    regex: r"(\w+) is not set".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["variable".to_string()],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("suggest_env_var".to_string()),
            success_rate: 0.9,
            platforms: vec![],
            package_managers: vec![],
        },

        // Python version mismatch
        ErrorPattern {
            id: "common_python_version".to_string(),
            name: "Python version mismatch".to_string(),
            description: "Wrong Python version for package".to_string(),
            category: ErrorCategory::Environment,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"requires Python '([<>=]+\d+\.\d+)'".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["version".to_string()],
                },
                PatternMatcher {
                    regex: r"This version of (.+) requires Python ([<>=]+\d+\.\d+)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string(), "version".to_string()],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("switch_python_version".to_string()),
            success_rate: 0.85,
            platforms: vec![],
            package_managers: vec!["pip".to_string(), "pip3".to_string()],
        },

        // Node version mismatch
        ErrorPattern {
            id: "common_node_version".to_string(),
            name: "Node version mismatch".to_string(),
            description: "Wrong Node.js version for package".to_string(),
            category: ErrorCategory::Environment,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r#"engine "node" is incompatible with this module"#.to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r#"Expected version "([^"]+)""#.to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["version".to_string()],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("switch_node_version".to_string()),
            success_rate: 0.85,
            platforms: vec![],
            package_managers: vec!["npm".to_string(), "yarn".to_string()],
        },
    ]
}