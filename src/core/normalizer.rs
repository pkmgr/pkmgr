use std::collections::HashMap;
use anyhow::{Result, bail};
use crate::core::platform::{PackageManager, PlatformInfo};

/// Package name normalization system
/// Converts universal package names to package manager specific names
pub struct PackageNormalizer {
    mappings: HashMap<String, DistributionMappings>,
}

/// Package mappings for different distributions
#[derive(Debug, Clone)]
pub struct DistributionMappings {
    pub apt: Option<Vec<String>>,
    pub dnf: Option<Vec<String>>,
    pub pacman: Option<Vec<String>>,
    pub brew: Option<Vec<String>>,
    pub winget: Option<Vec<String>>,
    pub choco: Option<Vec<String>>,
    pub scoop: Option<Vec<String>>,
    pub pkg: Option<Vec<String>>,
    pub pkg_add: Option<Vec<String>>,
    pub pkgin: Option<Vec<String>>,
}

impl PackageNormalizer {
    pub fn new() -> Self {
        let mut normalizer = Self {
            mappings: HashMap::new(),
        };
        normalizer.init_mappings();
        normalizer
    }

    /// Initialize the complete mapping table from CLAUDE.md specification
    fn init_mappings(&mut self) {
        // Python
        self.add_mapping("python", DistributionMappings {
            apt: Some(vec!["python3".to_string(), "python3.11".to_string(), "python3.12".to_string()]),
            dnf: Some(vec!["python3".to_string(), "python39".to_string(), "python311".to_string()]),
            pacman: Some(vec!["python".to_string()]),
            brew: Some(vec!["python@3.11".to_string(), "python@3.12".to_string()]),
            winget: Some(vec!["Python.Python.3.11".to_string(), "Python.Python.3.12".to_string()]),
            choco: Some(vec!["python".to_string(), "python3".to_string(), "python311".to_string()]),
            scoop: Some(vec!["python".to_string()]),
            pkg: Some(vec!["python3".to_string()]),
            pkg_add: Some(vec!["python3".to_string()]),
            pkgin: Some(vec!["python39".to_string()]),
        });

        // Node.js
        self.add_mapping("nodejs", DistributionMappings {
            apt: Some(vec!["nodejs".to_string()]),
            dnf: Some(vec!["nodejs".to_string()]),
            pacman: Some(vec!["nodejs".to_string()]),
            brew: Some(vec!["node".to_string()]),
            winget: Some(vec!["OpenJS.NodeJS".to_string()]),
            choco: Some(vec!["nodejs".to_string()]),
            scoop: Some(vec!["nodejs".to_string()]),
            pkg: Some(vec!["node".to_string()]),
            pkg_add: Some(vec!["node".to_string()]),
            pkgin: Some(vec!["nodejs".to_string()]),
        });

        // Aliases for Node.js
        self.add_mapping("node", DistributionMappings {
            apt: Some(vec!["nodejs".to_string()]),
            dnf: Some(vec!["nodejs".to_string()]),
            pacman: Some(vec!["nodejs".to_string()]),
            brew: Some(vec!["node".to_string()]),
            winget: Some(vec!["OpenJS.NodeJS".to_string()]),
            choco: Some(vec!["nodejs".to_string()]),
            scoop: Some(vec!["nodejs".to_string()]),
            pkg: Some(vec!["node".to_string()]),
            pkg_add: Some(vec!["node".to_string()]),
            pkgin: Some(vec!["nodejs".to_string()]),
        });

        // Docker
        self.add_mapping("docker", DistributionMappings {
            apt: Some(vec!["docker-ce".to_string()]), // with repo, never docker.io
            dnf: Some(vec!["docker-ce".to_string(), "podman".to_string()]), // podman as alternative
            pacman: Some(vec!["docker".to_string()]),
            brew: Some(vec!["docker".to_string()]),
            winget: Some(vec!["Docker.DockerDesktop".to_string()]),
            choco: Some(vec!["docker-desktop".to_string()]),
            scoop: Some(vec!["docker".to_string()]),
            pkg: Some(vec!["docker".to_string()]),
            pkg_add: Some(vec!["docker".to_string()]),
            pkgin: Some(vec!["docker".to_string()]),
        });

        // Git
        self.add_mapping("git", DistributionMappings {
            apt: Some(vec!["git".to_string()]),
            dnf: Some(vec!["git".to_string()]),
            pacman: Some(vec!["git".to_string()]),
            brew: Some(vec!["git".to_string()]),
            winget: Some(vec!["Git.Git".to_string()]),
            choco: Some(vec!["git".to_string()]),
            scoop: Some(vec!["git".to_string()]),
            pkg: Some(vec!["git".to_string()]),
            pkg_add: Some(vec!["git".to_string()]),
            pkgin: Some(vec!["git-base".to_string()]),
        });

        // Visual Studio Code
        self.add_mapping("vscode", DistributionMappings {
            apt: Some(vec!["code".to_string()]), // with repo
            dnf: Some(vec!["code".to_string()]), // with repo
            pacman: Some(vec!["visual-studio-code-bin".to_string()]), // AUR
            brew: Some(vec!["visual-studio-code".to_string()]),
            winget: Some(vec!["Microsoft.VisualStudioCode".to_string()]),
            choco: Some(vec!["vscode".to_string()]),
            scoop: Some(vec!["vscode".to_string()]),
            pkg: None, // Not available
            pkg_add: None,
            pkgin: None,
        });

        // Code alias for VS Code
        self.add_mapping("code", DistributionMappings {
            apt: Some(vec!["code".to_string()]),
            dnf: Some(vec!["code".to_string()]),
            pacman: Some(vec!["visual-studio-code-bin".to_string()]),
            brew: Some(vec!["visual-studio-code".to_string()]),
            winget: Some(vec!["Microsoft.VisualStudioCode".to_string()]),
            choco: Some(vec!["vscode".to_string()]),
            scoop: Some(vec!["vscode".to_string()]),
            pkg: None,
            pkg_add: None,
            pkgin: None,
        });

        // Google Chrome
        self.add_mapping("chrome", DistributionMappings {
            apt: Some(vec!["google-chrome-stable".to_string()]), // with repo
            dnf: Some(vec!["google-chrome-stable".to_string()]), // with repo
            pacman: Some(vec!["google-chrome".to_string()]), // AUR
            brew: Some(vec!["google-chrome".to_string()]),
            winget: Some(vec!["Google.Chrome".to_string()]),
            choco: Some(vec!["googlechrome".to_string()]),
            scoop: Some(vec!["googlechrome".to_string()]),
            pkg: Some(vec!["chromium".to_string()]), // Closest available
            pkg_add: Some(vec!["chromium".to_string()]),
            pkgin: Some(vec!["chromium".to_string()]),
        });

        // GCC Build Tools
        self.add_mapping("gcc", DistributionMappings {
            apt: Some(vec!["gcc".to_string(), "build-essential".to_string()]),
            dnf: Some(vec!["gcc".to_string(), "gcc-c++".to_string(), "make".to_string()]),
            pacman: Some(vec!["gcc".to_string(), "base-devel".to_string()]),
            brew: Some(vec!["gcc".to_string()]),
            winget: None, // Use MinGW or MSVC
            choco: Some(vec!["mingw".to_string(), "msys2".to_string()]),
            scoop: Some(vec!["gcc".to_string()]),
            pkg: Some(vec!["gcc".to_string()]),
            pkg_add: Some(vec!["gcc".to_string()]),
            pkgin: Some(vec!["gcc".to_string()]),
        });

        // MySQL
        self.add_mapping("mysql", DistributionMappings {
            apt: Some(vec!["mysql-server".to_string(), "mysql-client".to_string()]),
            dnf: Some(vec!["mysql-server".to_string(), "mysql".to_string()]),
            pacman: Some(vec!["mariadb".to_string()]), // MySQL replaced with MariaDB
            brew: Some(vec!["mysql".to_string()]),
            winget: Some(vec!["Oracle.MySQL".to_string()]),
            choco: Some(vec!["mysql".to_string()]),
            scoop: Some(vec!["mysql".to_string()]),
            pkg: Some(vec!["mysql80-server".to_string()]),
            pkg_add: Some(vec!["mysql-server".to_string()]),
            pkgin: Some(vec!["mysql-server".to_string()]),
        });

        // PostgreSQL
        self.add_mapping("postgresql", DistributionMappings {
            apt: Some(vec!["postgresql".to_string(), "postgresql-client".to_string()]),
            dnf: Some(vec!["postgresql-server".to_string(), "postgresql".to_string()]),
            pacman: Some(vec!["postgresql".to_string()]),
            brew: Some(vec!["postgresql@16".to_string()]),
            winget: Some(vec!["PostgreSQL.PostgreSQL".to_string()]),
            choco: Some(vec!["postgresql".to_string()]),
            scoop: Some(vec!["postgresql".to_string()]),
            pkg: Some(vec!["postgresql15-server".to_string()]),
            pkg_add: Some(vec!["postgresql-server".to_string()]),
            pkgin: Some(vec!["postgresql".to_string()]),
        });

        // Redis
        self.add_mapping("redis", DistributionMappings {
            apt: Some(vec!["redis-server".to_string(), "redis-tools".to_string()]),
            dnf: Some(vec!["redis".to_string()]),
            pacman: Some(vec!["redis".to_string()]),
            brew: Some(vec!["redis".to_string()]),
            winget: Some(vec!["Redis.Redis".to_string()]),
            choco: Some(vec!["redis-64".to_string()]),
            scoop: Some(vec!["redis".to_string()]),
            pkg: Some(vec!["redis".to_string()]),
            pkg_add: Some(vec!["redis".to_string()]),
            pkgin: Some(vec!["redis".to_string()]),
        });

        // Nginx
        self.add_mapping("nginx", DistributionMappings {
            apt: Some(vec!["nginx".to_string()]),
            dnf: Some(vec!["nginx".to_string()]),
            pacman: Some(vec!["nginx".to_string()]),
            brew: Some(vec!["nginx".to_string()]),
            winget: None,
            choco: Some(vec!["nginx".to_string()]),
            scoop: Some(vec!["nginx".to_string()]),
            pkg: Some(vec!["nginx".to_string()]),
            pkg_add: Some(vec!["nginx".to_string()]),
            pkgin: Some(vec!["nginx".to_string()]),
        });

        // Apache
        self.add_mapping("apache", DistributionMappings {
            apt: Some(vec!["apache2".to_string()]),
            dnf: Some(vec!["httpd".to_string()]),
            pacman: Some(vec!["apache".to_string()]),
            brew: Some(vec!["httpd".to_string()]),
            winget: Some(vec!["ApacheFriends.Xampp".to_string()]),
            choco: Some(vec!["apache-httpd".to_string()]),
            scoop: Some(vec!["apache".to_string()]),
            pkg: Some(vec!["apache24".to_string()]),
            pkg_add: Some(vec!["apache-httpd".to_string()]),
            pkgin: Some(vec!["apache".to_string()]),
        });
    }

    /// Add a package mapping
    fn add_mapping(&mut self, universal_name: &str, mappings: DistributionMappings) {
        self.mappings.insert(universal_name.to_string(), mappings);
    }

    /// Normalize a package name for a specific package manager
    pub fn normalize(&self, package_name: &str, package_manager: &PackageManager) -> Result<Vec<String>> {
        // First check for direct mapping
        if let Some(mappings) = self.mappings.get(package_name) {
            let packages = match package_manager {
                PackageManager::Apt => &mappings.apt,
                PackageManager::Dnf => &mappings.dnf,
                PackageManager::Yum => &mappings.dnf, // DNF is YUM successor
                PackageManager::Pacman => &mappings.pacman,
                PackageManager::Homebrew => &mappings.brew,
                PackageManager::Winget => &mappings.winget,
                PackageManager::Chocolatey => &mappings.choco,
                PackageManager::Scoop => &mappings.scoop,
                PackageManager::Pkg => &mappings.pkg,
                PackageManager::PkgAdd => &mappings.pkg_add,
                PackageManager::Pkgin => &mappings.pkgin,
                _ => &None, // Other package managers use original name
            };

            if let Some(package_list) = packages {
                return Ok(package_list.clone());
            }
        }

        // Check for common variations
        if let Some(normalized) = self.check_variations(package_name, package_manager) {
            return Ok(normalized);
        }

        // No mapping found, return original name
        Ok(vec![package_name.to_string()])
    }

    /// Check for common package name variations
    fn check_variations(&self, package_name: &str, package_manager: &PackageManager) -> Option<Vec<String>> {
        // Common patterns to check
        let variations = vec![
            // python/python3 variations
            if package_name == "python3" { Some("python") } else { None },
            if package_name == "python" { Some("python3") } else { None },

            // nodejs/node variations
            if package_name == "node" { Some("nodejs") } else { None },
            if package_name == "nodejs" { Some("node") } else { None },

            // dev/devel package variations
            if package_name.ends_with("-dev") {
                Some(&package_name[..package_name.len()-4])
            } else { None },
            if package_name.ends_with("-devel") {
                Some(&package_name[..package_name.len()-6])
            } else { None },
        ];

        for variation in variations.into_iter().flatten() {
            if let Ok(normalized) = self.normalize(variation, package_manager) {
                if normalized != vec![variation.to_string()] {
                    return Some(normalized);
                }
            }
        }

        None
    }

    /// Get all available alternatives for a package
    pub fn get_alternatives(&self, package_name: &str) -> Vec<(PackageManager, Vec<String>)> {
        let mut alternatives = Vec::new();

        if let Some(mappings) = self.mappings.get(package_name) {
            if let Some(packages) = &mappings.apt {
                alternatives.push((PackageManager::Apt, packages.clone()));
            }
            if let Some(packages) = &mappings.dnf {
                alternatives.push((PackageManager::Dnf, packages.clone()));
            }
            if let Some(packages) = &mappings.pacman {
                alternatives.push((PackageManager::Pacman, packages.clone()));
            }
            if let Some(packages) = &mappings.brew {
                alternatives.push((PackageManager::Homebrew, packages.clone()));
            }
            if let Some(packages) = &mappings.winget {
                alternatives.push((PackageManager::Winget, packages.clone()));
            }
            if let Some(packages) = &mappings.choco {
                alternatives.push((PackageManager::Chocolatey, packages.clone()));
            }
        }

        alternatives
    }

    /// Validate that a package name should not be used (e.g., docker.io)
    pub fn validate_package_name(&self, package_name: &str, package_manager: &PackageManager) -> Result<()> {
        // Warn about deprecated packages
        match (package_name, package_manager) {
            ("docker.io", PackageManager::Apt) => {
                bail!("❌ Use 'docker-ce' instead of 'docker.io' (outdated version)")
            }
            _ => Ok(())
        }
    }

    /// Get smart suggestions for fuzzy matches
    pub fn suggest_package(&self, query: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let query_lower = query.to_lowercase();

        for package_name in self.mappings.keys() {
            // Exact match
            if package_name.to_lowercase() == query_lower {
                suggestions.push(package_name.clone());
                continue;
            }

            // Fuzzy matching (Levenshtein distance ≤ 2)
            if levenshtein_distance(&query_lower, &package_name.to_lowercase()) <= 2 {
                suggestions.push(package_name.clone());
            }

            // Substring matching
            if package_name.to_lowercase().contains(&query_lower) ||
               query_lower.contains(&package_name.to_lowercase()) {
                suggestions.push(package_name.clone());
            }
        }

        // Remove duplicates and sort by relevance
        suggestions.sort();
        suggestions.dedup();
        suggestions.truncate(5); // Limit to 5 suggestions

        suggestions
    }

    /// Check if this package requires repository setup
    pub fn requires_repository(&self, package_name: &str) -> Option<&'static str> {
        match package_name {
            "docker-ce" | "docker" => Some("docker"),
            "code" | "vscode" => Some("microsoft"),
            "google-chrome-stable" | "chrome" => Some("google"),
            "postgresql" => Some("postgresql"),
            _ => None,
        }
    }
}

impl Default for PackageNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 { return b_len; }
    if b_len == 0 { return a_len; }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    // Initialize first row and column
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    // Fill the matrix
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i-1][j] + 1,     // deletion
                    matrix[i][j-1] + 1      // insertion
                ),
                matrix[i-1][j-1] + cost     // substitution
            );
        }
    }

    matrix[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_python() {
        let normalizer = PackageNormalizer::new();

        let result = normalizer.normalize("python", &PackageManager::Apt).unwrap();
        assert!(result.contains(&"python3".to_string()));

        let result = normalizer.normalize("python", &PackageManager::Pacman).unwrap();
        assert_eq!(result, vec!["python".to_string()]);
    }

    #[test]
    fn test_normalize_docker() {
        let normalizer = PackageNormalizer::new();

        let result = normalizer.normalize("docker", &PackageManager::Apt).unwrap();
        assert_eq!(result, vec!["docker-ce".to_string()]);

        let result = normalizer.normalize("docker", &PackageManager::Dnf).unwrap();
        assert!(result.contains(&"docker-ce".to_string()));
        assert!(result.contains(&"podman".to_string()));
    }

    #[test]
    fn test_validate_package_name() {
        let normalizer = PackageNormalizer::new();

        let result = normalizer.validate_package_name("docker.io", &PackageManager::Apt);
        assert!(result.is_err());

        let result = normalizer.validate_package_name("docker-ce", &PackageManager::Apt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("docker", "doker"), 1);
        assert_eq!(levenshtein_distance("python", "pyton"), 1);
        assert_eq!(levenshtein_distance("nodejs", "nodjs"), 1);
        assert_eq!(levenshtein_distance("git", "get"), 1);
    }

    #[test]
    fn test_suggest_package() {
        let normalizer = PackageNormalizer::new();

        let suggestions = normalizer.suggest_package("doker");
        assert!(suggestions.contains(&"docker".to_string()));

        let suggestions = normalizer.suggest_package("pyton");
        assert!(suggestions.contains(&"python".to_string()));
    }
}