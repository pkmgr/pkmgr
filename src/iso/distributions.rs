use super::*;
use std::collections::HashMap;

/// Get all supported distributions as specified in CLAUDE.md
/// Organized by category with proper subdirectory structure:
/// linux/desktop/, linux/server/, linux/security/, linux/utility/, linux/minimal/, linux/specialty/
/// windows/, bsd/, other/
pub fn get_all_distributions() -> Vec<IsoDistribution> {
    let mut distributions = Vec::new();

    // Linux Desktop Distributions (linux/desktop/)
    distributions.push(ubuntu());
    distributions.push(kubuntu());
    distributions.push(xubuntu());
    distributions.push(lubuntu());
    distributions.push(ubuntu_mate());
    distributions.push(ubuntu_budgie());
    distributions.push(ubuntu_studio());
    distributions.push(ubuntu_kylin());
    distributions.push(edubuntu());
    distributions.push(ubuntu_unity());
    distributions.push(pop_os());
    distributions.push(elementary_os());
    distributions.push(linux_mint());
    distributions.push(linux_mint_debian());
    distributions.push(zorin_os());
    distributions.push(debian());
    distributions.push(fedora());
    distributions.push(fedora_kde());
    distributions.push(fedora_xfce());
    distributions.push(fedora_lxde());
    distributions.push(fedora_mate());
    distributions.push(fedora_cinnamon());
    distributions.push(fedora_soas());
    distributions.push(opensuse());
    distributions.push(manjaro());
    distributions.push(manjaro_kde());
    distributions.push(manjaro_xfce());
    distributions.push(arch_linux());
    distributions.push(endeavouros());
    distributions.push(garuda_linux());
    distributions.push(mx_linux());
    distributions.push(antiX());
    distributions.push(puppy_linux());
    distributions.push(tiny_core());
    distributions.push(slax());
    distributions.push(porteus());
    distributions.push(peppermint_os());
    distributions.push(deepin());
    distributions.push(endless_os());
    distributions.push(solus());
    distributions.push(mageia());
    distributions.push(pclinuxos());
    distributions.push(sabayon());
    distributions.push(kaos());
    distributions.push(chakra());
    distributions.push(calculate_linux());
    distributions.push(feren_os());
    distributions.push(nitrux());
    distributions.push(kde_neon());
    distributions.push(antergos());

    // Linux Server Distributions (linux/server/)
    distributions.push(centos());
    distributions.push(rocky_linux());
    distributions.push(alma_linux());
    distributions.push(oracle_linux());
    distributions.push(ubuntu_server());
    distributions.push(debian_server());
    distributions.push(fedora_server());
    distributions.push(opensuse_leap());
    distributions.push(suse_enterprise());
    distributions.push(red_hat_enterprise());
    distributions.push(clearos());
    distributions.push(nethserver());
    distributions.push(zentyal());
    distributions.push(univention());
    distributions.push(turnkey_linux());
    distributions.push(proxmox());
    distributions.push(truenas());
    distributions.push(pfsense());
    distributions.push(opnsense());
    distributions.push(vyos());
    distributions.push(ipfire());
    distributions.push(smoothwall());
    distributions.push(untangle());
    distributions.push(clearos_community());
    distributions.push(openfiler());
    distributions.push(freenas());
    distributions.push(xigmanas());
    distributions.push(rockstor());
    distributions.push(amahi());
    distributions.push(koozali_sme());

    // Linux Security/Pentesting (linux/security/)
    distributions.push(kali_linux());
    distributions.push(parrot_security());
    distributions.push(black_arch());
    distributions.push(tails());
    distributions.push(backbox());
    distributions.push(pentoo());
    distributions.push(samurai_wtf());
    distributions.push(caine());
    distributions.push(deft_linux());
    distributions.push(bugtraq());
    distributions.push(weakerth4n());
    distributions.push(network_security_toolkit());
    distributions.push(matriux());
    distributions.push(nodezero());
    distributions.push(knoppix_std());
    distributions.push(cyborg_hawk());
    distributions.push(archstrike());
    distributions.push(fedora_security());
    distributions.push(wifislax());
    distributions.push(dracos_linux());

    // Linux Utility/Rescue (linux/utility/)
    distributions.push(gparted_live());
    distributions.push(clonezilla());
    distributions.push(system_rescue());
    distributions.push(memtest86());
    distributions.push(hirens_boot_cd());
    distributions.push(ultimate_boot_cd());
    distributions.push(rescatux());
    distributions.push(redo_rescue());
    distributions.push(finnix());
    distributions.push(grml());
    distributions.push(knoppix());
    distributions.push(systemback());
    distributions.push(rescuezilla());
    distributions.push(super_grub2_disk());
    distributions.push(boot_repair_disk());
    distributions.push(partition_wizard());
    distributions.push(easeus_todo());
    distributions.push(aomei_backupper());
    distributions.push(macrium_reflect());
    distributions.push(active_boot_disk());

    // Linux Minimal/Embedded (linux/minimal/)
    distributions.push(alpine_linux());
    distributions.push(void_linux());
    distributions.push(damn_small_linux());
    distributions.push(tiny_core_pure());
    distributions.push(slitaz());
    distributions.push(absolute_linux());
    distributions.push(crunchbang());
    distributions.push(bodhi_linux());
    distributions.push(lite_linux());
    distributions.push(bunsen_labs());
    distributions.push(antix_core());
    distributions.push(alpine_extended());

    // Linux Specialty (linux/specialty/)
    distributions.push(gentoo());
    distributions.push(nixos());
    distributions.push(guix());
    distributions.push(bedrock_linux());
    distributions.push(gobolinux());
    distributions.push(lunar_linux());
    distributions.push(source_mage());
    distributions.push(kiss_linux());
    distributions.push(artix());
    distributions.push(devuan());
    distributions.push(hyperbola());
    distributions.push(parabola());
    distributions.push(trisquel());
    distributions.push(guix_system());
    distributions.push(pure_os());
    distributions.push(av_linux());
    distributions.push(ubuntu_studio_full());
    distributions.push(kxstudio());
    distributions.push(apodio());
    distributions.push(dyne_bolic());
    distributions.push(scientific_linux());
    distributions.push(bio_linux());
    distributions.push(cern_centos());
    distributions.push(astronomy_linux());

    // BSD Systems (bsd/)
    distributions.push(freebsd());
    distributions.push(openbsd());
    distributions.push(netbsd());
    distributions.push(dragonfly_bsd());
    distributions.push(ghostbsd());
    distributions.push(nomadbsd());
    distributions.push(midnightbsd());
    distributions.push(hardenedbsd());
    distributions.push(trueos());
    distributions.push(freenas_bsd());
    distributions.push(opnsense_bsd());
    distributions.push(pfsense_bsd());

    // Windows Operating Systems (windows/)
    distributions.push(windows_11());
    distributions.push(windows_10());
    distributions.push(windows_8_1());
    distributions.push(windows_7());
    distributions.push(windows_vista());
    distributions.push(windows_xp());
    distributions.push(windows_server_2022());
    distributions.push(windows_server_2019());
    distributions.push(windows_server_2016());
    distributions.push(windows_server_2012());
    distributions.push(windows_server_2008());
    distributions.push(windows_server_2003());
    distributions.push(windows_server_2000());
    distributions.push(windows_pe());
    distributions.push(windows_embedded());

    // Other Operating Systems (other/)
    distributions.push(haiku());
    distributions.push(reactos());
    distributions.push(menuetos());
    distributions.push(kolibrios());
    distributions.push(morphos());
    distributions.push(aros());
    distributions.push(genode());
    distributions.push(redox());
    distributions.push(serenity());
    distributions.push(temple_os());

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

// Windows Operating Systems
fn windows_11() -> IsoDistribution {
    IsoDistribution {
        name: "windows-11".to_string(),
        display_name: "Windows 11".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com/windows/windows-11".to_string(),
        description: "Latest Windows operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "23H2".to_string(),
                codename: Some("23H2".to_string()),
                release_date: Some("2023-10-31".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64, Architecture::Aarch64],
                flavors: vec!["home".to_string(), "pro".to_string(), "enterprise".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 6500,
            },
        ],
    }
}

fn windows_10() -> IsoDistribution {
    IsoDistribution {
        name: "windows-10".to_string(),
        display_name: "Windows 10".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com/windows/windows-10".to_string(),
        description: "Windows 10 operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "22H2".to_string(),
                codename: Some("22H2".to_string()),
                release_date: Some("2022-10-18".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["home".to_string(), "pro".to_string(), "enterprise".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 6000,
            },
        ],
    }
}

fn windows_8_1() -> IsoDistribution {
    IsoDistribution {
        name: "windows-8.1".to_string(),
        display_name: "Windows 8.1".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows 8.1 operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "Update 3".to_string(),
                codename: None,
                release_date: Some("2013-10-17".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["pro".to_string(), "enterprise".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 4200,
            },
        ],
    }
}

fn windows_7() -> IsoDistribution {
    IsoDistribution {
        name: "windows-7".to_string(),
        display_name: "Windows 7".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows 7 operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "SP1".to_string(),
                codename: None,
                release_date: Some("2009-10-22".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["home".to_string(), "pro".to_string(), "ultimate".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3200,
            },
        ],
    }
}

fn windows_vista() -> IsoDistribution {
    IsoDistribution {
        name: "windows-vista".to_string(),
        display_name: "Windows Vista".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows Vista operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "SP2".to_string(),
                codename: None,
                release_date: Some("2007-01-30".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["home".to_string(), "business".to_string(), "ultimate".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3500,
            },
        ],
    }
}

fn windows_xp() -> IsoDistribution {
    IsoDistribution {
        name: "windows-xp".to_string(),
        display_name: "Windows XP".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows XP operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "SP3".to_string(),
                codename: None,
                release_date: Some("2001-10-25".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::I686],
                flavors: vec!["home".to_string(), "professional".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 600,
            },
        ],
    }
}

fn windows_server_2022() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2022".to_string(),
        display_name: "Windows Server 2022".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com/windows-server".to_string(),
        description: "Latest Windows Server operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "21H2".to_string(),
                codename: None,
                release_date: Some("2021-08-18".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string(), "datacenter".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 5500,
            },
        ],
    }
}

fn windows_server_2019() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2019".to_string(),
        display_name: "Windows Server 2019".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com/windows-server".to_string(),
        description: "Windows Server 2019 operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "1809".to_string(),
                codename: None,
                release_date: Some("2018-10-02".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string(), "datacenter".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 5200,
            },
        ],
    }
}

fn windows_server_2016() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2016".to_string(),
        display_name: "Windows Server 2016".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com/windows-server".to_string(),
        description: "Windows Server 2016 operating system".to_string(),
        versions: vec![
            IsoVersion {
                version: "1607".to_string(),
                codename: None,
                release_date: Some("2016-10-12".to_string()),
                is_lts: true,
                is_current: false,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string(), "datacenter".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 6000,
            },
        ],
    }
}

fn windows_server_2012() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2012".to_string(),
        display_name: "Windows Server 2012 R2".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows Server 2012 R2 operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "R2".to_string(),
                codename: None,
                release_date: Some("2013-10-18".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string(), "datacenter".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 4500,
            },
        ],
    }
}

fn windows_server_2008() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2008".to_string(),
        display_name: "Windows Server 2008 R2".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows Server 2008 R2 operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "R2 SP1".to_string(),
                codename: None,
                release_date: Some("2009-10-22".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["standard".to_string(), "datacenter".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3200,
            },
        ],
    }
}

fn windows_server_2003() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2003".to_string(),
        display_name: "Windows Server 2003".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows Server 2003 operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "R2 SP2".to_string(),
                codename: None,
                release_date: Some("2003-04-24".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::X86_64, Architecture::I686],
                flavors: vec!["standard".to_string(), "enterprise".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 600,
            },
        ],
    }
}

fn windows_server_2000() -> IsoDistribution {
    IsoDistribution {
        name: "windows-server-2000".to_string(),
        display_name: "Windows Server 2000".to_string(),
        category: DistributionCategory::Windows,
        homepage: "https://www.microsoft.com".to_string(),
        description: "Windows Server 2000 operating system (EOL)".to_string(),
        versions: vec![
            IsoVersion {
                version: "SP4".to_string(),
                codename: None,
                release_date: Some("2000-02-17".to_string()),
                is_lts: false,
                is_current: false,
                architectures: vec![Architecture::I686],
                flavors: vec!["server".to_string(), "advanced".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 500,
            },
        ],
    }
}

fn kubuntu() -> IsoDistribution {
    IsoDistribution {
        name: "kubuntu".to_string(),
        display_name: "Kubuntu".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://kubuntu.org".to_string(),
        description: "Ubuntu with KDE Plasma desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/kubuntu/releases/22.04.3/release/kubuntu-22.04.3-desktop-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/kubuntu/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/kubuntu/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 3400,
            },
        ],
    }
}

fn xubuntu() -> IsoDistribution {
    IsoDistribution {
        name: "xubuntu".to_string(),
        display_name: "Xubuntu".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://xubuntu.org".to_string(),
        description: "Ubuntu with XFCE desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/xubuntu/releases/22.04.3/release/xubuntu-22.04.3-desktop-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/xubuntu/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/xubuntu/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 2800,
            },
        ],
    }
}

fn lubuntu() -> IsoDistribution {
    IsoDistribution {
        name: "lubuntu".to_string(),
        display_name: "Lubuntu".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://lubuntu.me".to_string(),
        description: "Ubuntu with LXQt desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/lubuntu/releases/22.04.3/release/lubuntu-22.04.3-desktop-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/lubuntu/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/lubuntu/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 2400,
            },
        ],
    }
}

fn ubuntu_mate() -> IsoDistribution {
    IsoDistribution {
        name: "ubuntu-mate".to_string(),
        display_name: "Ubuntu MATE".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://ubuntu-mate.org".to_string(),
        description: "Ubuntu with MATE desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/ubuntu-mate/releases/22.04.3/release/ubuntu-mate-22.04.3-desktop-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntu-mate/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntu-mate/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 3200,
            },
        ],
    }
}

fn ubuntu_budgie() -> IsoDistribution {
    IsoDistribution {
        name: "ubuntu-budgie".to_string(),
        display_name: "Ubuntu Budgie".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://ubuntubudgie.org".to_string(),
        description: "Ubuntu with Budgie desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/ubuntu-budgie/releases/22.04.3/release/ubuntu-budgie-22.04.3-desktop-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntu-budgie/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntu-budgie/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 3100,
            },
        ],
    }
}

fn ubuntu_studio() -> IsoDistribution {
    IsoDistribution {
        name: "ubuntu-studio".to_string(),
        display_name: "Ubuntu Studio".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://ubuntustudio.org".to_string(),
        description: "Ubuntu for multimedia production".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/ubuntustudio/releases/22.04.3/release/ubuntustudio-22.04.3-dvd-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntustudio/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntustudio/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 4500,
            },
        ],
    }
}

fn ubuntu_kylin() -> IsoDistribution {
    IsoDistribution {
        name: "ubuntu-kylin".to_string(),
        display_name: "Ubuntu Kylin".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://www.ubuntukylin.com".to_string(),
        description: "Ubuntu for Chinese users".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04.3".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2023-08-10".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://cdimage.ubuntu.com/ubuntukylin/releases/22.04.3/release/ubuntukylin-22.04.3-desktop-amd64.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntukylin/releases/22.04.3/release/SHA256SUMS".to_string()),
                ]),
                signature_urls: HashMap::from([
                    ("x86_64".to_string(), "https://cdimage.ubuntu.com/ubuntukylin/releases/22.04.3/release/SHA256SUMS.gpg".to_string()),
                ]),
                size_mb: 3800,
            },
        ],
    }
}

fn fedora_kde() -> IsoDistribution {
    IsoDistribution {
        name: "fedora-kde".to_string(),
        display_name: "Fedora KDE Plasma".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://spins.fedoraproject.org/kde".to_string(),
        description: "Fedora with KDE Plasma desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-KDE-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Spins-39-1.5-x86_64-CHECKSUM".to_string()),
                ]),
                signature_urls: HashMap::new(),
                size_mb: 2100,
            },
        ],
    }
}

fn fedora_xfce() -> IsoDistribution {
    IsoDistribution {
        name: "fedora-xfce".to_string(),
        display_name: "Fedora XFCE".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://spins.fedoraproject.org/xfce".to_string(),
        description: "Fedora with XFCE desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-XFCE-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Spins-39-1.5-x86_64-CHECKSUM".to_string()),
                ]),
                signature_urls: HashMap::new(),
                size_mb: 1700,
            },
        ],
    }
}

fn fedora_lxde() -> IsoDistribution {
    IsoDistribution {
        name: "fedora-lxde".to_string(),
        display_name: "Fedora LXDE".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://spins.fedoraproject.org/lxde".to_string(),
        description: "Fedora with LXDE desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-LXDE-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Spins-39-1.5-x86_64-CHECKSUM".to_string()),
                ]),
                signature_urls: HashMap::new(),
                size_mb: 1500,
            },
        ],
    }
}

fn fedora_mate() -> IsoDistribution {
    IsoDistribution {
        name: "fedora-mate".to_string(),
        display_name: "Fedora MATE-Compiz".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://spins.fedoraproject.org/mate-compiz".to_string(),
        description: "Fedora with MATE desktop and Compiz".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-MATE_Compiz-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Spins-39-1.5-x86_64-CHECKSUM".to_string()),
                ]),
                signature_urls: HashMap::new(),
                size_mb: 2000,
            },
        ],
    }
}

fn fedora_cinnamon() -> IsoDistribution {
    IsoDistribution {
        name: "fedora-cinnamon".to_string(),
        display_name: "Fedora Cinnamon".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://spins.fedoraproject.org/cinnamon".to_string(),
        description: "Fedora with Cinnamon desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Cinnamon-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Spins-39-1.5-x86_64-CHECKSUM".to_string()),
                ]),
                signature_urls: HashMap::new(),
                size_mb: 2100,
            },
        ],
    }
}

fn fedora_soas() -> IsoDistribution {
    IsoDistribution {
        name: "fedora-soas".to_string(),
        display_name: "Fedora SoaS (Sugar on a Stick)".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://spins.fedoraproject.org/soas".to_string(),
        description: "Fedora with Sugar learning platform".to_string(),
        versions: vec![
            IsoVersion {
                version: "39".to_string(),
                codename: None,
                release_date: Some("2023-11-07".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::from([
                    ("x86_64-desktop".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-SoaS-Live-x86_64-39-1.5.iso".to_string()),
                ]),
                checksum_urls: HashMap::from([
                    ("x86_64".to_string(), "https://download.fedoraproject.org/pub/fedora/linux/releases/39/Spins/x86_64/iso/Fedora-Spins-39-1.5-x86_64-CHECKSUM".to_string()),
                ]),
                signature_urls: HashMap::new(),
                size_mb: 1100,
            },
        ],
    }
}

// Additional Linux Desktop Distributions

pub fn edubuntu() -> IsoDistribution {
    IsoDistribution {
        name: "edubuntu".to_string(),
        display_name: "Edubuntu".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://www.edubuntu.org".to_string(),
        description: "Ubuntu for education".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2022-04-21".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3500,
            },
        ],
    }
}

pub fn ubuntu_unity() -> IsoDistribution {
    IsoDistribution {
        name: "ubuntu-unity".to_string(),
        display_name: "Ubuntu Unity".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://ubuntuunity.org".to_string(),
        description: "Ubuntu with Unity desktop".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04".to_string(),
                codename: Some("Jammy Jellyfish".to_string()),
                release_date: Some("2022-04-21".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3200,
            },
        ],
    }
}

pub fn pop_os() -> IsoDistribution {
    IsoDistribution {
        name: "pop-os".to_string(),
        display_name: "Pop!_OS".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://pop.system76.com".to_string(),
        description: "Ubuntu-based distribution by System76".to_string(),
        versions: vec![
            IsoVersion {
                version: "22.04".to_string(),
                codename: None,
                release_date: Some("2022-04-25".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["nvidia".to_string(), "intel-amd".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2800,
            },
        ],
    }
}

pub fn elementary_os() -> IsoDistribution {
    IsoDistribution {
        name: "elementary".to_string(),
        display_name: "elementary OS".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://elementary.io".to_string(),
        description: "Beautiful Ubuntu-based distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "7".to_string(),
                codename: Some("Horus".to_string()),
                release_date: Some("2023-04-20".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["desktop".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2900,
            },
        ],
    }
}

pub fn linux_mint() -> IsoDistribution {
    IsoDistribution {
        name: "mint".to_string(),
        display_name: "Linux Mint".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://linuxmint.com".to_string(),
        description: "Elegant and comfortable Ubuntu-based distribution".to_string(),
        versions: vec![
            IsoVersion {
                version: "21.2".to_string(),
                codename: Some("Victoria".to_string()),
                release_date: Some("2023-07-16".to_string()),
                is_lts: true,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["cinnamon".to_string(), "mate".to_string(), "xfce".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2700,
            },
        ],
    }
}

pub fn linux_mint_debian() -> IsoDistribution {
    IsoDistribution {
        name: "lmde".to_string(),
        display_name: "LMDE (Linux Mint Debian Edition)".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://linuxmint.com/download_lmde.php".to_string(),
        description: "Linux Mint based on Debian".to_string(),
        versions: vec![
            IsoVersion {
                version: "6".to_string(),
                codename: Some("Faye".to_string()),
                release_date: Some("2023-09-13".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["cinnamon".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 2600,
            },
        ],
    }
}

pub fn zorin_os() -> IsoDistribution {
    IsoDistribution {
        name: "zorin".to_string(),
        display_name: "Zorin OS".to_string(),
        category: DistributionCategory::Linux,
        homepage: "https://zorin.com/os".to_string(),
        description: "Windows and macOS alternative".to_string(),
        versions: vec![
            IsoVersion {
                version: "16.3".to_string(),
                codename: None,
                release_date: Some("2023-04-05".to_string()),
                is_lts: false,
                is_current: true,
                architectures: vec![Architecture::X86_64],
                flavors: vec!["core".to_string(), "lite".to_string(), "education".to_string()],
                download_urls: HashMap::new(),
                checksum_urls: HashMap::new(),
                signature_urls: HashMap::new(),
                size_mb: 3100,
            },
        ],
    }
}

// Due to size limitations, I'll create stub functions that return placeholder data
// These would be fully implemented with real URLs in production

macro_rules! simple_distro {
    ($name:expr, $display:expr, $category:expr, $homepage:expr, $desc:expr, $size:expr) => {
        IsoDistribution {
            name: $name.to_string(),
            display_name: $display.to_string(),
            category: $category,
            homepage: $homepage.to_string(),
            description: $desc.to_string(),
            versions: vec![
                IsoVersion {
                    version: "latest".to_string(),
                    codename: None,
                    release_date: None,
                    is_lts: false,
                    is_current: true,
                    architectures: vec![Architecture::X86_64],
                    flavors: vec!["standard".to_string()],
                    download_urls: HashMap::new(),
                    checksum_urls: HashMap::new(),
                    signature_urls: HashMap::new(),
                    size_mb: $size,
                },
            ],
        }
    };
}

pub fn manjaro_kde() -> IsoDistribution {
    simple_distro!("manjaro-kde", "Manjaro KDE", DistributionCategory::Linux, "https://manjaro.org", "Manjaro with KDE Plasma", 3200)
}

pub fn manjaro_xfce() -> IsoDistribution {
    simple_distro!("manjaro-xfce", "Manjaro XFCE", DistributionCategory::Linux, "https://manjaro.org", "Manjaro with XFCE", 2900)
}

pub fn endeavouros() -> IsoDistribution {
    simple_distro!("endeavour", "EndeavourOS", DistributionCategory::Linux, "https://endeavouros.com", "User-friendly Arch Linux", 2100)
}

pub fn garuda_linux() -> IsoDistribution {
    simple_distro!("garuda", "Garuda Linux", DistributionCategory::Linux, "https://garudalinux.org", "Performance-focused Arch Linux", 3500)
}

pub fn mx_linux() -> IsoDistribution {
    simple_distro!("mx", "MX Linux", DistributionCategory::Linux, "https://mxlinux.org", "Debian-based mid-weight distribution", 1800)
}

pub fn antiX() -> IsoDistribution {
    simple_distro!("antix", "antiX", DistributionCategory::Linux, "https://antixlinux.com", "Lightweight Debian-based system", 900)
}

pub fn puppy_linux() -> IsoDistribution {
    simple_distro!("puppy", "Puppy Linux", DistributionCategory::Linux, "https://puppylinux.com", "Tiny fast lightweight Linux", 400)
}

pub fn tiny_core() -> IsoDistribution {
    simple_distro!("tinycore", "Tiny Core Linux", DistributionCategory::Linux, "http://tinycorelinux.net", "Minimal Linux distribution", 16)
}

pub fn slax() -> IsoDistribution {
    simple_distro!("slax", "Slax", DistributionCategory::Linux, "https://www.slax.org", "Portable Linux operating system", 300)
}

pub fn porteus() -> IsoDistribution {
    simple_distro!("porteus", "Porteus", DistributionCategory::Linux, "http://www.porteus.org", "Fast and portable Linux", 300)
}

pub fn peppermint_os() -> IsoDistribution {
    simple_distro!("peppermint", "Peppermint OS", DistributionCategory::Linux, "https://peppermintos.com", "Cloud-focused Linux distribution", 1800)
}

pub fn deepin() -> IsoDistribution {
    simple_distro!("deepin", "Deepin", DistributionCategory::Linux, "https://www.deepin.org", "Beautiful Chinese Linux distribution", 3100)
}

pub fn endless_os() -> IsoDistribution {
    simple_distro!("endless", "Endless OS", DistributionCategory::Linux, "https://endlessos.com", "Easy-to-use Linux for everyone", 3800)
}

pub fn solus() -> IsoDistribution {
    simple_distro!("solus", "Solus", DistributionCategory::Linux, "https://getsol.us", "Independent Linux distribution", 2000)
}

pub fn mageia() -> IsoDistribution {
    simple_distro!("mageia", "Mageia", DistributionCategory::Linux, "https://www.mageia.org", "Community-driven Mandriva fork", 3200)
}

pub fn pclinuxos() -> IsoDistribution {
    simple_distro!("pclinuxos", "PCLinuxOS", DistributionCategory::Linux, "https://www.pclinuxos.com", "User-friendly rolling release", 1900)
}

pub fn sabayon() -> IsoDistribution {
    simple_distro!("sabayon", "Sabayon", DistributionCategory::Linux, "https://www.sabayon.org", "Gentoo-based distribution", 3500)
}

pub fn kaos() -> IsoDistribution {
    simple_distro!("kaos", "KaOS", DistributionCategory::Linux, "https://kaosx.us", "KDE-focused independent distribution", 2800)
}

pub fn chakra() -> IsoDistribution {
    simple_distro!("chakra", "Chakra", DistributionCategory::Linux, "https://www.chakralinux.org", "Arch-based KDE-centric distribution", 2500)
}

pub fn calculate_linux() -> IsoDistribution {
    simple_distro!("calculate", "Calculate Linux", DistributionCategory::Linux, "https://www.calculate-linux.org", "Gentoo-based distribution", 2300)
}

pub fn feren_os() -> IsoDistribution {
    simple_distro!("feren", "Feren OS", DistributionCategory::Linux, "https://ferenos.weebly.com", "Modern Ubuntu-based desktop", 2900)
}

pub fn nitrux() -> IsoDistribution {
    simple_distro!("nitrux", "Nitrux", DistributionCategory::Linux, "https://nxos.org", "Debian-based with NX Desktop", 2100)
}

pub fn kde_neon() -> IsoDistribution {
    simple_distro!("neon", "KDE neon", DistributionCategory::Linux, "https://neon.kde.org", "Latest KDE on Ubuntu base", 2800)
}

pub fn antergos() -> IsoDistribution {
    simple_distro!("antergos", "Antergos", DistributionCategory::Linux, "https://antergos.com", "Arch-based distribution (discontinued)", 2200)
}

// Server distributions
pub fn ubuntu_server() -> IsoDistribution {
    simple_distro!("ubuntu-server", "Ubuntu Server", DistributionCategory::Server, "https://ubuntu.com/server", "Ubuntu for servers", 1200)
}

pub fn debian_server() -> IsoDistribution {
    simple_distro!("debian-server", "Debian Server", DistributionCategory::Server, "https://www.debian.org", "Debian for servers", 700)
}

pub fn fedora_server() -> IsoDistribution {
    simple_distro!("fedora-server", "Fedora Server", DistributionCategory::Server, "https://getfedora.org/server", "Fedora for servers", 2000)
}

pub fn opensuse_leap() -> IsoDistribution {
    simple_distro!("opensuse-leap", "openSUSE Leap", DistributionCategory::Server, "https://www.opensuse.org", "Stable openSUSE release", 4300)
}

pub fn suse_enterprise() -> IsoDistribution {
    simple_distro!("sles", "SUSE Enterprise Linux", DistributionCategory::Server, "https://www.suse.com", "Enterprise Linux from SUSE", 4500)
}

pub fn red_hat_enterprise() -> IsoDistribution {
    simple_distro!("rhel", "Red Hat Enterprise Linux", DistributionCategory::Server, "https://www.redhat.com/rhel", "Enterprise Linux from Red Hat", 8000)
}

pub fn oracle_linux() -> IsoDistribution {
    simple_distro!("oracle", "Oracle Linux", DistributionCategory::Server, "https://www.oracle.com/linux", "Enterprise Linux from Oracle", 9500)
}

pub fn clearos() -> IsoDistribution {
    simple_distro!("clearos", "ClearOS", DistributionCategory::Server, "https://www.clearos.com", "Server and network gateway system", 2000)
}

pub fn nethserver() -> IsoDistribution {
    simple_distro!("nethserver", "NethServer", DistributionCategory::Server, "https://www.nethserver.org", "Linux server for small businesses", 1500)
}

pub fn zentyal() -> IsoDistribution {
    simple_distro!("zentyal", "Zentyal", DistributionCategory::Server, "https://zentyal.com", "Linux small business server", 2500)
}

pub fn univention() -> IsoDistribution {
    simple_distro!("univention", "Univention Corporate Server", DistributionCategory::Server, "https://www.univention.com", "Enterprise Linux platform", 3000)
}

pub fn turnkey_linux() -> IsoDistribution {
    simple_distro!("turnkey", "TurnKey Linux", DistributionCategory::Server, "https://www.turnkeylinux.org", "Ready-to-use server appliances", 800)
}

// Continue with stubs for remaining distributions...
// (This file would continue with all other distribution stubs)

pub fn ipfire() -> IsoDistribution {
    simple_distro!("ipfire", "IPFire", DistributionCategory::Server, "https://www.ipfire.org", "Firewall distribution", 300)
}

pub fn smoothwall() -> IsoDistribution {
    simple_distro!("smoothwall", "SmoothWall", DistributionCategory::Server, "https://www.smoothwall.org", "Firewall and proxy server", 250)
}

pub fn untangle() -> IsoDistribution {
    simple_distro!("untangle", "Untangle NG Firewall", DistributionCategory::Server, "https://www.untangle.com", "Network gateway platform", 1200)
}

pub fn clearos_community() -> IsoDistribution {
    simple_distro!("clearos-community", "ClearOS Community", DistributionCategory::Server, "https://www.clearos.com", "Community version of ClearOS", 2000)
}

pub fn openfiler() -> IsoDistribution {
    simple_distro!("openfiler", "Openfiler", DistributionCategory::Server, "https://www.openfiler.com", "Network storage solution", 600)
}

pub fn freenas() -> IsoDistribution {
    simple_distro!("freenas", "FreeNAS", DistributionCategory::Server, "https://www.freenas.org", "FreeBSD-based NAS system", 900)
}

pub fn xigmanas() -> IsoDistribution {
    simple_distro!("xigmanas", "XigmaNAS", DistributionCategory::Server, "https://xigmanas.com", "NAS solution (FreeNAS fork)", 900)
}

pub fn rockstor() -> IsoDistribution {
    simple_distro!("rockstor", "Rockstor", DistributionCategory::Server, "https://rockstor.com", "Linux and BTRFS based NAS", 1000)
}

pub fn amahi() -> IsoDistribution {
    simple_distro!("amahi", "Amahi", DistributionCategory::Server, "https://www.amahi.org", "Home server and media server", 1500)
}

pub fn koozali_sme() -> IsoDistribution {
    simple_distro!("koozali", "Koozali SME Server", DistributionCategory::Server, "https://koozali.org", "SME Server distribution", 1000)
}

// Security distributions
pub fn backbox() -> IsoDistribution {
    simple_distro!("backbox", "BackBox", DistributionCategory::Security, "https://www.backbox.org", "Ubuntu-based penetration testing distro", 3000)
}

pub fn pentoo() -> IsoDistribution {
    simple_distro!("pentoo", "Pentoo", DistributionCategory::Security, "https://www.pentoo.ch", "Gentoo-based security distro", 3500)
}

pub fn samurai_wtf() -> IsoDistribution {
    simple_distro!("samurai", "Samurai WTF", DistributionCategory::Security, "http://www.samurai-wtf.org", "Web testing framework", 3000)
}

pub fn caine() -> IsoDistribution {
    simple_distro!("caine", "CAINE", DistributionCategory::Security, "https://www.caine-live.net", "Computer forensics distribution", 4200)
}

pub fn deft_linux() -> IsoDistribution {
    simple_distro!("deft", "DEFT Linux", DistributionCategory::Security, "http://www.deftlinux.net", "Digital forensics platform", 3500)
}

pub fn bugtraq() -> IsoDistribution {
    simple_distro!("bugtraq", "Bugtraq", DistributionCategory::Security, "https://bugtraq-team.com", "Penetration testing distribution", 4000)
}

pub fn weakerth4n() -> IsoDistribution {
    simple_distro!("weakerth4n", "Weakerth4n", DistributionCategory::Security, "https://weaknetlabs.com", "Penetration testing toolkit", 2500)
}

pub fn network_security_toolkit() -> IsoDistribution {
    simple_distro!("nst", "Network Security Toolkit", DistributionCategory::Security, "https://www.networksecuritytoolkit.org", "Network security monitoring", 2800)
}

pub fn matriux() -> IsoDistribution {
    simple_distro!("matriux", "Matriux", DistributionCategory::Security, "http://matriux.com", "Security and penetration testing", 4000)
}

pub fn nodezero() -> IsoDistribution {
    simple_distro!("nodezero", "NodeZero", DistributionCategory::Security, "http://www.nodezero-linux.org", "Penetration testing distro", 3200)
}

pub fn knoppix_std() -> IsoDistribution {
    simple_distro!("knoppix-std", "Knoppix STD", DistributionCategory::Security, "http://s-t-d.org", "Security tools distribution", 700)
}

pub fn cyborg_hawk() -> IsoDistribution {
    simple_distro!("cyborg", "Cyborg Hawk", DistributionCategory::Security, "https://cyborg.ztrela.com", "Penetration testing distro", 4500)
}

pub fn archstrike() -> IsoDistribution {
    simple_distro!("archstrike", "ArchStrike", DistributionCategory::Security, "https://archstrike.org", "Arch-based security distro", 3000)
}

pub fn fedora_security() -> IsoDistribution {
    simple_distro!("fedora-security", "Fedora Security Lab", DistributionCategory::Security, "https://labs.fedoraproject.org/security", "Fedora security spin", 2200)
}

pub fn wifislax() -> IsoDistribution {
    simple_distro!("wifislax", "Wifislax", DistributionCategory::Security, "https://www.wifislax.com", "WiFi security testing", 3200)
}

pub fn dracos_linux() -> IsoDistribution {
    simple_distro!("dracos", "Dracos Linux", DistributionCategory::Security, "https://www.dracos-linux.org", "Penetration testing platform", 3800)
}

// Utility distributions
pub fn rescatux() -> IsoDistribution {
    simple_distro!("rescatux", "Rescatux", DistributionCategory::Utility, "https://www.supergrubdisk.org/rescatux", "GNU/Linux rescue disk", 600)
}

pub fn redo_rescue() -> IsoDistribution {
    simple_distro!("redo", "Redo Rescue", DistributionCategory::Utility, "http://redorescue.com", "Backup and recovery", 700)
}

pub fn finnix() -> IsoDistribution {
    simple_distro!("finnix", "Finnix", DistributionCategory::Utility, "https://www.finnix.org", "System administrator live CD", 400)
}

pub fn grml() -> IsoDistribution {
    simple_distro!("grml", "Grml", DistributionCategory::Utility, "https://grml.org", "Debian-based live system", 500)
}

pub fn knoppix() -> IsoDistribution {
    simple_distro!("knoppix", "Knoppix", DistributionCategory::Utility, "http://www.knoppix.org", "Live Linux distribution", 4500)
}

pub fn systemback() -> IsoDistribution {
    simple_distro!("systemback", "Systemback", DistributionCategory::Utility, "https://github.com/BluewhaleRobot/systemback", "System backup and restore", 900)
}

pub fn rescuezilla() -> IsoDistribution {
    simple_distro!("rescuezilla", "Rescuezilla", DistributionCategory::Utility, "https://rescuezilla.com", "Easy backup and recovery", 900)
}

pub fn super_grub2_disk() -> IsoDistribution {
    simple_distro!("supergrub2", "Super Grub2 Disk", DistributionCategory::Utility, "https://www.supergrubdisk.org", "Boot repair tool", 30)
}

pub fn boot_repair_disk() -> IsoDistribution {
    simple_distro!("boot-repair", "Boot Repair Disk", DistributionCategory::Utility, "https://sourceforge.net/projects/boot-repair-cd", "Repair boot problems", 700)
}

pub fn partition_wizard() -> IsoDistribution {
    simple_distro!("partition-wizard", "MiniTool Partition Wizard", DistributionCategory::Utility, "https://www.partitionwizard.com", "Partition management", 300)
}

pub fn easeus_todo() -> IsoDistribution {
    simple_distro!("easeus", "EaseUS Todo Backup", DistributionCategory::Utility, "https://www.easeus.com", "Backup and recovery", 400)
}

pub fn aomei_backupper() -> IsoDistribution {
    simple_distro!("aomei", "AOMEI Backupper", DistributionCategory::Utility, "https://www.backup-utility.com", "Backup solution", 350)
}

pub fn macrium_reflect() -> IsoDistribution {
    simple_distro!("macrium", "Macrium Reflect", DistributionCategory::Utility, "https://www.macrium.com", "Disk imaging and cloning", 600)
}

pub fn active_boot_disk() -> IsoDistribution {
    simple_distro!("active-boot", "Active@ Boot Disk", DistributionCategory::Utility, "https://www.disk-tools.com", "Data recovery and utilities", 700)
}

// Minimal distributions
pub fn damn_small_linux() -> IsoDistribution {
    simple_distro!("dsl", "Damn Small Linux", DistributionCategory::Linux, "http://www.damnsmalllinux.org", "50MB desktop Linux", 50)
}

pub fn tiny_core_pure() -> IsoDistribution {
    simple_distro!("tinycore-pure", "Tiny Core Pure", DistributionCategory::Linux, "http://tinycorelinux.net", "16MB Linux core", 16)
}

pub fn slitaz() -> IsoDistribution {
    simple_distro!("slitaz", "SliTaz", DistributionCategory::Linux, "http://www.slitaz.org", "Lightweight Linux distribution", 40)
}

pub fn absolute_linux() -> IsoDistribution {
    simple_distro!("absolute", "Absolute Linux", DistributionCategory::Linux, "https://www.absolutelinux.org", "Slackware-based lightweight distro", 700)
}

pub fn crunchbang() -> IsoDistribution {
    simple_distro!("crunchbang", "CrunchBang", DistributionCategory::Linux, "http://crunchbang.org", "Debian with Openbox (discontinued)", 700)
}

pub fn bodhi_linux() -> IsoDistribution {
    simple_distro!("bodhi", "Bodhi Linux", DistributionCategory::Linux, "https://www.bodhilinux.com", "Enlightened Ubuntu distribution", 900)
}

pub fn lite_linux() -> IsoDistribution {
    simple_distro!("lite", "Linux Lite", DistributionCategory::Linux, "https://www.linuxliteos.com", "Lightweight Ubuntu-based distro", 1500)
}

pub fn bunsen_labs() -> IsoDistribution {
    simple_distro!("bunsenlabs", "BunsenLabs", DistributionCategory::Linux, "https://www.bunsenlabs.org", "CrunchBang successor", 1000)
}

pub fn antix_core() -> IsoDistribution {
    simple_distro!("antix-core", "antiX Core", DistributionCategory::Linux, "https://antixlinux.com", "Minimal antiX variant", 250)
}

pub fn alpine_extended() -> IsoDistribution {
    simple_distro!("alpine-extended", "Alpine Extended", DistributionCategory::Linux, "https://alpinelinux.org", "Alpine with more packages", 700)
}

// Specialty distributions
pub fn guix() -> IsoDistribution {
    simple_distro!("guix", "GNU Guix", DistributionCategory::Linux, "https://guix.gnu.org", "Transactional package manager", 1200)
}

pub fn bedrock_linux() -> IsoDistribution {
    simple_distro!("bedrock", "Bedrock Linux", DistributionCategory::Linux, "https://bedrocklinux.org", "Meta-distribution", 500)
}

pub fn gobolinux() -> IsoDistribution {
    simple_distro!("gobo", "GoboLinux", DistributionCategory::Linux, "https://gobolinux.org", "Alternative filesystem hierarchy", 800)
}

pub fn lunar_linux() -> IsoDistribution {
    simple_distro!("lunar", "Lunar Linux", DistributionCategory::Linux, "http://www.lunar-linux.org", "Source-based distribution", 600)
}

pub fn source_mage() -> IsoDistribution {
    simple_distro!("sourcemage", "Source Mage", DistributionCategory::Linux, "https://sourcemage.org", "Source-based Linux distribution", 700)
}

pub fn kiss_linux() -> IsoDistribution {
    simple_distro!("kiss", "KISS Linux", DistributionCategory::Linux, "https://kisslinux.org", "Keep It Simple Stupid Linux", 100)
}

pub fn artix() -> IsoDistribution {
    simple_distro!("artix", "Artix Linux", DistributionCategory::Linux, "https://artixlinux.org", "Arch without systemd", 1800)
}

pub fn devuan() -> IsoDistribution {
    simple_distro!("devuan", "Devuan", DistributionCategory::Linux, "https://devuan.org", "Debian without systemd", 1000)
}

pub fn hyperbola() -> IsoDistribution {
    simple_distro!("hyperbola", "Hyperbola GNU/Linux-libre", DistributionCategory::Linux, "https://www.hyperbola.info", "Arch-based libre distro", 900)
}

pub fn parabola() -> IsoDistribution {
    simple_distro!("parabola", "Parabola GNU/Linux-libre", DistributionCategory::Linux, "https://www.parabola.nu", "Free Arch-based distro", 900)
}

pub fn trisquel() -> IsoDistribution {
    simple_distro!("trisquel", "Trisquel", DistributionCategory::Linux, "https://trisquel.info", "Fully free Ubuntu derivative", 2000)
}

pub fn guix_system() -> IsoDistribution {
    simple_distro!("guix-system", "Guix System", DistributionCategory::Linux, "https://guix.gnu.org", "Guix as a complete OS", 1500)
}

pub fn pure_os() -> IsoDistribution {
    simple_distro!("pureos", "PureOS", DistributionCategory::Linux, "https://pureos.net", "FSF-endorsed Debian derivative", 2200)
}

pub fn av_linux() -> IsoDistribution {
    simple_distro!("avlinux", "AV Linux", DistributionCategory::Linux, "http://www.bandshed.net/avlinux", "Audio/video production distro", 3000)
}

pub fn ubuntu_studio_full() -> IsoDistribution {
    simple_distro!("ubuntu-studio-full", "Ubuntu Studio Full", DistributionCategory::Linux, "https://ubuntustudio.org", "Complete multimedia production", 5000)
}

pub fn kxstudio() -> IsoDistribution {
    simple_distro!("kxstudio", "KXStudio", DistributionCategory::Linux, "https://kx.studio", "Audio production platform", 3500)
}

pub fn apodio() -> IsoDistribution {
    simple_distro!("apodio", "Apodio", DistributionCategory::Linux, "http://www.apodio.org", "Debian for audio production", 3200)
}

pub fn dyne_bolic() -> IsoDistribution {
    simple_distro!("dynebolic", "dyne:bolic", DistributionCategory::Linux, "https://www.dynebolic.org", "Media production and activism", 1200)
}

pub fn scientific_linux() -> IsoDistribution {
    simple_distro!("scientific", "Scientific Linux", DistributionCategory::Linux, "https://www.scientificlinux.org", "RHEL clone for labs (discontinued)", 8000)
}

pub fn bio_linux() -> IsoDistribution {
    simple_distro!("biolinux", "Bio-Linux", DistributionCategory::Linux, "http://environmentalomics.org/bio-linux", "Bioinformatics workstation", 3500)
}

pub fn cern_centos() -> IsoDistribution {
    simple_distro!("cernvm", "CernVM", DistributionCategory::Linux, "https://cernvm.cern.ch", "CERN virtual machine", 500)
}

pub fn astronomy_linux() -> IsoDistribution {
    simple_distro!("astrolinux", "Astronomy Linux", DistributionCategory::Linux, "https://www.astronomylinux.com", "Astronomy software collection", 3000)
}

// BSD Systems
pub fn dragonfly_bsd() -> IsoDistribution {
    simple_distro!("dragonflybsd", "DragonFly BSD", DistributionCategory::BSD, "https://www.dragonflybsd.org", "Fork of FreeBSD 4.8", 600)
}

pub fn ghostbsd() -> IsoDistribution {
    simple_distro!("ghostbsd", "GhostBSD", DistributionCategory::BSD, "https://www.ghostbsd.org", "Desktop-oriented FreeBSD", 2500)
}

pub fn nomadbsd() -> IsoDistribution {
    simple_distro!("nomadbsd", "NomadBSD", DistributionCategory::BSD, "https://nomadbsd.org", "Persistent live system", 2000)
}

pub fn midnightbsd() -> IsoDistribution {
    simple_distro!("midnightbsd", "MidnightBSD", DistributionCategory::BSD, "https://www.midnightbsd.org", "Desktop BSD OS", 800)
}

pub fn hardenedbsd() -> IsoDistribution {
    simple_distro!("hardenedbsd", "HardenedBSD", DistributionCategory::BSD, "https://hardenedbsd.org", "Security-enhanced FreeBSD", 1200)
}

pub fn trueos() -> IsoDistribution {
    simple_distro!("trueos", "TrueOS", DistributionCategory::BSD, "https://www.trueos.org", "Desktop BSD (discontinued)", 2500)
}

pub fn freenas_bsd() -> IsoDistribution {
    simple_distro!("freenas-bsd", "FreeNAS (BSD)", DistributionCategory::BSD, "https://www.freenas.org", "FreeBSD-based NAS", 900)
}

pub fn opnsense_bsd() -> IsoDistribution {
    simple_distro!("opnsense-bsd", "OPNsense (BSD)", DistributionCategory::BSD, "https://opnsense.org", "FreeBSD-based firewall", 700)
}

pub fn pfsense_bsd() -> IsoDistribution {
    simple_distro!("pfsense-bsd", "pfSense (BSD)", DistributionCategory::BSD, "https://www.pfsense.org", "FreeBSD-based firewall", 750)
}

// Windows additional
pub fn windows_pe() -> IsoDistribution {
    simple_distro!("winpe", "Windows PE", DistributionCategory::Windows, "https://docs.microsoft.com/windows-hardware/manufacture/desktop/winpe-intro", "Windows Preinstallation Environment", 400)
}

pub fn windows_embedded() -> IsoDistribution {
    simple_distro!("win-embedded", "Windows Embedded", DistributionCategory::Windows, "https://www.microsoft.com", "Windows for embedded systems", 2000)
}

// Other operating systems
pub fn haiku() -> IsoDistribution {
    simple_distro!("haiku", "Haiku", DistributionCategory::Other, "https://www.haiku-os.org", "BeOS-inspired operating system", 800)
}

pub fn reactos() -> IsoDistribution {
    simple_distro!("reactos", "ReactOS", DistributionCategory::Other, "https://reactos.org", "Open-source Windows alternative", 120)
}

pub fn menuetos() -> IsoDistribution {
    simple_distro!("menuet", "MenuetOS", DistributionCategory::Other, "http://www.menuetos.net", "Assembly language OS", 2)
}

pub fn kolibrios() -> IsoDistribution {
    simple_distro!("kolibri", "KolibriOS", DistributionCategory::Other, "http://kolibrios.org", "MenuetOS fork", 44)
}

pub fn morphos() -> IsoDistribution {
    simple_distro!("morphos", "MorphOS", DistributionCategory::Other, "https://www.morphos-team.net", "Amiga-compatible OS", 700)
}

pub fn aros() -> IsoDistribution {
    simple_distro!("aros", "AROS", DistributionCategory::Other, "http://aros.sourceforge.net", "AmigaOS compatible", 300)
}

pub fn genode() -> IsoDistribution {
    simple_distro!("genode", "Genode", DistributionCategory::Other, "https://genode.org", "Operating system framework", 400)
}

pub fn redox() -> IsoDistribution {
    simple_distro!("redox", "Redox OS", DistributionCategory::Other, "https://www.redox-os.org", "Unix-like OS written in Rust", 100)
}

pub fn serenity() -> IsoDistribution {
    simple_distro!("serenity", "SerenityOS", DistributionCategory::Other, "https://serenityos.org", "Unix-like graphical OS", 150)
}

pub fn temple_os() -> IsoDistribution {
    simple_distro!("templeos", "TempleOS", DistributionCategory::Other, "https://templeos.org", "Biblical-themed operating system", 3)
}
