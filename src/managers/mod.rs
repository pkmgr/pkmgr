use anyhow::Result;
use crate::core::{
    platform::{PlatformInfo, PackageManager as PlatformPackageManager},
    traits::PackageManager,
};

pub mod apt;
pub mod dnf;
pub mod pacman;
pub mod homebrew;
pub mod winget;
pub mod chocolatey;
pub mod scoop;

pub struct PackageManagerFactory;

impl PackageManagerFactory {
    pub fn create(platform_info: &PlatformInfo) -> Result<Box<dyn PackageManager>> {
        let primary_manager = platform_info.primary_package_manager()
            .ok_or_else(|| anyhow::anyhow!("No package manager detected"))?;

        match primary_manager {
            PlatformPackageManager::Apt => Ok(Box::new(apt::AptManager::new())),
            PlatformPackageManager::Dnf => Ok(Box::new(dnf::DnfManager::new())),
            PlatformPackageManager::Pacman => Ok(Box::new(pacman::PacmanManager::new())),
            PlatformPackageManager::Homebrew => Ok(Box::new(homebrew::HomebrewManager::new())),
            PlatformPackageManager::Winget => Ok(Box::new(winget::WingetManager::new())),
            PlatformPackageManager::Chocolatey => Ok(Box::new(chocolatey::ChocolateyManager::new())),
            PlatformPackageManager::Scoop => Ok(Box::new(scoop::ScoopManager::new())),
            _ => Err(anyhow::anyhow!("Unsupported package manager: {}", primary_manager)),
        }
    }
}