use crate::recovery::{ErrorPattern, ErrorCategory, ErrorSeverity, PatternMatcher, MatchLocation, FixStrategy};
use std::collections::HashMap;

/// Get Arch Linux specific error patterns
pub fn get_patterns() -> Vec<ErrorPattern> {
    vec![
        // File exists in filesystem error
        ErrorPattern {
            id: "arch_file_exists".to_string(),
            name: "File exists in filesystem".to_string(),
            description: "Package file conflicts with existing filesystem files".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: failed to commit transaction \(conflicting files\)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"(\S+): (/\S+) exists in filesystem".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string(), "file".to_string()],
                },
            ],
            fix_strategy: FixStrategy::ForceOverwrite {
                patterns: vec!["--overwrite".to_string(), "'*'".to_string()],
            },
            success_rate: 0.98,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["pacman".to_string(), "yay".to_string(), "paru".to_string()],
        },

        // Partial upgrade error
        ErrorPattern {
            id: "arch_partial_upgrade".to_string(),
            name: "Partial upgrade detected".to_string(),
            description: "System is in partial upgrade state, full system upgrade required".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::Critical,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: failed to prepare transaction \(could not satisfy dependencies\)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"installing .+ breaks dependency".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["pacman".to_string(), "-Syu".to_string(), "--noconfirm".to_string()],
                vec!["yay".to_string(), "-Sua".to_string(), "--noconfirm".to_string()],
            ]),
            success_rate: 0.95,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["pacman".to_string()],
        },

        // Keyring issues
        ErrorPattern {
            id: "arch_keyring_outdated".to_string(),
            name: "Keyring outdated".to_string(),
            description: "Arch Linux keyring needs to be updated".to_string(),
            category: ErrorCategory::Keyring,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: .+: signature from .+ is marginal trust".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"error: .+: signature from .+ is unknown trust".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["pacman".to_string(), "-Sy".to_string(), "archlinux-keyring".to_string(), "--noconfirm".to_string()],
                vec!["pacman-key".to_string(), "--refresh-keys".to_string()],
            ]),
            success_rate: 0.99,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["pacman".to_string()],
        },

        // Database lock
        ErrorPattern {
            id: "arch_db_locked".to_string(),
            name: "Database locked".to_string(),
            description: "Pacman database is locked by another process".to_string(),
            category: ErrorCategory::Lock,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: could not lock database: File exists".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"if you're sure a package manager is not already running".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::Command(vec![
                "rm".to_string(),
                "/var/lib/pacman/db.lck".to_string(),
            ]),
            success_rate: 1.0,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["pacman".to_string()],
        },

        // AUR build failure
        ErrorPattern {
            id: "arch_aur_build_fail".to_string(),
            name: "AUR package build failure".to_string(),
            description: "Failed to build AUR package".to_string(),
            category: ErrorCategory::Build,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"==> ERROR: A failure occurred in build\(\)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"error: failed to build '(.+)'".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string()],
                },
            ],
            fix_strategy: FixStrategy::CleanRetry {
                clean_commands: vec![
                    vec!["yay".to_string(), "-Scc".to_string(), "--noconfirm".to_string()],
                    vec!["rm".to_string(), "-rf".to_string(), "~/.cache/yay/*".to_string()],
                ],
                retry_original: true,
            },
            success_rate: 0.95,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["yay".to_string(), "paru".to_string()],
        },

        // GPG key issues
        ErrorPattern {
            id: "arch_gpg_key_missing".to_string(),
            name: "GPG key missing".to_string(),
            description: "Required GPG key is not in keyring".to_string(),
            category: ErrorCategory::Signature,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"Error: Problem importing keys".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"gpg: keyserver receive failed: (.+)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["error".to_string()],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["gpg".to_string(), "--keyserver".to_string(), "keyserver.ubuntu.com".to_string(), "--recv-keys".to_string(), "{key}".to_string()],
                vec!["pacman-key".to_string(), "--lsign-key".to_string(), "{key}".to_string()],
            ]),
            success_rate: 0.9,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["yay".to_string(), "paru".to_string()],
        },

        // Corrupted package
        ErrorPattern {
            id: "arch_corrupted_package".to_string(),
            name: "Corrupted package".to_string(),
            description: "Package file is corrupted or invalid".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"error: could not open file .+: Unrecognized archive format".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"error: '(.+)': invalid or corrupted package".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string()],
                },
            ],
            fix_strategy: FixStrategy::CleanRetry {
                clean_commands: vec![
                    vec!["rm".to_string(), "-rf".to_string(), "/var/cache/pacman/pkg/{package}*".to_string()],
                ],
                retry_original: true,
            },
            success_rate: 0.99,
            platforms: vec!["arch".to_string()],
            package_managers: vec!["pacman".to_string()],
        },
    ]
}