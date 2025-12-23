use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use crate::ui::output::Output;
use crate::ui::prompt::Prompt;
use super::{UsbDevice, UsbOperation};
use super::device::DeviceDetector;
use super::writer::UsbWriter;

pub struct UsbWizard {
    output: Output,
    prompt: Prompt,
    detector: DeviceDetector,
}

impl UsbWizard {
    pub fn new(output: Output) -> Self {
        let emoji_enabled = output.emoji_enabled;
        Self {
            output,
            prompt: Prompt::new(emoji_enabled),
            detector: DeviceDetector::new(),
        }
    }

    /// Launch the interactive USB wizard
    pub async fn run(&self) -> Result<()> {
        self.output.section("USB Device Setup Wizard");

        loop {
            // Step 1: Device Selection
            let device = match self.select_device().await? {
                Some(device) => device,
                None => {
                    self.output.info("No USB devices selected. Exiting wizard.");
                    return Ok(());
                }
            };

            // Step 2: Operation Selection
            let operation = match self.select_operation(&device).await? {
                Some(op) => op,
                None => continue, // Go back to device selection
            };

            // Step 3: Execute Operation
            match self.execute_operation(operation, &device).await {
                Ok(_) => {
                    self.output.success("Operation completed successfully!");

                    if !self.prompt.confirm("Do you want to perform another USB operation?")? {
                        break;
                    }
                }
                Err(e) => {
                    self.output.error(&format!("Operation failed: {}", e));

                    if !self.prompt.confirm("Do you want to try again?")? {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Select a USB device
    async fn select_device(&self) -> Result<Option<UsbDevice>> {
        self.output.progress("Detecting USB devices...");

        let devices = self.detector.list_usb_devices()?;

        if devices.is_empty() {
            self.output.warn("No USB devices detected.");
            self.output.info("Please insert a USB device and try again.");
            return Ok(None);
        }

        // Display device list
        self.output.section("Detected USB Devices");

        for (i, device) in devices.iter().enumerate() {
            let status = if device.is_mounted {
                "mounted"
            } else {
                "unmounted"
            };

            let unknown = "unknown".to_string();
            let filesystem = device.filesystem.as_ref()
                .unwrap_or(&unknown);

            println!("{}. {} - {} ({}, {})",
                i + 1,
                device.path.display(),
                device.name,
                device.format_size(),
                filesystem
            );

            if device.is_mounted {
                for mount_point in &device.mount_points {
                    println!("   Mounted at: {}", mount_point.display());
                }
            }
        }

        println!();
        println!("R. Refresh device list");
        println!("Q. Quit wizard");
        println!();

        loop {
            let choice = self.prompt.input(&format!("Select device [1-{}]: ", devices.len()))?;

            match choice.to_lowercase().as_str() {
                "r" => return Box::pin(self.select_device()).await,
                "q" => return Ok(None),
                _ => {
                    if let Ok(index) = choice.parse::<usize>() {
                        if index > 0 && index <= devices.len() {
                            return Ok(Some(devices[index - 1].clone()));
                        }
                    }
                    self.output.warn("Invalid selection. Please try again.");
                }
            }
        }
    }

    /// Select operation to perform
    async fn select_operation(&self, device: &UsbDevice) -> Result<Option<UsbOperation>> {
        self.output.section("Select Operation");

        println!("Device: {} ({})", device.name, device.format_size());
        println!();
        println!("1. Write ISO to USB (single boot)");
        println!("2. Create multi-boot USB");
        println!("3. Add ISO to existing multi-boot USB");
        println!("4. Remove ISO from multi-boot USB");
        println!("5. List ISOs on multi-boot USB");
        println!("6. Erase USB device");
        println!("B. Back to device selection");
        println!("Q. Quit wizard");
        println!();

        loop {
            let choice = self.prompt.input("Select operation [1-6]: ")?;

            match choice.to_lowercase().as_str() {
                "1" => {
                    // Write single ISO
                    if let Some(iso_path) = self.select_iso().await? {
                        return Ok(Some(UsbOperation::WriteSingle {
                            iso_path,
                            device: device.path.clone(),
                        }));
                    }
                }
                "2" => {
                    // Create multi-boot
                    if !device.is_suitable_for_multi_boot() {
                        self.output.warn(&format!(
                            "Device is too small for multi-boot. Minimum 16 GB required, have {}.",
                            device.format_size()
                        ));
                        continue;
                    }

                    let initial_isos = self.select_multiple_isos().await?;
                    if !initial_isos.is_empty() {
                        return Ok(Some(UsbOperation::CreateMultiBoot {
                            device: device.path.clone(),
                            initial_isos,
                        }));
                    }
                }
                "3" => {
                    // Add to multi-boot
                    if let Some(iso_path) = self.select_iso().await? {
                        return Ok(Some(UsbOperation::AddToMultiBoot {
                            device: device.path.clone(),
                            iso_path,
                        }));
                    }
                }
                "4" => {
                    // Remove from multi-boot
                    let iso_name = self.prompt.input("Enter ISO name to remove: ")?;
                    return Ok(Some(UsbOperation::RemoveFromMultiBoot {
                        device: device.path.clone(),
                        iso_name,
                    }));
                }
                "5" => {
                    // List ISOs - this will be handled differently
                    self.list_multiboot_isos(&device.path).await?;
                }
                "6" => {
                    // Erase device
                    self.output.error(&format!(
                        "WARNING: This will PERMANENTLY ERASE all data on {}",
                        device.path.display()
                    ));

                    let confirm = self.prompt.input("Type 'YES' in capitals to confirm: ")?;
                    if confirm == "YES" {
                        return Ok(Some(UsbOperation::Erase {
                            device: device.path.clone(),
                        }));
                    } else {
                        self.output.info("Erase cancelled.");
                    }
                }
                "b" => return Ok(None),
                "q" => std::process::exit(0),
                _ => {
                    self.output.warn("Invalid selection. Please try again.");
                }
            }
        }
    }

    /// Select an ISO file
    async fn select_iso(&self) -> Result<Option<PathBuf>> {
        // First check for downloaded ISOs
        let iso_dir = dirs::home_dir()
            .map(|h| h.join("Downloads/ISOs"))
            .unwrap_or_else(|| PathBuf::from("/tmp"));

        if iso_dir.exists() {
            self.output.info(&format!("Looking for ISOs in {}", iso_dir.display()));

            let mut isos = Vec::new();

            // Recursively find all ISO files
            for entry in walkdir::WalkDir::new(&iso_dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().extension()
                    .and_then(|s| s.to_str())
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("iso"))
                {
                    isos.push(entry.path().to_path_buf());
                }
            }

            if !isos.is_empty() {
                self.output.section("Available ISO Files");

                for (i, iso) in isos.iter().enumerate() {
                    let name = iso.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");

                    let size = tokio::fs::metadata(iso).await
                        .map(|m| format_size(m.len()))
                        .unwrap_or_else(|_| "unknown".to_string());

                    println!("{}. {} ({})", i + 1, name, size);
                }

                println!();
                println!("M. Manual path entry");
                println!("D. Download new ISO");
                println!("C. Cancel");
                println!();

                loop {
                    let choice = self.prompt.input(&format!("Select ISO [1-{}]: ", isos.len()))?;

                    match choice.to_lowercase().as_str() {
                        "m" => break, // Fall through to manual entry
                        "d" => {
                            self.output.info("Use 'pkmgr iso install <distro>' to download ISOs");
                            return Ok(None);
                        }
                        "c" => return Ok(None),
                        _ => {
                            if let Ok(index) = choice.parse::<usize>() {
                                if index > 0 && index <= isos.len() {
                                    return Ok(Some(isos[index - 1].clone()));
                                }
                            }
                            self.output.warn("Invalid selection. Please try again.");
                        }
                    }
                }
            }
        }

        // Manual path entry
        let path = self.prompt.input("Enter ISO file path (or 'cancel'): ")?;

        if path.to_lowercase() == "cancel" {
            return Ok(None);
        }

        let iso_path = PathBuf::from(path);

        if !iso_path.exists() {
            self.output.error(&format!("File not found: {}", iso_path.display()));
            return Ok(None);
        }

        if iso_path.extension()
            .and_then(|s| s.to_str())
            .map_or(false, |ext| !ext.eq_ignore_ascii_case("iso"))
        {
            self.output.warn("File does not appear to be an ISO file.");
            if !self.prompt.confirm("Continue anyway?")? {
                return Ok(None);
            }
        }

        Ok(Some(iso_path))
    }

    /// Select multiple ISO files for multi-boot
    async fn select_multiple_isos(&self) -> Result<Vec<PathBuf>> {
        let mut selected: Vec<PathBuf> = Vec::new();

        self.output.info("Select ISOs to add to multi-boot USB (select 'done' when finished)");

        loop {
            if !selected.is_empty() {
                self.output.section("Selected ISOs");
                for (i, iso) in selected.iter().enumerate() {
                    let name = iso.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    println!("{}. {}", i + 1, name);
                }
                println!();
            }

            if let Some(iso) = self.select_iso().await? {
                selected.push(iso);

                if !self.prompt.confirm("Add another ISO?")? {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(selected)
    }

    /// Execute the selected operation
    async fn execute_operation(&self, operation: UsbOperation, device: &UsbDevice) -> Result<()> {
        // First unmount the device if needed
        if device.is_mounted {
            self.output.progress("Unmounting device...");
            self.detector.unmount_device(device)?;
        }

        let writer = UsbWriter::new(self.output.clone());

        match operation {
            UsbOperation::WriteSingle { iso_path, device: device_path } => {
                // Confirm operation
                self.output.warn(&format!(
                    "This will overwrite all data on {}",
                    device_path.display()
                ));

                if !self.prompt.confirm("Continue?")? {
                    return Ok(());
                }

                writer.write_iso(&iso_path, device, true).await?;

                self.output.info("Safely ejecting device...");
                self.detector.eject_device(device)?;
            }

            UsbOperation::CreateMultiBoot { device: device_path, initial_isos } => {
                self.output.warn(&format!(
                    "This will erase {} and create a multi-boot USB",
                    device_path.display()
                ));

                if !self.prompt.confirm("Continue?")? {
                    return Ok(());
                }

                // This would create the multi-boot structure
                self.create_multiboot_usb(device, initial_isos).await?;
            }

            UsbOperation::AddToMultiBoot { device: _, iso_path } => {
                self.output.info(&format!("Adding {} to multi-boot USB", iso_path.display()));
                // Implementation would go here
                self.output.warn("Multi-boot management not yet implemented");
            }

            UsbOperation::RemoveFromMultiBoot { device: _, iso_name } => {
                self.output.info(&format!("Removing {} from multi-boot USB", iso_name));
                // Implementation would go here
                self.output.warn("Multi-boot management not yet implemented");
            }

            UsbOperation::Erase { device: device_path } => {
                // Determine filesystem
                let filesystem = if device.size_gb() > 32.0 {
                    "exfat"
                } else {
                    "fat32"
                };

                writer.erase_device(device, filesystem).await?;
            }
        }

        Ok(())
    }

    /// Create a multi-boot USB
    async fn create_multiboot_usb(&self, device: &UsbDevice, initial_isos: Vec<PathBuf>) -> Result<()> {
        // This is a placeholder for the actual multi-boot creation logic
        // In a real implementation, this would:
        // 1. Format the device with appropriate filesystem
        // 2. Install bootloader (GRUB2 or Ventoy)
        // 3. Create directory structure
        // 4. Copy ISOs
        // 5. Generate boot configuration

        self.output.warn("Multi-boot USB creation not yet fully implemented");
        self.output.info("Would create multi-boot USB with:");
        for iso in initial_isos {
            let name = iso.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            self.output.info(&format!("  - {}", name));
        }

        Ok(())
    }

    /// List ISOs on a multi-boot USB
    async fn list_multiboot_isos(&self, device_path: &Path) -> Result<()> {
        self.output.section("Multi-boot USB Contents");
        self.output.warn("Multi-boot listing not yet implemented");
        self.output.info(&format!("Would list ISOs on {}", device_path.display()));
        Ok(())
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}