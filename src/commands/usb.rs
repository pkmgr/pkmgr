use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;
use crate::commands::Cli;
use crate::core::config::Config;
use crate::ui::output::Output;

#[derive(Debug, Subcommand, Clone)]
pub enum UsbCommands {
    /// Launch interactive USB wizard
    Interactive,
    /// List USB devices
    List,
    /// Completely wipe USB device
    Erase {
        device: String,
        #[arg(long, default_value = "auto")]
        filesystem: String,
    },
    /// Write single ISO to USB (dd-style)
    Write {
        iso_file: String,
        device: String,
        #[arg(long)]
        no_verify: bool,
    },
    /// Create or manage multi-boot USB
    #[command(subcommand)]
    Boot(BootCommands),
}

#[derive(Debug, Subcommand, Clone)]
pub enum BootCommands {
    /// Create new multi-boot USB
    Create {
        device: String,
        #[arg(long)]
        isos: Vec<String>,
        #[arg(long, default_value = "grub2")]
        bootloader: String,
    },
    /// Add ISO to multi-boot USB
    Add {
        iso_or_distro: String,
        #[arg(long)]
        device: Option<String>,
    },
    /// Remove ISO from multi-boot USB
    Remove {
        iso_or_distro: String,
        #[arg(long)]
        device: Option<String>,
    },
    /// List ISOs on multi-boot USB
    List {
        device: Option<String>,
    },
    /// Clean up old/duplicate ISOs from USB
    Clean {
        device: Option<String>,
    },
}

pub async fn execute(cmd: UsbCommands, cli: &Cli, config: &Config, output: &Output) -> Result<()> {
    // Check if usb module is available
    #[cfg(feature = "usb")]
    {
        use crate::usb::{device::DeviceDetector, wizard::UsbWizard, writer::UsbWriter};

        match cmd {
            UsbCommands::Interactive => {
                let wizard = UsbWizard::new(output.clone());
                wizard.run().await?;
            }
            UsbCommands::List => {
                list_usb_devices(output)?;
            }
            UsbCommands::Erase { device, filesystem } => {
                erase_device(&device, &filesystem, output).await?;
            }
            UsbCommands::Write { iso_file, device, no_verify } => {
                write_iso(&iso_file, &device, !no_verify, output).await?;
            }
            UsbCommands::Boot(boot_cmd) => {
                handle_boot_command(boot_cmd, output)?;
            }
        }
    }

    #[cfg(not(feature = "usb"))]
    {
        match cmd {
            UsbCommands::Interactive => {
                output.info("💾 USB Interactive Wizard");
                output.warn("USB support not compiled in");
            }
            UsbCommands::List => {
                output.info("Would list USB devices");
                output.warn("USB support not compiled in");
            }
            UsbCommands::Erase { device, filesystem } => {
                output.info(&format!("🔥 Would erase USB device: {} with {}", device, filesystem));
                output.warn("USB support not compiled in");
            }
            UsbCommands::Write { iso_file, device, no_verify } => {
                output.info(&format!("💿 Would write {} to {}", iso_file, device));
                output.warn("USB support not compiled in");
            }
            UsbCommands::Boot(boot_cmd) => {
                output.info("🛠️ Multi-boot USB management");
                output.warn("USB support not compiled in");
            }
        }
    }

    Ok(())
}

#[cfg(feature = "usb")]
fn list_usb_devices(output: &Output) -> Result<()> {
    use crate::usb::device::DeviceDetector;

    output.section("USB Devices");
    output.progress("Detecting USB devices...");

    let detector = DeviceDetector::new();
    let devices = detector.list_usb_devices()?;

    if devices.is_empty() {
        output.warn("No USB devices detected");
        return Ok(());
    }

    for device in devices {
        let status = if device.is_mounted { "mounted" } else { "unmounted" };
        let fs = device.filesystem.as_ref().unwrap_or(&"unknown".to_string());

        output.info(&format!("{} - {} ({}, {}, {})",
            device.path.display(),
            device.name,
            device.format_size(),
            fs,
            status
        ));
    }

    Ok(())
}

#[cfg(feature = "usb")]
fn handle_boot_command(cmd: BootCommands, output: &Output) -> Result<()> {
    match cmd {
        BootCommands::Create { device, isos, bootloader } => {
            output.info(&format!("Creating multi-boot USB on {} with {}", device, bootloader));
            for iso in isos {
                output.info(&format!("  - {}", iso));
            }
            output.warn("Multi-boot creation pending implementation");
        }
        BootCommands::Add { iso_or_distro, device } => {
            output.info(&format!("Adding {} to multi-boot USB", iso_or_distro));
            output.warn("Multi-boot add pending implementation");
        }
        BootCommands::Remove { iso_or_distro, device } => {
            output.info(&format!("Removing {} from multi-boot USB", iso_or_distro));
            output.warn("Multi-boot remove pending implementation");
        }
        BootCommands::List { device } => {
            output.info("Listing multi-boot USB contents");
            output.warn("Multi-boot list pending implementation");
        }
        BootCommands::Clean { device } => {
            output.info("Cleaning multi-boot USB");
            output.warn("Multi-boot clean pending implementation");
        }
    }
    Ok(())
}

#[cfg(feature = "usb")]
async fn erase_device(device_path: &str, filesystem: &str, output: &Output) -> Result<()> {
    use crate::usb::device::DeviceDetector;
    use crate::usb::writer::UsbWriter;
    use std::path::PathBuf;

    output.print_header(&format!("🔥 Erasing USB Device: {}", device_path));

    let detector = DeviceDetector::new();
    let devices = detector.list_usb_devices()?;

    let device_pathbuf = PathBuf::from(device_path);
    let device = devices.iter()
        .find(|d| d.path == device_pathbuf)
        .ok_or_else(|| anyhow::anyhow!("Device {} not found", device_path))?;

    // Confirm with user
    use crate::ui::prompt::Prompt;
    let prompt = Prompt::new(output.emoji_enabled);

    output.warn(&format!(
        "This will PERMANENTLY ERASE all data on {} ({} - {})",
        device.path.display(),
        device.name,
        device.format_size()
    ));

    if !prompt.confirm_dangerous("Type 'YES' in capitals to proceed")? {
        output.info("Operation cancelled");
        return Ok(());
    }

    let writer = UsbWriter::new(output.clone());
    writer.erase_device(device, filesystem).await?;

    Ok(())
}

#[cfg(feature = "usb")]
async fn write_iso(iso_file: &str, device_path: &str, verify: bool, output: &Output) -> Result<()> {
    use crate::usb::device::DeviceDetector;
    use crate::usb::writer::UsbWriter;
    use std::path::{Path, PathBuf};

    output.print_header(&format!("💿 Writing ISO to USB Device"));

    // Check ISO exists
    let iso_path = Path::new(iso_file);
    if !iso_path.exists() {
        anyhow::bail!("ISO file '{}' not found", iso_file);
    }

    // Find device
    let detector = DeviceDetector::new();
    let devices = detector.list_usb_devices()?;

    let device_pathbuf = PathBuf::from(device_path);
    let device = devices.iter()
        .find(|d| d.path == device_pathbuf)
        .ok_or_else(|| anyhow::anyhow!("Device {} not found", device_path))?;

    // Confirm with user
    use crate::ui::prompt::Prompt;
    let prompt = Prompt::new(output.emoji_enabled);

    output.warn(&format!(
        "This will overwrite all data on {} ({} - {})",
        device.path.display(),
        device.name,
        device.format_size()
    ));

    if !prompt.confirm("Continue?")? {
        output.info("Operation cancelled");
        return Ok(());
    }

    let writer = UsbWriter::new(output.clone());
    writer.write_iso(iso_path, device, verify).await?;

    Ok(())
}
