use crate::recovery::{ErrorPattern, ErrorCategory, ErrorSeverity, PatternMatcher, MatchLocation, FixStrategy};

/// Get Debian/Ubuntu specific error patterns
pub fn get_patterns() -> Vec<ErrorPattern> {
    vec![
        // Broken dependencies
        ErrorPattern {
            id: "debian_broken_deps".to_string(),
            name: "Broken dependencies".to_string(),
            description: "Package has unmet dependencies".to_string(),
            category: ErrorCategory::Dependency,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"The following packages have unmet dependencies:".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"(\S+) : Depends: (\S+)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string(), "dependency".to_string()],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["apt-get".to_string(), "update".to_string()],
                vec!["apt-get".to_string(), "--fix-broken".to_string(), "install".to_string(), "-y".to_string()],
            ]),
            success_rate: 0.98,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // dpkg interrupted
        ErrorPattern {
            id: "debian_dpkg_interrupted".to_string(),
            name: "dpkg was interrupted".to_string(),
            description: "Previous dpkg operation was interrupted".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"E: dpkg was interrupted".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::Command(vec![
                "dpkg".to_string(),
                "--configure".to_string(),
                "-a".to_string(),
            ]),
            success_rate: 0.99,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // Lock files
        ErrorPattern {
            id: "debian_lock_held".to_string(),
            name: "APT lock held".to_string(),
            description: "Another process is using APT".to_string(),
            category: ErrorCategory::Lock,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"Could not get lock /var/lib/dpkg/lock-frontend".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Unable to acquire the dpkg frontend lock".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["killall".to_string(), "apt".to_string(), "apt-get".to_string()],
                vec!["rm".to_string(), "-f".to_string(), "/var/lib/dpkg/lock-frontend".to_string()],
                vec!["rm".to_string(), "-f".to_string(), "/var/lib/dpkg/lock".to_string()],
                vec!["rm".to_string(), "-f".to_string(), "/var/cache/apt/archives/lock".to_string()],
                vec!["dpkg".to_string(), "--configure".to_string(), "-a".to_string()],
            ]),
            success_rate: 0.95,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // GPG key error
        ErrorPattern {
            id: "debian_gpg_error".to_string(),
            name: "GPG key error".to_string(),
            description: "Repository GPG key is missing or invalid".to_string(),
            category: ErrorCategory::Signature,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"GPG error: .+ NO_PUBKEY ([A-F0-9]+)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["key".to_string()],
                },
            ],
            fix_strategy: FixStrategy::Command(vec![
                "apt-key".to_string(),
                "adv".to_string(),
                "--keyserver".to_string(),
                "keyserver.ubuntu.com".to_string(),
                "--recv-keys".to_string(),
                "{key}".to_string(),
            ]),
            success_rate: 0.95,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // Hash sum mismatch
        ErrorPattern {
            id: "debian_hash_mismatch".to_string(),
            name: "Hash sum mismatch".to_string(),
            description: "Package file checksum doesn't match".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"Hash Sum mismatch".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["apt-get".to_string(), "clean".to_string()],
                vec!["apt-get".to_string(), "update".to_string()],
            ]),
            success_rate: 0.98,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // Disk space
        ErrorPattern {
            id: "debian_no_space".to_string(),
            name: "No space left".to_string(),
            description: "Insufficient disk space for installation".to_string(),
            category: ErrorCategory::DiskSpace,
            severity: ErrorSeverity::Critical,
            patterns: vec![
                PatternMatcher {
                    regex: r"You don't have enough free space in /var/cache/apt/archives".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"No space left on device".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["apt-get".to_string(), "clean".to_string()],
                vec!["apt-get".to_string(), "autoclean".to_string()],
                vec!["apt-get".to_string(), "autoremove".to_string(), "--purge".to_string()],
            ]),
            success_rate: 0.8,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // Repository not found
        ErrorPattern {
            id: "debian_repo_404".to_string(),
            name: "Repository not found".to_string(),
            description: "APT repository returns 404 error".to_string(),
            category: ErrorCategory::Repository,
            severity: ErrorSeverity::Medium,
            patterns: vec![
                PatternMatcher {
                    regex: r"Err:.+ 404  Not Found".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec![],
                },
                PatternMatcher {
                    regex: r"Failed to fetch (.+)  404  Not Found".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["url".to_string()],
                },
            ],
            fix_strategy: FixStrategy::BuiltIn("fix_404_repos".to_string()),
            success_rate: 0.85,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },

        // Post-install script failure
        ErrorPattern {
            id: "debian_postinst_fail".to_string(),
            name: "Post-install script failed".to_string(),
            description: "Package post-installation script returned error".to_string(),
            category: ErrorCategory::Package,
            severity: ErrorSeverity::High,
            patterns: vec![
                PatternMatcher {
                    regex: r"subprocess installed post-installation script returned error exit status (\d+)".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["exit_code".to_string()],
                },
                PatternMatcher {
                    regex: r"package (.+) failed to install/upgrade".to_string(),
                    location: MatchLocation::Stderr,
                    capture_groups: vec!["package".to_string()],
                },
            ],
            fix_strategy: FixStrategy::CommandSequence(vec![
                vec!["dpkg".to_string(), "--configure".to_string(), "-a".to_string()],
                vec!["apt-get".to_string(), "--fix-broken".to_string(), "install".to_string()],
            ]),
            success_rate: 0.75,
            platforms: vec!["debian".to_string(), "ubuntu".to_string()],
            package_managers: vec!["apt".to_string(), "apt-get".to_string()],
        },
    ]
}