use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use super::{MultiBootConfig, BootEntry, BootloaderType};
use crate::iso::IsoDistribution;

pub struct BootloaderManager {
    bootloader_type: BootloaderType,
}

impl BootloaderManager {
    pub fn new(bootloader_type: BootloaderType) -> Self {
        Self { bootloader_type }
    }

    /// Install bootloader to USB device
    pub fn install_bootloader(&self, device: &Path) -> Result<()> {
        match self.bootloader_type {
            BootloaderType::Grub2 => self.install_grub2(device),
            BootloaderType::Syslinux => self.install_syslinux(device),
            BootloaderType::Ventoy => self.install_ventoy(device),
        }
    }

    /// Generate boot configuration
    pub fn generate_config(&self, usb_root: &Path, entries: &[BootEntry]) -> Result<()> {
        match self.bootloader_type {
            BootloaderType::Grub2 => self.generate_grub_config(usb_root, entries),
            BootloaderType::Syslinux => self.generate_syslinux_config(usb_root, entries),
            BootloaderType::Ventoy => Ok(()), // Ventoy auto-detects ISOs
        }
    }

    fn install_grub2(&self, device: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::process::Command;

            // Install GRUB to MBR
            let status = Command::new("grub-install")
                .arg("--target=i386-pc")
                .arg("--boot-directory=/mnt/usb/boot")
                .arg("--force")
                .arg(device)
                .status()
                .context("Failed to install GRUB")?;

            if !status.success() {
                bail!("GRUB installation failed");
            }

            // Install UEFI support if available
            let _ = Command::new("grub-install")
                .arg("--target=x86_64-efi")
                .arg("--efi-directory=/mnt/usb")
                .arg("--boot-directory=/mnt/usb/boot")
                .arg("--removable")
                .status();
        }

        Ok(())
    }

    fn install_syslinux(&self, device: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::process::Command;

            let status = Command::new("syslinux")
                .arg("--install")
                .arg(device)
                .status()
                .context("Failed to install Syslinux")?;

            if !status.success() {
                bail!("Syslinux installation failed");
            }
        }

        Ok(())
    }

    fn install_ventoy(&self, _device: &Path) -> Result<()> {
        // Ventoy requires downloading and running their installer
        // For now, we'll just note this
        bail!("Ventoy installation requires manual setup. Download from ventoy.net");
    }

    fn generate_grub_config(&self, usb_root: &Path, entries: &[BootEntry]) -> Result<()> {
        let grub_cfg_path = usb_root.join("boot/grub/grub.cfg");

        // Ensure directory exists
        fs::create_dir_all(grub_cfg_path.parent().unwrap())?;

        let mut config = String::new();

        // GRUB configuration header
        config.push_str("# pkmgr Multi-boot USB Configuration\n");
        config.push_str("# Generated automatically - do not edit\n\n");

        config.push_str("set timeout=10\n");
        config.push_str("set default=0\n");
        config.push_str("set gfxmode=auto\n");
        config.push_str("insmod all_video\n");
        config.push_str("terminal_output gfxterm\n\n");

        // Theme configuration
        config.push_str("if [ -f /boot/grub/themes/pkmgr/theme.txt ]; then\n");
        config.push_str("    set theme=/boot/grub/themes/pkmgr/theme.txt\n");
        config.push_str("fi\n\n");

        // Group entries by category
        let mut categories = std::collections::HashMap::new();
        for entry in entries {
            categories.entry(entry.category.clone())
                .or_insert_with(Vec::new)
                .push(entry);
        }

        // Main menu
        for (category, entries) in categories.iter() {
            config.push_str(&format!("submenu '{}' {{\n", category));

            for entry in entries {
                config.push_str(&self.generate_grub_entry(entry)?);
            }

            config.push_str("}\n\n");
        }

        // Utility entries
        config.push_str("submenu 'System Tools' {\n");
        config.push_str("    menuentry 'Memory Test (Memtest86+)' {\n");
        config.push_str("        linux16 /boot/memtest86+\n");
        config.push_str("    }\n\n");

        config.push_str("    menuentry 'Hardware Detection Tool' {\n");
        config.push_str("        linux /boot/hdt/hdt.c32\n");
        config.push_str("    }\n\n");

        config.push_str("    menuentry 'GRUB Command Line' {\n");
        config.push_str("        commandline\n");
        config.push_str("    }\n\n");

        config.push_str("    menuentry 'Reboot' {\n");
        config.push_str("        reboot\n");
        config.push_str("    }\n\n");

        config.push_str("    menuentry 'Power Off' {\n");
        config.push_str("        halt\n");
        config.push_str("    }\n");
        config.push_str("}\n");

        // Write configuration
        let mut file = fs::File::create(&grub_cfg_path)?;
        file.write_all(config.as_bytes())?;

        Ok(())
    }

    fn generate_grub_entry(&self, entry: &BootEntry) -> Result<String> {
        let mut config = String::new();

        config.push_str(&format!("    menuentry '{}' {{\n", entry.display_name));
        config.push_str(&format!("        set isofile=\"{}\"\n", entry.iso_path));
        config.push_str("        loopback loop $isofile\n");

        // Distribution-specific boot parameters
        let boot_params = self.get_boot_params(&entry.name, &entry.version);

        match entry.name.as_str() {
            "ubuntu" | "debian" | "mint" => {
                config.push_str("        linux (loop)/casper/vmlinuz boot=casper ");
                config.push_str(&format!("iso-scan/filename=$isofile {}\n", boot_params));
                config.push_str("        initrd (loop)/casper/initrd\n");
            }

            "fedora" | "centos" | "rocky" | "alma" => {
                config.push_str("        linux (loop)/isolinux/vmlinuz inst.stage2=hd:LABEL=MULTIBOOT ");
                config.push_str(&format!("iso-scan/filename=$isofile {}\n", boot_params));
                config.push_str("        initrd (loop)/isolinux/initrd.img\n");
            }

            "arch" | "manjaro" => {
                config.push_str("        linux (loop)/arch/boot/x86_64/vmlinuz-linux ");
                config.push_str("archisodevice=/dev/loop0 img_dev=/dev/disk/by-label/MULTIBOOT ");
                config.push_str(&format!("img_loop=$isofile {}\n", boot_params));
                config.push_str("        initrd (loop)/arch/boot/x86_64/initramfs-linux.img\n");
            }

            "kali" | "parrot" => {
                config.push_str("        linux (loop)/live/vmlinuz boot=live ");
                config.push_str(&format!("findiso=$isofile {}\n", boot_params));
                config.push_str("        initrd (loop)/live/initrd.img\n");
            }

            _ => {
                // Generic fallback
                config.push_str("        linux (loop)/vmlinuz ");
                config.push_str(&format!("iso-scan/filename=$isofile {}\n", boot_params));
                config.push_str("        initrd (loop)/initrd.img\n");
            }
        }

        config.push_str("    }\n\n");

        Ok(config)
    }

    fn get_boot_params(&self, distro: &str, _version: &str) -> String {
        match distro {
            "ubuntu" | "debian" | "mint" => "quiet splash",
            "fedora" | "centos" | "rocky" | "alma" => "quiet",
            "arch" | "manjaro" => "quiet splash",
            "kali" | "parrot" => "quiet splash apparmor=0",
            "tails" => "quiet splash noautologin",
            _ => "quiet",
        }.to_string()
    }

    fn generate_syslinux_config(&self, usb_root: &Path, entries: &[BootEntry]) -> Result<()> {
        let cfg_path = usb_root.join("syslinux/syslinux.cfg");

        fs::create_dir_all(cfg_path.parent().unwrap())?;

        let mut config = String::new();

        config.push_str("# pkmgr Multi-boot USB Configuration\n");
        config.push_str("DEFAULT menu.c32\n");
        config.push_str("PROMPT 0\n");
        config.push_str("TIMEOUT 100\n");
        config.push_str("MENU TITLE pkmgr Multi-boot USB\n\n");

        for entry in entries {
            config.push_str(&format!("LABEL {}\n", entry.name.replace(' ', "_")));
            config.push_str(&format!("    MENU LABEL {}\n", entry.display_name));
            config.push_str(&format!("    KERNEL memdisk\n"));
            config.push_str(&format!("    INITRD {}\n", entry.iso_path));
            config.push_str("    APPEND iso\n\n");
        }

        let mut file = fs::File::create(&cfg_path)?;
        file.write_all(config.as_bytes())?;

        Ok(())
    }

    /// Create directory structure for multi-boot USB
    pub fn create_directory_structure(&self, usb_root: &Path) -> Result<()> {
        let dirs = vec![
            "boot",
            "boot/grub",
            "boot/grub/themes",
            "boot/grub/themes/pkmgr",
            "boot/syslinux",
            "isos",
            "isos/OS",
            "isos/OS/Linux",
            "isos/OS/Windows",
            "isos/OS/BSD",
            "isos/Security",
            "isos/Tools",
            "isos/Server",
            "persistence",
        ];

        for dir in dirs {
            let error_msg = format!("Failed to create directory: {}", dir);
            fs::create_dir_all(usb_root.join(dir))
                .context(error_msg)?;
        }

        // Create info file
        let info_path = usb_root.join("pkmgr-multiboot.txt");
        let mut info_file = fs::File::create(info_path)?;
        info_file.write_all(b"This is a pkmgr multi-boot USB drive\n")?;
        info_file.write_all(b"Created by CasjaysDev Package Manager\n")?;
        info_file.write_all(b"https://github.com/pkmgr/pkmgr\n")?;

        Ok(())
    }
}

/// Determine the appropriate category for an ISO
pub fn categorize_iso(name: &str) -> String {
    let name_lower = name.to_lowercase();

    if name_lower.contains("kali") ||
       name_lower.contains("parrot") ||
       name_lower.contains("blackarch") ||
       name_lower.contains("tails") {
        return "Security Tools".to_string();
    }

    if name_lower.contains("ubuntu") ||
       name_lower.contains("debian") ||
       name_lower.contains("fedora") ||
       name_lower.contains("arch") ||
       name_lower.contains("manjaro") ||
       name_lower.contains("opensuse") ||
       name_lower.contains("mint") {
        return "Linux Distributions".to_string();
    }

    if name_lower.contains("proxmox") ||
       name_lower.contains("truenas") ||
       name_lower.contains("pfsense") ||
       name_lower.contains("opnsense") {
        return "Server Systems".to_string();
    }

    if name_lower.contains("freebsd") ||
       name_lower.contains("openbsd") ||
       name_lower.contains("netbsd") {
        return "BSD Systems".to_string();
    }

    if name_lower.contains("gparted") ||
       name_lower.contains("clonezilla") ||
       name_lower.contains("systemrescue") ||
       name_lower.contains("memtest") {
        return "System Tools".to_string();
    }

    "Other".to_string()
}