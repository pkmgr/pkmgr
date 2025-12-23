use anyhow::{Context, Result};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub mod manager;
pub mod distributions;
pub mod verification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsoDistribution {
    pub name: String,
    pub display_name: String,
    pub category: DistributionCategory,
    pub versions: Vec<IsoVersion>,
    pub homepage: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionCategory {
    Linux,
    Security,
    Server,
    BSD,
    Utility,
    Windows,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsoVersion {
    pub version: String,
    pub codename: Option<String>,
    pub release_date: Option<String>,
    pub is_lts: bool,
    pub is_current: bool,
    pub architectures: Vec<Architecture>,
    pub flavors: Vec<String>, // Desktop environments or editions
    pub download_urls: HashMap<String, String>, // arch -> url
    pub checksum_urls: HashMap<String, String>, // arch -> checksum url
    pub signature_urls: HashMap<String, String>, // arch -> signature url
    pub size_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Architecture {
    X86_64,
    Aarch64,
    Armv7,
    I686,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::X86_64 => write!(f, "x86_64"),
            Architecture::Aarch64 => write!(f, "aarch64"),
            Architecture::Armv7 => write!(f, "armv7"),
            Architecture::I686 => write!(f, "i686"),
        }
    }
}

impl std::fmt::Display for DistributionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistributionCategory::Linux => write!(f, "Linux"),
            DistributionCategory::Security => write!(f, "Security/Penetration Testing"),
            DistributionCategory::Server => write!(f, "Server/Enterprise"),
            DistributionCategory::BSD => write!(f, "BSD Systems"),
            DistributionCategory::Utility => write!(f, "Utility/Rescue Tools"),
            DistributionCategory::Windows => write!(f, "Windows"),
            DistributionCategory::Other => write!(f, "Other Operating Systems"),
        }
    }
}