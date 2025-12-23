use crate::recovery::{ErrorPattern, ErrorCategory, ErrorSeverity, PatternMatcher, MatchLocation, FixStrategy};

/// Get Fedora/RHEL specific error patterns
pub fn get_patterns() -> Vec<ErrorPattern> {
    vec![
        // DNF database corruption
        ErrorPattern {
            id: "fedora_db_corrupt".to_string(),
            name: "DNF database corrupted".to_string(),
            description: "RPM database is corrupted and needs rebuilding".to_string(),
            category: ErrorCategory::Database,
            severity: ErrorSeverity::Critical,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: rpmdb: .+ cannot allocate memory".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"error: cannot open Packages database".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["rm".to_string(), "-f".to_string(), "/var/lib/rpm/__db*".to_string()],
                vec!["rpm".to_string(), "--rebuilddb".to_string()],
                vec!["dnf".to_string(), "clean".to_string(), "all".to_string()],
            ]),
            success_rate: 1.0,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string(), "yum".to_string()],
        },

        // Module conflicts
        ErrorPattern {
            id: "fedora_module_conflict".to_string(),
            name: "Module stream conflict".to_string(),
            description: "DNF module streams are conflicting".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Modular dependency problems:".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"conflicting requests".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["dnf".to_string(), "module".to_string(), "reset".to_string(), "*".to_string()],
                vec!["dnf".to_string(), "distro-sync".to_string(), "-y".to_string()],
            ]),
            success_rate: 0.9,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string()],
        },

        // Transaction check error
        ErrorPattern {
            id: "fedora_transaction_check".to_string(),
            name: "Transaction check error".to_string(),
            description: "DNF transaction check failed".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Error: Transaction check error:".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"file (.+) conflicts between attempted installs".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["file".to_string()],
                },
            ],
            fix_strategy: FixStrategy::Command(vec![
                "dnf".to_string(),
                "install".to_string(),
                "--allowerasing".to_string(),
                "-y".to_string(),
            ]),
            success_rate: 0.85,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string()],
        },

        // GPG check failed
        ErrorPattern {
            id: "fedora_gpg_check".to_string(),
            name: "GPG check failed".to_string(),
            description: "Package GPG signature verification failed".to_string(),
            category: ErrorCategory::Signature,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"GPG key retrieval failed: \[Errno 14\]".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Public key for (.+) is not installed".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string()],
                },
            ],
            fix_strategy: FixStrategy::Command(vec![
                "dnf".to_string(),
                "install".to_string(),
                "-y".to_string(),
                "--nogpgcheck".to_string(),
            ]),
            success_rate: 0.9,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string(), "yum".to_string()],
        },

        // Cache corruption
        ErrorPattern {
            id: "fedora_cache_corrupt".to_string(),
            name: "Cache corrupted".to_string(),
            description: "DNF cache is corrupted".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"Cache-only enabled but no cache".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Metadata file does not match checksum".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["dnf".to_string(), "clean".to_string(), "all".to_string()],
                vec!["dnf".to_string(), "makecache".to_string()],
            ]),
            success_rate: 0.99,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string(), "yum".to_string()],
        },

        // Dependency resolution
        ErrorPattern {
            id: "fedora_dep_resolution".to_string(),
            name: "Dependency resolution failed".to_string(),
            description: "Cannot resolve package dependencies".to_string(),
            category: ErrorCategory::Dependency,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Error: Problem: package .+ requires".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"nothing provides (.+) needed by".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["dependency".to_string()],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["dnf".to_string(), "update".to_string(), "-y".to_string()],
                vec!["dnf".to_string(), "install".to_string(), "--best".to_string(), "--allowerasing".to_string()],
            ]),
            success_rate: 0.8,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string()],
        },

        // Protected packages
        ErrorPattern {
            id: "fedora_protected".to_string(),
            name: "Protected package conflict".to_string(),
            description: "Attempting to remove protected package".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Error: This command has to be run with superuser privileges".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Error: Problem: The operation would result in removing the following protected packages".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::Command(vec![
                "dnf".to_string(),
                "install".to_string(),
                "--setopt=protected_packages=".to_string(),
                "-y".to_string(),
            ]),
            success_rate: 0.7,
            platforms: vec!["fedora".to_string(), "rhel".to_string(), "centos".to_string()],
            package_managers: vec!["dnf".to_string()],
        },
    ]
}