use super::*;
use std::collections::HashMap;

/// Get all supported distributions as specified in CLAUDE.md
pub fn get_all_distributions() -> Vec<IsoDistribution> {
    let mut distributions = Vec::new();

    // Linux Distributions
    distributions.push(ubuntu());
    distributions.push(debian());
    distributions.push(fedora());
    distributions.push(arch_linux());
    distributions.push(manjaro());
    distributions.push(opensuse());
    distributions.push(centos());
    distributions.push(rocky_linux());
    distributions.push(alma_linux());
    distributions.push(alpine_linux());
    distributions.push(void_linux());
    distributions.push(gentoo());
    distributions.push(nixos());

    // Security/Penetration Testing
    distributions.push(kali_linux());
    distributions.push(parrot_security());
    distributions.push(black_arch());
    distributions.push(tails());

    // Server/Enterprise
    distributions.push(proxmox());
    distributions.push(truenas());
    distributions.push(pfsense());
    distributions.push(opnsense());
    distributions.push(vyos());

    // BSD Systems
    distributions.push(freebsd());
    distributions.push(openbsd());
    distributions.push(netbsd());

    // Utility/Rescue Tools
    distributions.push(gparted_live());
    distributions.push(clonezilla());
    distributions.push(system_rescue());
    distributions.push(memtest86());
    distributions.push(hirens_boot_cd());
    distributions.push(ultimate_boot_cd());

    distributions
}

fn ubuntu() -> IsoDistribution {
    IsoDistribution {
        name: "ubuntu".to_string(),
        display_name: "Ubuntu".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://ubuntu.com".to_string(),
        description: "Popular Linux distribution based on Debian".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["desktop".to_string(), "server".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://releases.ubuntu.com/22.04.3/ubuntu-22.04.3-desktop-amd64.iso".to_string()),
                    ("x86_64-server".to_string(), "https://releases.ubuntu.com/22.04.3/ubuntu-22.04.3-live-server-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://releases.ubuntu.com/22.04.3/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://releases.ubuntu.com/22.04.3/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 4700,
            },
            IsoVersion {
                version: "20.04.6".to_string(),
                codename: Some("Focal Fossa".to_string()),
                release_date: Some("2023-03-23".to_string()),
                is_lts: true,
                is_current: false,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string(), "server".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3800,
            },
        ],
    }
}

fn debian() -> IsoDistribution {
    IsoDistribution {
        name: "debian".to_string(),
        display_name: "Debian".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://www.debian.org".to_string(),
        description: "The universal operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "12.2.0".to_string(),
                codename: Some("Bookworm".to_string()),
                release_date: Some("2023-10-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64, Architecture::I686],
                flavors: vec!["netinst".to_string(), "dvd".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-netinst".to_string(), "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-12.2.0-amd64-netinst.iso".to_string()),
                ]),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 659,
            },
        ],
    }
}

fn fedora() -> IsoDistribution {
    IsoDistribution {
        name: "fedora".to_string(),
        display_name: "Fedora".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://getfedora.org".to_string(),
        description: "Cutting-edge Linux distribution sponsored by Red Hat".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["workstation".to_string(), "server".to_string(), "spins".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-workstation".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Workstation/x86_64/iso/Fedora-Workstation-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2100,
            },
        ],
    }
}

fn arch_linux() -> IsoDistribution {
    IsoDistribution {
        name: "arch".to_string(),
        display_name: "Arch Linux".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://archlinux.org".to_string(),
        description: "Rolling release distribution for experienced users".to_string(),
        versions: vec![
            IsoVersion {
                version: "2023.11.01".to_string(),
                codename: None,
                release_date: Some("2023-11-01".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::from([
                    ("x86_64".to_string(), "https://geo.mirror.pkgbuild.com/iso/latest/archlinux-x86_64.iso".to_string()),
                ]),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 800,
            },
        ],
    }
}

fn manjaro() -> IsoDistribution {
    IsoDistribution {
        name: "manjaro".to_string(),
        display_name: "Manjaro".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://manjaro.org".to_string(),
        description: "User-friendly Arch-based distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "23.1".to_string(),
                codename: Some("Vulcan".to_string()),
                release_date: Some("2023-10-15".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["xfce".to_string(), "kde".to_string(), "gnome".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3200,
            },
        ],
    }
}

fn opensuse() -> IsoDistribution {
    IsoDistribution {
        name: "opensuse".to_string(),
        display_name: "openSUSE".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://www.opensuse.org".to_string(),
        description: "Enterprise-grade Linux with YaST configuration tool".to_string(),
        versions: vec![
            IsoVersion {
                version: "Tumbleweed".to_string(),
                codename: None,
                release_date: None,
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["kde".to_string(), "gnome".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 4500,
            },
            IsoVersion {
                version: "Leap 15.5".to_string(),
                codename: None,
                release_date: Some("2023-06-07".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["kde".to_string(), "gnome".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 4300,
            },
        ],
    }
}

fn centos() -> IsoDistribution {
    IsoDistribution {
        name: "centos".to_string(),
        display_name: "CentOS".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://www.centos.org".to_string(),
        description: "Community Enterprise Operating System".to_string(),
        versions: vec![
            IsoVersion {
                version: "Stream 9".to_string(),
                codename: None,
                release_date: None,
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["dvd".to_string(), "boot".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 10000,
            },
        ],
    }
}

fn rocky_linux() -> IsoDistribution {
    IsoDistribution {
        name: "rocky".to_string(),
        display_name: "Rocky Linux".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://rockylinux.org".to_string(),
        description: "Enterprise Linux, community-driven".to_string(),
        versions: vec![
            IsoVersion {
                version: "9.3".to_string(),
                codename: Some("Blue Onyx".to_string()),
                release_date: Some("2023-11-20".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["dvd".to_string(), "minimal".to_string(), "boot".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 10300,
            },
        ],
    }
}

fn alma_linux() -> IsoDistribution {
    IsoDistribution {
        name: "almalinux".to_string(),
        display_name: "AlmaLinux".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://almalinux.org".to_string(),
        description: "Enterprise Linux distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "9.3".to_string(),
                codename: Some("Shamrock Pampas Cat".to_string()),
                release_date: Some("2023-11-13".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["dvd".to_string(), "minimal".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 10000,
            },
        ],
    }
}

fn alpine_linux() -> IsoDistribution {
    IsoDistribution {
        name: "alpine".to_string(),
        display_name: "Alpine Linux".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://alpinelinux.org".to_string(),
        description: "Security-oriented, lightweight Linux".to_string(),
        versions: vec![
            IsoVersion {
                version: "3.18.4".to_string(),
                codename: None,
                release_date: Some("2023-09-28".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64, Architecture::Armv7],
                flavors: vec!["standard".to_string(), "extended".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 180,
            },
        ],
    }
}

fn void_linux() -> IsoDistribution {
    IsoDistribution {
        name: "void".to_string(),
        display_name: "Void Linux".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://voidlinux.org".to_string(),
        description: "Independent Linux distribution with runit init".to_string(),
        versions: vec![
            IsoVersion {
                version: "20230628".to_string(),
                codename: None,
                release_date: Some("2023-06-28".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["xfce".to_string(), "base".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 1100,
            },
        ],
    }
}

fn gentoo() -> IsoDistribution {
    IsoDistribution {
        name: "gentoo".to_string(),
        display_name: "Gentoo".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://www.gentoo.org".to_string(),
        description: "Source-based meta-distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "20231119".to_string(),
                codename: None,
                release_date: Some("2023-11-19".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["stage3".to_string(), "livegui".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2800,
            },
        ],
    }
}

fn nixos() -> IsoDistribution {
    IsoDistribution {
        name: "nixos".to_string(),
        display_name: "NixOS".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://nixos.org".to_string(),
        description: "Declarative Linux distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "23.05".to_string(),
                codename: Some("Stoat".to_string()),
                release_date: Some("2023-05-31".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["gnome".to_string(), "plasma".to_string(), "minimal".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2300,
            },
        ],
    }
}

// Security distributions
fn kali_linux() -> IsoDistribution {
    IsoDistribution {
        name: "kali".to_string(),
        display_name: "Kali Linux".to_string(),
        category: DistributionCategory::Security,
        homepage: "https://www.kali.org".to_string(),
        description: "Penetration testing and security auditing".to_string(),
        versions: vec![
            IsoVersion {
                version: "2023.3".to_string(),
                codename: None,
                release_date: Some("2023-08-24".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["live".to_string(), "installer".to_string(), "netinstaller".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-live".to_string(), "https://cdimage.kali.org/kali-2023.3/kali-linux-2023.3-live-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 4000,
            },
        ],
    }
}

fn parrot_security() -> IsoDistribution {
    IsoDistribution {
        name: "parrot".to_string(),
        display_name: "Parrot Security".to_string(),
        category: DistributionCategory::Security,
        homepage: "https://parrotlinux.org".to_string(),
        description: "Security and privacy focused distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "5.3".to_string(),
                codename: Some("Electro Ara".to_string()),
                release_date: Some("2023-06-14".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["security".to_string(), "home".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 4900,
            },
        ],
    }
}

fn black_arch() -> IsoDistribution {
    IsoDistribution {
        name: "blackarch".to_string(),
        display_name: "BlackArch Linux".to_string(),
        category: DistributionCategory::Security,
        homepage: "https://blackarch.org".to_string(),
        description: "Arch-based penetration testing distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "2023.04.01".to_string(),
                codename: None,
                release_date: Some("2023-04-01".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["full".to_string(), "netinstall".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 22000,
            },
        ],
    }
}

fn tails() -> IsoDistribution {
    IsoDistribution {
        name: "tails".to_string(),
        display_name: "Tails".to_string(),
        category: DistributionCategory::Security,
        homepage: "https://tails.boum.org".to_string(),
        description: "Privacy and anonymity focused live system".to_string(),
        versions: vec![
            IsoVersion {
                version: "5.19".to_string(),
                codename: None,
                release_date: Some("2023-10-31".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 1300,
            },
        ],
    }
}

// Server distributions
fn proxmox() -> IsoDistribution {
    IsoDistribution {
        name: "proxmox".to_string(),
        display_name: "Proxmox VE".to_string(),
        category: DistributionCategory::Server,
        homepage: "https://www.proxmox.com".to_string(),
        description: "Virtualization management platform".to_string(),
        versions: vec![
            IsoVersion {
                version: "8.0".to_string(),
                codename: None,
                release_date: Some("2023-06-22".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 1200,
            },
        ],
    }
}

fn truenas() -> IsoDistribution {
    IsoDistribution {
        name: "truenas".to_string(),
        display_name: "TrueNAS".to_string(),
        category: DistributionCategory::Server,
        homepage: "https://www.truenas.com".to_string(),
        description: "Network attached storage solution".to_string(),
        versions: vec![
            IsoVersion {
                version: "13.0-U5.3".to_string(),
                codename: None,
                release_date: Some("2023-08-01".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["core".to_string(), "scale".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 1000,
            },
        ],
    }
}

fn pfsense() -> IsoDistribution {
    IsoDistribution {
        name: "pfsense".to_string(),
        display_name: "pfSense".to_string(),
        category: DistributionCategory::Server,
        homepage: "https://www.pfsense.org".to_string(),
        description: "Firewall and router platform".to_string(),
        versions: vec![
            IsoVersion {
                version: "2.7.0".to_string(),
                codename: None,
                release_date: Some("2023-06-26".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["ce".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 750,
            },
        ],
    }
}

fn opnsense() -> IsoDistribution {
    IsoDistribution {
        name: "opnsense".to_string(),
        display_name: "OPNsense".to_string(),
        category: DistributionCategory::Server,
        homepage: "https://opnsense.org".to_string(),
        description: "Open source firewall and routing platform".to_string(),
        versions: vec![
            IsoVersion {
                version: "23.7".to_string(),
                codename: Some("Restless Roadrunner".to_string()),
                release_date: Some("2023-07-31".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["dvd".to_string(), "vga".to_string(), "serial".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 700,
            },
        ],
    }
}

fn vyos() -> IsoDistribution {
    IsoDistribution {
        name: "vyos".to_string(),
        display_name: "VyOS".to_string(),
        category: DistributionCategory::Server,
        homepage: "https://vyos.io".to_string(),
        description: "Network operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "1.4".to_string(),
                codename: Some("Sagitta".to_string()),
                release_date: Some("2023-09-09".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["rolling".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 450,
            },
        ],
    }
}

// BSD Systems
fn freebsd() -> IsoDistribution {
    IsoDistribution {
        name: "freebsd".to_string(),
        display_name: "FreeBSD".to_string(),
        category: DistributionCategory::BSD,
        homepage: "https://www.freebsd.org".to_string(),
        description: "Advanced BSD operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "14.0".to_string(),
                codename: None,
                release_date: Some("2023-11-20".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["disc1".to_string(), "dvd1".to_string(), "memstick".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 1000,
            },
        ],
    }
}

fn openbsd() -> IsoDistribution {
    IsoDistribution {
        name: "openbsd".to_string(),
        display_name: "OpenBSD".to_string(),
        category: DistributionCategory::BSD,
        homepage: "https://www.openbsd.org".to_string(),
        description: "Security-focused BSD operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "7.4".to_string(),
                codename: None,
                release_date: Some("2023-10-16".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["install".to_string(), "cd".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 600,
            },
        ],
    }
}

fn netbsd() -> IsoDistribution {
    IsoDistribution {
        name: "netbsd".to_string(),
        display_name: "NetBSD".to_string(),
        category: DistributionCategory::BSD,
        homepage: "https://www.netbsd.org".to_string(),
        description: "Portable BSD operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "10.0_RC1".to_string(),
                codename: None,
                release_date: Some("2023-11-28".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["install".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 500,
            },
        ],
    }
}

// Utility/Rescue Tools
fn gparted_live() -> IsoDistribution {
    IsoDistribution {
        name: "gparted".to_string(),
        display_name: "GParted Live".to_string(),
        category: DistributionCategory::Utility,
        homepage: "https://gparted.org".to_string(),
        description: "Partition editor live system".to_string(),
        versions: vec![
            IsoVersion {
                version: "1.5.0-6".to_string(),
                codename: None,
                release_date: Some("2023-10-09".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 500,
            },
        ],
    }
}

fn clonezilla() -> IsoDistribution {
    IsoDistribution {
        name: "clonezilla".to_string(),
        display_name: "Clonezilla Live".to_string(),
        category: DistributionCategory::Utility,
        homepage: "https://clonezilla.org".to_string(),
        description: "Disk cloning and imaging solution".to_string(),
        versions: vec![
            IsoVersion {
                version: "3.1.0-22".to_string(),
                codename: None,
                release_date: Some("2023-10-24".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["stable".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 400,
            },
        ],
    }
}

fn system_rescue() -> IsoDistribution {
    IsoDistribution {
        name: "systemrescue".to_string(),
        display_name: "SystemRescue".to_string(),
        category: DistributionCategory::Utility,
        homepage: "https://www.system-rescue.org".to_string(),
        description: "System rescue toolkit".to_string(),
        versions: vec![
            IsoVersion {
                version: "10.02".to_string(),
                codename: None,
                release_date: Some("2023-08-19".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 800,
            },
        ],
    }
}

fn memtest86() -> IsoDistribution {
    IsoDistribution {
        name: "memtest86".to_string(),
        display_name: "MemTest86+".to_string(),
        category: DistributionCategory::Utility,
        homepage: "https://www.memtest86.com".to_string(),
        description: "Memory testing tool".to_string(),
        versions: vec![
            IsoVersion {
                version: "6.20".to_string(),
                codename: None,
                release_date: Some("2023-05-15".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 10,
            },
        ],
    }
}

fn hirens_boot_cd() -> IsoDistribution {
    IsoDistribution {
        name: "hirens".to_string(),
        display_name: "Hiren's Boot CD".to_string(),
        category: DistributionCategory::Utility,
        homepage: "https://www.hirensbootcd.org".to_string(),
        description: "All-in-one boot disk utilities".to_string(),
        versions: vec![
            IsoVersion {
                version: "1.0.2".to_string(),
                codename: None,
                release_date: Some("2021-06-12".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["pe".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3000,
            },
        ],
    }
}

fn ultimate_boot_cd() -> IsoDistribution {
    IsoDistribution {
        name: "ubcd".to_string(),
        display_name: "Ultimate Boot CD".to_string(),
        category: DistributionCategory::Utility,
        homepage: "https://www.ultimatebootcd.com".to_string(),
        description: "Diagnostic tools compilation".to_string(),
        versions: vec![
            IsoVersion {
                version: "5.3.9".to_string(),
                codename: None,
                release_date: Some("2021-01-01".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["standard".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 700,
            },
        ],
    }
}