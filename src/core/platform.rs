use std::env;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Linux,
    MacOs,
    Windows,
    FreeBsd,
    OpenBsd,
    NetBsd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Architecture {
    X86_64,
    Aarch64,
    Armv7,
    I686,
    Ppc64le,
    S390x,
    Riscv64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Pacman,
    Zypper,
    Apk,
    Emerge,
    Xbps,
    Pkg,
    PkgAdd,
    Pkgin,
    Homebrew,
    MacPorts,
    Winget,
    Chocolatey,
    Scoop,
}

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub platform: Platform,
    pub architecture: Architecture,
    pub package_managers: Vec<PackageManager>,
    pub distribution: Option<String>,
    pub version: Option<String>,
}

impl Platform {
    pub fn detect() -> Result<PlatformInfo> {
        Ok(PlatformInfo::detect())
    }
}

impl PlatformInfo {
    pub fn detect() -> Self {
        let platform = Self::detect_platform();
        let architecture = Self::detect_architecture();
        let package_managers = Self::detect_package_managers(&platform);
        let (distribution, version) = Self::detect_distribution(&platform);

        Self {
            platform,
            architecture,
            package_managers,
            distribution,
            version,
        }
    }

    /// Async version of detect for compatibility
    pub async fn detect_async() -> Result<Self> {
        Ok(Self::detect())
    }

    /// Get the primary (first) package manager
    pub fn primary_package_manager(&self) -> Option<&PackageManager> {
        self.package_managers.first()
    }

    /// Get the operating system name
    pub fn os(&self) -> String {
        match self.platform {
            Platform::Linux => "Linux".to_string(),
            Platform::MacOs => "macOS".to_string(),
            Platform::Windows => "Windows".to_string(),
            Platform::FreeBsd => "FreeBSD".to_string(),
            Platform::OpenBsd => "OpenBSD".to_string(),
            Platform::NetBsd => "NetBSD".to_string(),
        }
    }

    fn detect_platform() -> Platform {
        match env::consts::OS {
            "linux" => Platform::Linux,
            "macos" => Platform::MacOs,
            "windows" => Platform::Windows,
            "freebsd" => Platform::FreeBsd,
            "openbsd" => Platform::OpenBsd,
            "netbsd" => Platform::NetBsd,
            _ => Platform::Linux, // Default fallback
        }
    }

    fn detect_architecture() -> Architecture {
        match env::consts::ARCH {
            "x86_64" => Architecture::X86_64,
            "aarch64" => Architecture::Aarch64,
            "armv7" => Architecture::Armv7,
            "x86" => Architecture::I686,
            "powerpc64" => Architecture::Ppc64le,
            "s390x" => Architecture::S390x,
            "riscv64" => Architecture::Riscv64,
            _ => Architecture::X86_64, // Default fallback
        }
    }

    fn detect_package_managers(platform: &Platform) -> Vec<PackageManager> {
        match platform {
            Platform::Linux => Self::detect_linux_package_managers(),
            Platform::MacOs => Self::detect_macos_package_managers(),
            Platform::Windows => Self::detect_windows_package_managers(),
            Platform::FreeBsd => vec![PackageManager::Pkg],
            Platform::OpenBsd => vec![PackageManager::PkgAdd],
            Platform::NetBsd => vec![PackageManager::Pkgin],
        }
    }

    fn detect_linux_package_managers() -> Vec<PackageManager> {
        let mut managers = Vec::new();

        // Check for package managers in order of preference
        if Self::command_exists("apt") {
            managers.push(PackageManager::Apt);
        }
        if Self::command_exists("dnf") {
            managers.push(PackageManager::Dnf);
        }
        if Self::command_exists("yum") && !managers.contains(&PackageManager::Dnf) {
            managers.push(PackageManager::Yum);
        }
        if Self::command_exists("pacman") {
            managers.push(PackageManager::Pacman);
        }
        if Self::command_exists("zypper") {
            managers.push(PackageManager::Zypper);
        }
        if Self::command_exists("apk") {
            managers.push(PackageManager::Apk);
        }
        if Self::command_exists("emerge") {
            managers.push(PackageManager::Emerge);
        }
        if Self::command_exists("xbps-install") {
            managers.push(PackageManager::Xbps);
        }

        managers
    }

    fn detect_macos_package_managers() -> Vec<PackageManager> {
        let mut managers = Vec::new();

        if Self::command_exists("brew") {
            managers.push(PackageManager::Homebrew);
        }
        if Self::command_exists("port") {
            managers.push(PackageManager::MacPorts);
        }

        managers
    }

    fn detect_windows_package_managers() -> Vec<PackageManager> {
        let mut managers = Vec::new();

        if Self::command_exists("winget") {
            managers.push(PackageManager::Winget);
        }
        if Self::command_exists("choco") {
            managers.push(PackageManager::Chocolatey);
        }
        if Self::command_exists("scoop") {
            managers.push(PackageManager::Scoop);
        }

        managers
    }

    fn detect_distribution(platform: &Platform) -> (Option<String>, Option<String>) {
        if *platform != Platform::Linux {
            return (None, None);
        }

        // Try to read /etc/os-release
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            let mut name = None;
            let mut version = None;

            for line in content.lines() {
                if line.starts_with("ID=") {
                    name = Some(line.strip_prefix("ID=").unwrap_or("").trim_matches('"').to_string());
                } else if line.starts_with("VERSION_ID=") {
                    version = Some(line.strip_prefix("VERSION_ID=").unwrap_or("").trim_matches('"').to_string());
                }
            }

            if name.is_some() {
                return (name, version);
            }
        }

        // Fallback methods for older systems
        if let Ok(content) = std::fs::read_to_string("/etc/redhat-release") {
            if content.contains("CentOS") {
                return (Some("centos".to_string()), None);
            } else if content.contains("Red Hat") {
                return (Some("rhel".to_string()), None);
            } else if content.contains("Fedora") {
                return (Some("fedora".to_string()), None);
            }
        }

        if std::path::Path::new("/etc/debian_version").exists() {
            return (Some("debian".to_string()), None);
        }

        if std::path::Path::new("/etc/arch-release").exists() {
            return (Some("arch".to_string()), None);
        }

        (None, None)
    }

    fn command_exists(command: &str) -> bool {
        std::process::Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn is_arch_linux(&self) -> bool {
        self.distribution.as_ref().map_or(false, |d| d == "arch")
    }

    pub fn is_debian_based(&self) -> bool {
        self.distribution.as_ref().map_or(false, |d| {
            matches!(d.as_str(), "debian" | "ubuntu")
        })
    }

    pub fn is_rhel_based(&self) -> bool {
        self.distribution.as_ref().map_or(false, |d| {
            matches!(d.as_str(), "rhel" | "centos" | "fedora" | "rocky" | "almalinux")
        })
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOs => write!(f, "macOS"),
            Platform::Windows => write!(f, "Windows"),
            Platform::FreeBsd => write!(f, "FreeBSD"),
            Platform::OpenBsd => write!(f, "OpenBSD"),
            Platform::NetBsd => write!(f, "NetBSD"),
        }
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::X86_64 => write!(f, "x86_64"),
            Architecture::Aarch64 => write!(f, "aarch64"),
            Architecture::Armv7 => write!(f, "armv7"),
            Architecture::I686 => write!(f, "i686"),
            Architecture::Ppc64le => write!(f, "ppc64le"),
            Architecture::S390x => write!(f, "s390x"),
            Architecture::Riscv64 => write!(f, "riscv64"),
        }
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Apt => write!(f, "apt"),
            PackageManager::Dnf => write!(f, "dnf"),
            PackageManager::Yum => write!(f, "yum"),
            PackageManager::Pacman => write!(f, "pacman"),
            PackageManager::Zypper => write!(f, "zypper"),
            PackageManager::Apk => write!(f, "apk"),
            PackageManager::Emerge => write!(f, "emerge"),
            PackageManager::Xbps => write!(f, "xbps"),
            PackageManager::Pkg => write!(f, "pkg"),
            PackageManager::PkgAdd => write!(f, "pkg_add"),
            PackageManager::Pkgin => write!(f, "pkgin"),
            PackageManager::Homebrew => write!(f, "brew"),
            PackageManager::MacPorts => write!(f, "port"),
            PackageManager::Winget => write!(f, "winget"),
            PackageManager::Chocolatey => write!(f, "choco"),
            PackageManager::Scoop => write!(f, "scoop"),
        }
    }
}