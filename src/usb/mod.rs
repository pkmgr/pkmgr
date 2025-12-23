use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

pub mod device;
pub mod wizard;
pub mod bootloader;
pub mod writer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub path: PathBuf,
    pub name: String,
    pub size_bytes: u64,
    pub size_display: String,
    pub filesystem: Option<String>,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub is_removable: bool,
    pub is_mounted: bool,
    pub mount_points: Vec<PathBuf>,
    pub partitions: Vec<UsbPartition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbPartition {
    pub path: PathBuf,
    pub number: u32,
    pub size_bytes: u64,
    pub filesystem: Option<String>,
    pub label: Option<String>,
    pub uuid: Option<String>,
    pub mount_point: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum UsbOperation {
    WriteSingle {
        iso_path: PathBuf,
        device: PathBuf,
    },
    CreateMultiBoot {
        device: PathBuf,
        initial_isos: Vec<PathBuf>,
    },
    AddToMultiBoot {
        device: PathBuf,
        iso_path: PathBuf,
    },
    RemoveFromMultiBoot {
        device: PathBuf,
        iso_name: String,
    },
    Erase {
        device: PathBuf,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiBootConfig {
    pub version: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
    pub bootloader: BootloaderType,
    pub entries: Vec<BootEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootloaderType {
    Grub2,
    Syslinux,
    Ventoy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootEntry {
    pub name: String,
    pub display_name: String,
    pub iso_path: String,
    pub category: String,
    pub version: String,
    pub architecture: String,
    pub boot_params: Vec<String>,
    pub added: chrono::DateTime<chrono::Utc>,
}

impl UsbDevice {
    pub fn size_gb(&self) -> f64 {
        self.size_bytes as f64 / 1_000_000_000.0
    }

    pub fn is_suitable_for_single_iso(&self) -> bool {
        self.is_removable && self.size_gb() >= 4.0
    }

    pub fn is_suitable_for_multi_boot(&self) -> bool {
        self.is_removable && self.size_gb() >= 16.0
    }

    pub fn format_size(&self) -> String {
        let gb = self.size_gb();
        if gb < 1.0 {
            format!("{:.0} MB", gb * 1000.0)
        } else {
            format!("{:.1} GB", gb)
        }
    }
}

/// Check if a device is safe to use for USB operations
pub fn is_device_safe(device_path: &Path) -> Result<bool> {
    // Never allow operations on these devices
    let forbidden_patterns = vec![
        "/dev/sda",     // Usually system disk
        "/dev/nvme",    // NVMe drives
        "/dev/md",      // RAID arrays
        "/dev/dm",      // Device mapper (LVM)
        "/dev/loop",    // Loop devices
    ];

    let path_str = device_path.to_string_lossy();
    for pattern in forbidden_patterns {
        if path_str.starts_with(pattern) {
            return Ok(false);
        }
    }

    // Additional safety checks will be in device.rs
    Ok(true)
}