use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::fs;
use crate::ui::output::Output;
use super::{Repository, RepositoryType, TrustLevel, get_known_repositories};

pub struct RepositoryDetector {
    output: Output,
}

impl RepositoryDetector {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Auto-detect package that needs a repository
    pub fn detect_required_repository(&self, package: &str) -> Option<Repository> {
        let known_repos = get_known_repositories();
        let package_lower = package.to_lowercase();

        // Docker packages
        if package_lower == "docker-ce" ||
           package_lower == "docker-ce-cli" ||
           package_lower.starts_with("containerd") {
            return self.create_docker_repository();
        }

        // PostgreSQL packages
        if package_lower.starts_with("postgresql-") &&
           package_lower.contains(|c: char| c.is_ascii_digit()) {
            return self.create_postgresql_repository();
        }

        // MongoDB packages
        if package_lower.starts_with("mongodb") {
            return self.create_mongodb_repository();
        }

        // Microsoft packages
        if package_lower == "code" ||
           package_lower == "vscode" ||
           package_lower == "microsoft-edge-stable" ||
           package_lower.starts_with("dotnet") ||
           package_lower == "powershell" {
            return self.create_microsoft_repository();
        }

        // HashiCorp packages
        if package_lower == "terraform" ||
           package_lower == "vault" ||
           package_lower == "consul" ||
           package_lower == "nomad" ||
           package_lower == "packer" {
            return self.create_hashicorp_repository();
        }

        // Kubernetes packages
        if package_lower == "kubectl" ||
           package_lower == "kubeadm" ||
           package_lower == "kubelet" {
            return self.create_kubernetes_repository();
        }

        None
    }

    /// Create Docker repository configuration
    fn create_docker_repository(&self) -> Option<Repository> {
        let os = self.detect_os_type()?;
        let codename = self.get_os_codename();

        let url = match os {
            OsType::Debian | OsType::Ubuntu => {
                format!("https://download.docker.com/linux/{}",
                    if os == OsType::Ubuntu { "ubuntu" } else { "debian" })
            }
            OsType::Fedora | OsType::RedHat | OsType::CentOS => {
                "https://download.docker.com/linux/centos".to_string()
            }
            _ => return None,
        };

        let mut repo = Repository::new(
            "docker-ce".to_string(),
            url.clone(),
            self.get_repo_type_for_os(&os),
        );

        repo.metadata.vendor = Some("Docker Inc.".to_string());
        repo.metadata.description = Some("Docker CE repository".to_string());
        repo.metadata.is_verified = true;
        repo.metadata.trust_level = TrustLevel::Verified;

        if let Some(codename) = codename {
            repo.suites = vec![codename];
        }
        repo.components = vec!["stable".to_string()];

        // Add GPG key info
        if let Some(known) = get_known_repositories().iter()
            .find(|k| k.name == "docker") {
            if let Some(fingerprint) = known.gpg_fingerprint {
                repo.gpg_key = Some(super::GpgKeyInfo {
                    fingerprint: fingerprint.to_string(),
                    key_id: fingerprint[fingerprint.len()-8..].to_string(),
                    key_server: None,
                    key_url: known.gpg_key_url.map(|s| s.to_string()),
                    trusted: false,
                    expires: None,
                    last_refreshed: None,
                });
            }
        }

        Some(repo)
    }

    /// Create PostgreSQL repository configuration
    fn create_postgresql_repository(&self) -> Option<Repository> {
        let os = self.detect_os_type()?;
        let codename = self.get_os_codename();

        let url = match os {
            OsType::Debian | OsType::Ubuntu => {
                "https://apt.postgresql.org/pub/repos/apt".to_string()
            }
            OsType::Fedora | OsType::RedHat | OsType::CentOS => {
                "https://download.postgresql.org/pub/repos/yum".to_string()
            }
            _ => return None,
        };

        let mut repo = Repository::new(
            "pgdg".to_string(),
            url,
            self.get_repo_type_for_os(&os),
        );

        repo.metadata.vendor = Some("PostgreSQL Global Development Group".to_string());
        repo.metadata.description = Some("PostgreSQL PGDG repository".to_string());
        repo.metadata.is_verified = true;
        repo.metadata.trust_level = TrustLevel::Verified;

        if let Some(codename) = codename {
            repo.suites = vec![format!("{}-pgdg", codename)];
        }
        repo.components = vec!["main".to_string()];

        Some(repo)
    }

    /// Create MongoDB repository configuration
    fn create_mongodb_repository(&self) -> Option<Repository> {
        let os = self.detect_os_type()?;
        let version = "7.0"; // Default to latest stable

        let url = match os {
            OsType::Ubuntu => {
                format!("https://repo.mongodb.org/apt/ubuntu")
            }
            OsType::Debian => {
                format!("https://repo.mongodb.org/apt/debian")
            }
            OsType::RedHat | OsType::CentOS => {
                format!("https://repo.mongodb.org/yum/redhat")
            }
            _ => return None,
        };

        let mut repo = Repository::new(
            "mongodb".to_string(),
            url,
            self.get_repo_type_for_os(&os),
        );

        repo.metadata.vendor = Some("MongoDB Inc.".to_string());
        repo.metadata.description = Some("MongoDB official repository".to_string());
        repo.metadata.is_verified = true;
        repo.metadata.trust_level = TrustLevel::Verified;

        Some(repo)
    }

    /// Create Microsoft repository configuration
    fn create_microsoft_repository(&self) -> Option<Repository> {
        let os = self.detect_os_type()?;
        let arch = self.detect_architecture();

        let url = match os {
            OsType::Ubuntu => {
                format!("https://packages.microsoft.com/ubuntu/22.04/prod")
            }
            OsType::Debian => {
                format!("https://packages.microsoft.com/debian/12/prod")
            }
            OsType::Fedora => {
                format!("https://packages.microsoft.com/fedora/39/prod")
            }
            OsType::RedHat | OsType::CentOS => {
                format!("https://packages.microsoft.com/rhel/9/prod")
            }
            _ => return None,
        };

        let mut repo = Repository::new(
            "microsoft".to_string(),
            url,
            self.get_repo_type_for_os(&os),
        );

        repo.metadata.vendor = Some("Microsoft Corporation".to_string());
        repo.metadata.description = Some("Microsoft package repository".to_string());
        repo.metadata.is_verified = true;
        repo.metadata.trust_level = TrustLevel::Verified;
        repo.architectures = vec![arch];

        Some(repo)
    }

    /// Create HashiCorp repository configuration
    fn create_hashicorp_repository(&self) -> Option<Repository> {
        let os = self.detect_os_type()?;

        let url = match os {
            OsType::Debian | OsType::Ubuntu => {
                "https://apt.releases.hashicorp.com".to_string()
            }
            OsType::Fedora | OsType::RedHat | OsType::CentOS => {
                "https://rpm.releases.hashicorp.com".to_string()
            }
            _ => return None,
        };

        let mut repo = Repository::new(
            "hashicorp".to_string(),
            url,
            self.get_repo_type_for_os(&os),
        );

        repo.metadata.vendor = Some("HashiCorp".to_string());
        repo.metadata.description = Some("HashiCorp official repository".to_string());
        repo.metadata.is_verified = true;
        repo.metadata.trust_level = TrustLevel::Verified;

        Some(repo)
    }

    /// Create Kubernetes repository configuration
    fn create_kubernetes_repository(&self) -> Option<Repository> {
        let os = self.detect_os_type()?;

        let url = match os {
            OsType::Debian | OsType::Ubuntu => {
                "https://packages.cloud.google.com/apt".to_string()
            }
            OsType::Fedora | OsType::RedHat | OsType::CentOS => {
                "https://packages.cloud.google.com/yum".to_string()
            }
            _ => return None,
        };

        let mut repo = Repository::new(
            "kubernetes".to_string(),
            url,
            self.get_repo_type_for_os(&os),
        );

        repo.metadata.vendor = Some("Kubernetes".to_string());
        repo.metadata.description = Some("Kubernetes official repository".to_string());
        repo.metadata.is_verified = true;
        repo.metadata.trust_level = TrustLevel::Verified;
        repo.suites = vec!["kubernetes-xenial".to_string()];
        repo.components = vec!["main".to_string()];

        Some(repo)
    }

    /// Detect if a URL is a mirror of a known repository
    pub fn detect_mirror(&self, url: &str) -> Option<String> {
        // Common mirror patterns
        let mirror_patterns = vec![
            // China mirrors
            ("mirrors.aliyun.com", "Aliyun Mirror"),
            ("mirrors.tuna.tsinghua.edu.cn", "TUNA Mirror"),
            ("mirrors.ustc.edu.cn", "USTC Mirror"),
            ("mirrors.cloud.tencent.com", "Tencent Cloud Mirror"),

            // CDN mirrors
            ("cloudfront.net", "CloudFront CDN"),
            ("fastly.net", "Fastly CDN"),
            ("azureedge.net", "Azure CDN"),

            // University mirrors
            ("mirror.mit.edu", "MIT Mirror"),
            ("mirrors.kernel.org", "Kernel.org Mirror"),
        ];

        for (pattern, name) in mirror_patterns {
            if url.contains(pattern) {
                // Try to detect what it's mirroring
                let known_repos = get_known_repositories();
                for known in known_repos {
                    for known_pattern in known.patterns {
                        if url.contains(known_pattern) ||
                           url.contains(&known.name) {
                            return Some(format!("{} of {}", name, known.vendor));
                        }
                    }
                }
                return Some(name.to_string());
            }
        }

        None
    }

    /// Get repository type for OS
    fn get_repo_type_for_os(&self, os: &OsType) -> RepositoryType {
        match os {
            OsType::Debian | OsType::Ubuntu => RepositoryType::Apt,
            OsType::Fedora => RepositoryType::Dnf,
            OsType::RedHat | OsType::CentOS if self.has_command("dnf") => RepositoryType::Dnf,
            OsType::RedHat | OsType::CentOS => RepositoryType::Yum,
            OsType::Arch => RepositoryType::Pacman,
            OsType::OpenSuse => RepositoryType::Zypper,
            _ => RepositoryType::Custom("Unknown".to_string()),
        }
    }

    /// Detect OS type
    fn detect_os_type(&self) -> Option<OsType> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
                let os_release_lower = os_release.to_lowercase();

                if os_release_lower.contains("ubuntu") {
                    return Some(OsType::Ubuntu);
                } else if os_release_lower.contains("debian") {
                    return Some(OsType::Debian);
                } else if os_release_lower.contains("fedora") {
                    return Some(OsType::Fedora);
                } else if os_release_lower.contains("centos") {
                    return Some(OsType::CentOS);
                } else if os_release_lower.contains("red hat") ||
                          os_release_lower.contains("rhel") {
                    return Some(OsType::RedHat);
                } else if os_release_lower.contains("arch") {
                    return Some(OsType::Arch);
                } else if os_release_lower.contains("opensuse") ||
                          os_release_lower.contains("suse") {
                    return Some(OsType::OpenSuse);
                }
            }
        }

        None
    }

    /// Get OS codename (for Debian/Ubuntu)
    fn get_os_codename(&self) -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
                for line in os_release.lines() {
                    if line.starts_with("VERSION_CODENAME=") {
                        let codename = line.trim_start_matches("VERSION_CODENAME=")
                            .trim_matches('"');
                        return Some(codename.to_string());
                    }
                }
            }

            // Try lsb_release
            if let Ok(output) = std::process::Command::new("lsb_release")
                .arg("-cs")
                .output() {
                let codename = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                if !codename.is_empty() {
                    return Some(codename);
                }
            }
        }

        None
    }

    /// Detect system architecture
    fn detect_architecture(&self) -> String {
        #[cfg(target_arch = "x86_64")]
        return "amd64".to_string();

        #[cfg(target_arch = "aarch64")]
        return "arm64".to_string();

        #[cfg(target_arch = "x86")]
        return "i386".to_string();

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "x86")))]
        return "unknown".to_string();
    }

    /// Check if a command exists
    fn has_command(&self, cmd: &str) -> bool {
        std::process::Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OsType {
    Ubuntu,
    Debian,
    Fedora,
    RedHat,
    CentOS,
    Arch,
    OpenSuse,
    Other,
}