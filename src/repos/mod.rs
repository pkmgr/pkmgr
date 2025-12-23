use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod manager;
pub mod gpg;
pub mod detector;
pub mod config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub repo_type: RepositoryType,
    pub enabled: bool,
    pub priority: u32,
    pub gpg_key: Option<GpgKeyInfo>,
    pub architectures: Vec<String>,
    pub components: Vec<String>,
    pub suites: Vec<String>,
    pub metadata: RepositoryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepositoryType {
    Apt,
    Yum,
    Dnf,
    Zypper,
    Pacman,
    Aur,
    Homebrew,
    Winget,
    Chocolatey,
    Scoop,
    Flatpak,
    Snap,
    AppImage,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpgKeyInfo {
    pub fingerprint: String,
    pub key_id: String,
    pub key_server: Option<String>,
    pub key_url: Option<String>,
    pub trusted: bool,
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
    pub last_refreshed: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMetadata {
    pub vendor: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub package_count: Option<usize>,
    pub size_bytes: Option<u64>,
    pub mirror_of: Option<String>,
    pub is_official: bool,
    pub is_verified: bool,
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrustLevel {
    Official,   // OS vendor repos
    Verified,   // Known vendors (Docker, Microsoft, etc.)
    Community,  // PPAs, AUR, COPR
    Corporate,  // Internal mirrors
    Unknown,    // User-added
}

impl Repository {
    pub fn new(name: String, url: String, repo_type: RepositoryType) -> Self {
        Self {
            name,
            url,
            repo_type,
            enabled: true,
            priority: 100,
            gpg_key: None,
            architectures: vec![],
            components: vec![],
            suites: vec![],
            metadata: RepositoryMetadata::default(),
        }
    }

    pub fn with_gpg_key(mut self, key: GpgKeyInfo) -> Self {
        self.gpg_key = Some(key);
        self
    }

    pub fn with_trust_level(mut self, level: TrustLevel) -> Self {
        self.metadata.trust_level = level;
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ref key) = self.gpg_key {
            if let Some(expires) = key.expires {
                return expires < chrono::Utc::now();
            }
        }
        false
    }

    pub fn needs_refresh(&self) -> bool {
        if let Some(ref key) = self.gpg_key {
            if let Some(last_refreshed) = key.last_refreshed {
                // Refresh if older than 30 days
                let days_30 = chrono::Duration::days(30);
                return chrono::Utc::now() - last_refreshed > days_30;
            }
            // Never refreshed
            return true;
        }
        false
    }
}

impl Default for RepositoryMetadata {
    fn default() -> Self {
        Self {
            vendor: None,
            description: None,
            homepage: None,
            last_updated: None,
            package_count: None,
            size_bytes: None,
            mirror_of: None,
            is_official: false,
            is_verified: false,
            trust_level: TrustLevel::Unknown,
        }
    }
}

impl std::fmt::Display for RepositoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryType::Apt => write!(f, "APT"),
            RepositoryType::Yum => write!(f, "YUM"),
            RepositoryType::Dnf => write!(f, "DNF"),
            RepositoryType::Zypper => write!(f, "Zypper"),
            RepositoryType::Pacman => write!(f, "Pacman"),
            RepositoryType::Aur => write!(f, "AUR"),
            RepositoryType::Homebrew => write!(f, "Homebrew"),
            RepositoryType::Winget => write!(f, "Winget"),
            RepositoryType::Chocolatey => write!(f, "Chocolatey"),
            RepositoryType::Scoop => write!(f, "Scoop"),
            RepositoryType::Flatpak => write!(f, "Flatpak"),
            RepositoryType::Snap => write!(f, "Snap"),
            RepositoryType::AppImage => write!(f, "AppImage"),
            RepositoryType::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustLevel::Official => write!(f, "Official"),
            TrustLevel::Verified => write!(f, "Verified"),
            TrustLevel::Community => write!(f, "Community"),
            TrustLevel::Corporate => write!(f, "Corporate"),
            TrustLevel::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Known repository patterns for auto-detection
#[derive(Debug, Clone)]
pub struct KnownRepository {
    pub name: &'static str,
    pub patterns: Vec<&'static str>,
    pub gpg_fingerprint: Option<&'static str>,
    pub gpg_key_url: Option<&'static str>,
    pub trust_level: TrustLevel,
    pub vendor: &'static str,
    pub description: &'static str,
}

/// Get all known repositories
pub fn get_known_repositories() -> Vec<KnownRepository> {
    vec![
        // Docker
        KnownRepository {
            name: "docker",
            patterns: vec!["docker.com", "docker.io", "download.docker.com"],
            gpg_fingerprint: Some("9DC858229FC7DD38854AE2D88D81803C0EBFCD88"),
            gpg_key_url: Some("https://download.docker.com/linux/ubuntu/gpg"),
            trust_level: TrustLevel::Verified,
            vendor: "Docker Inc.",
            description: "Docker CE repository",
        },

        // PostgreSQL
        KnownRepository {
            name: "postgresql",
            patterns: vec!["postgresql.org", "apt.postgresql.org"],
            gpg_fingerprint: Some("A4D3BFB9ACCC4CF8"),
            gpg_key_url: Some("https://www.postgresql.org/media/keys/ACCC4CF8.asc"),
            trust_level: TrustLevel::Verified,
            vendor: "PostgreSQL Global Development Group",
            description: "PostgreSQL PGDG repository",
        },

        // MongoDB
        KnownRepository {
            name: "mongodb",
            patterns: vec!["mongodb.org", "repo.mongodb.org"],
            gpg_fingerprint: Some("B00A0BD1E2C63C11"),
            gpg_key_url: Some("https://www.mongodb.org/static/pgp/server-6.0.asc"),
            trust_level: TrustLevel::Verified,
            vendor: "MongoDB Inc.",
            description: "MongoDB official repository",
        },

        // Microsoft
        KnownRepository {
            name: "microsoft",
            patterns: vec!["packages.microsoft.com"],
            gpg_fingerprint: Some("BC528686B50D79E339D3721CEB3E94ADBE1229CF"),
            gpg_key_url: Some("https://packages.microsoft.com/keys/microsoft.asc"),
            trust_level: TrustLevel::Verified,
            vendor: "Microsoft Corporation",
            description: "Microsoft package repository",
        },

        // HashiCorp
        KnownRepository {
            name: "hashicorp",
            patterns: vec!["hashicorp.com", "releases.hashicorp.com"],
            gpg_fingerprint: Some("E8A032E094D8EB4EA189D270DA418C88A3219F7B"),
            gpg_key_url: Some("https://apt.releases.hashicorp.com/gpg"),
            trust_level: TrustLevel::Verified,
            vendor: "HashiCorp",
            description: "HashiCorp official repository",
        },

        // Kubernetes
        KnownRepository {
            name: "kubernetes",
            patterns: vec!["kubernetes.io", "packages.cloud.google.com/apt", "packages.cloud.google.com/yum"],
            gpg_fingerprint: Some("54A647F9048D5688D7DA2ABE6A030B21BA07F4FB"),
            gpg_key_url: Some("https://packages.cloud.google.com/apt/doc/apt-key.gpg"),
            trust_level: TrustLevel::Verified,
            vendor: "Kubernetes",
            description: "Kubernetes official repository",
        },

        // Elastic
        KnownRepository {
            name: "elastic",
            patterns: vec!["elastic.co", "artifacts.elastic.co"],
            gpg_fingerprint: Some("4609 5ACC 8548 582C 1A26 99A9 D27D 666C D88E 42B4"),
            gpg_key_url: Some("https://artifacts.elastic.co/GPG-KEY-elasticsearch"),
            trust_level: TrustLevel::Verified,
            vendor: "Elastic",
            description: "Elastic Stack repository",
        },

        // Grafana
        KnownRepository {
            name: "grafana",
            patterns: vec!["grafana.com", "packages.grafana.com", "apt.grafana.com"],
            gpg_fingerprint: Some("4E40DDF6D76E284A4A6780E48C8C34C524098CB6"),
            gpg_key_url: Some("https://packages.grafana.com/gpg.key"),
            trust_level: TrustLevel::Verified,
            vendor: "Grafana Labs",
            description: "Grafana official repository",
        },

        // Node.js
        KnownRepository {
            name: "nodesource",
            patterns: vec!["nodesource.com", "deb.nodesource.com", "rpm.nodesource.com"],
            gpg_fingerprint: Some("9FD3B784BC1C6FC31A8A0A1C1655A0AB68576280"),
            gpg_key_url: Some("https://deb.nodesource.com/gpgkey/nodesource.gpg.key"),
            trust_level: TrustLevel::Verified,
            vendor: "NodeSource",
            description: "Node.js official repository",
        },

        // Yarn
        KnownRepository {
            name: "yarn",
            patterns: vec!["yarnpkg.com", "dl.yarnpkg.com"],
            gpg_fingerprint: Some("72ECF46A56B4AD39C907BBB71646B01B86E50310"),
            gpg_key_url: Some("https://dl.yarnpkg.com/debian/pubkey.gpg"),
            trust_level: TrustLevel::Verified,
            vendor: "Yarn",
            description: "Yarn package manager repository",
        },

        // PHP Remi
        KnownRepository {
            name: "remi",
            patterns: vec!["remirepo.net", "rpms.remirepo.net"],
            gpg_fingerprint: Some("5F11735A0C0F176702C1E1F7B6B8244530F3A2AC"),
            gpg_key_url: Some("https://rpms.remirepo.net/RPM-GPG-KEY-remi"),
            trust_level: TrustLevel::Verified,
            vendor: "Remi Collet",
            description: "Remi's RPM repository for PHP",
        },

        // EPEL
        KnownRepository {
            name: "epel",
            patterns: vec!["fedoraproject.org/epel", "download.fedoraproject.org/pub/epel"],
            gpg_fingerprint: Some("8483C65D8F7EBD46B18E0F8C8F1F2AFA1A8F4B31"),
            gpg_key_url: Some("https://dl.fedoraproject.org/pub/epel/RPM-GPG-KEY-EPEL-9"),
            trust_level: TrustLevel::Official,
            vendor: "Fedora Project",
            description: "Extra Packages for Enterprise Linux",
        },
    ]
}