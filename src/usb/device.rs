use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use super::{UsbDevice, UsbPartition};

pub struct DeviceDetector;

impl DeviceDetector {
    pub fn new() -> Self {
        Self {}
    }

    /// List all USB devices that are safe to use
    pub fn list_usb_devices(&self) -> Result<Vec<UsbDevice>> {
        #[cfg(target_os = "linux")]
        return self.list_usb_devices_linux();

        #[cfg(target_os = "macos")]
        return self.list_usb_devices_macos();

        #[cfg(target_os = "windows")]
        return self.list_usb_devices_windows();

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        bail!("USB device detection not supported on this platform");
    }

    #[cfg(target_os = "linux")]
    fn list_usb_devices_linux(&self) -> Result<Vec<UsbDevice>> {
        let mut devices = Vec::new();

        // Read block devices from /sys/block
        let block_dir = Path::new("/sys/block");
        for entry in fs::read_dir(block_dir)? {
            let entry = entry?;
            let device_name = entry.file_name();
            let device_name = device_name.to_string_lossy();

            // Skip non-USB devices
            if device_name.starts_with("loop") ||
               device_name.starts_with("dm-") ||
               device_name.starts_with("md") ||
               device_name.starts_with("ram") {
                continue;
            }

            let device_path = entry.path();

            // Check if device is removable
            let removable_path = device_path.join("removable");
            if let Ok(removable) = fs::read_to_string(&removable_path) {
                if removable.trim() != "1" {
                    continue; // Not removable, skip
                }
            } else {
                continue; // Can't determine if removable, skip for safety
            }

            // Get device info
            let dev_path = PathBuf::from(format!("/dev/{}", device_name));

            // Check if it's actually a USB device
            if !self.is_usb_device_linux(&device_path)? {
                continue;
            }

            // Get device size
            let size_path = device_path.join("size");
            let size_blocks: u64 = fs::read_to_string(&size_path)?
                .trim()
                .parse()
                .unwrap_or(0);
            let size_bytes = size_blocks * 512;

            // Skip devices that are too small or too large (likely not USB sticks)
            if size_bytes < 1_000_000_000 || size_bytes > 2_000_000_000_000 {
                continue; // Less than 1GB or more than 2TB
            }

            // Get vendor and model
            let vendor = fs::read_to_string(device_path.join("device/vendor"))
                .ok()
                .map(|s| s.trim().to_string());

            let model = fs::read_to_string(device_path.join("device/model"))
                .ok()
                .map(|s| s.trim().to_string());

            // Get filesystem and mount info using lsblk
            let (filesystem, is_mounted, mount_points) = self.get_device_info_lsblk(&dev_path)?;

            // Get partitions
            let partitions = self.get_partitions_linux(&dev_path)?;

            let device = UsbDevice {
                path: dev_path,
                name: format!("{} {}",
                    vendor.as_ref().unwrap_or(&"Unknown".to_string()),
                    model.as_ref().unwrap_or(&device_name.to_string())
                ),
                size_bytes,
                size_display: format_size(size_bytes),
                filesystem,
                vendor,
                model,
                is_removable: true,
                is_mounted,
                mount_points,
                partitions,
            };

            devices.push(device);
        }

        Ok(devices)
    }

    #[cfg(target_os = "linux")]
    fn is_usb_device_linux(&self, device_path: &Path) -> Result<bool> {
        // Check if the device is connected via USB by traversing the sysfs tree
        let mut current = device_path.to_path_buf();

        for _ in 0..10 { // Limit traversal depth
            if current.join("subsystem").exists() {
                if let Ok(subsystem) = fs::read_link(current.join("subsystem")) {
                    if subsystem.to_string_lossy().contains("usb") {
                        return Ok(true);
                    }
                }
            }

            // Move up to parent device
            current = match current.parent() {
                Some(parent) if parent != Path::new("/sys") => parent.to_path_buf(),
                _ => break,
            };
        }

        Ok(false)
    }

    #[cfg(target_os = "linux")]
    fn get_device_info_lsblk(&self, device_path: &Path) -> Result<(Option<String>, bool, Vec<PathBuf>)> {
        let output = Command::new("lsblk")
            .arg("-J") // JSON output
            .arg("-o")
            .arg("NAME,FSTYPE,MOUNTPOINT")
            .arg(device_path)
            .output()?;

        if !output.status.success() {
            return Ok((None, false, Vec::new()));
        }

        // Parse JSON output
        let json_str = String::from_utf8_lossy(&output.stdout);

        // Simple parsing without serde_json dependency
        let mut filesystem = None;
        let mut mount_points = Vec::new();

        // Extract filesystem type and mount points from JSON
        for line in json_str.lines() {
            if line.contains("\"fstype\"") {
                if let Some(fs) = line.split('"').nth(3) {
                    if !fs.is_empty() {
                        filesystem = Some(fs.to_string());
                    }
                }
            }
            if line.contains("\"mountpoint\"") {
                if let Some(mount) = line.split('"').nth(3) {
                    if !mount.is_empty() && mount != "null" {
                        mount_points.push(PathBuf::from(mount));
                    }
                }
            }
        }

        let is_mounted = !mount_points.is_empty();

        Ok((filesystem, is_mounted, mount_points))
    }

    #[cfg(target_os = "linux")]
    fn get_partitions_linux(&self, device_path: &Path) -> Result<Vec<UsbPartition>> {
        let mut partitions = Vec::new();
        let device_name = device_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Look for partition devices
        for i in 1..=16 {
            let partition_path = PathBuf::from(format!("{}{}", device_path.display(), i));
            if !partition_path.exists() {
                // Try with 'p' separator (for nvme, mmc devices)
                let partition_path = PathBuf::from(format!("{}p{}", device_path.display(), i));
                if !partition_path.exists() {
                    continue;
                }
            }

            // Get partition info
            let (filesystem, _, mount_point) = self.get_device_info_lsblk(&partition_path)?;

            // Get partition size from sysfs
            let size_path = format!("/sys/block/{}/{}{}/size", device_name, device_name, i);
            let size_bytes = if let Ok(size_str) = fs::read_to_string(&size_path) {
                size_str.trim().parse::<u64>().unwrap_or(0) * 512
            } else {
                0
            };

            partitions.push(UsbPartition {
                path: partition_path,
                number: i as u32,
                size_bytes,
                filesystem,
                label: None, // Could be retrieved with blkid
                uuid: None,  // Could be retrieved with blkid
                mount_point: mount_point.first().cloned(),
            });
        }

        Ok(partitions)
    }

    #[cfg(target_os = "macos")]
    fn list_usb_devices_macos(&self) -> Result<Vec<UsbDevice>> {
        let mut devices = Vec::new();

        // Use diskutil to list devices
        let output = Command::new("diskutil")
            .arg("list")
            .arg("-plist")
            .output()?;

        if !output.status.success() {
            bail!("Failed to list disks using diskutil");
        }

        // Parse diskutil output (simplified - would need proper plist parsing)
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Get detailed info for each disk
        for disk_num in 0..20 {
            let disk_path = format!("/dev/disk{}", disk_num);
            if !Path::new(&disk_path).exists() {
                continue;
            }

            // Check if it's removable and external
            let info_output = Command::new("diskutil")
                .arg("info")
                .arg(&disk_path)
                .output()?;

            if !info_output.status.success() {
                continue;
            }

            let info_str = String::from_utf8_lossy(&info_output.stdout);

            // Check if removable and external
            if !info_str.contains("Removable Media: Yes") {
                continue;
            }

            // Extract device info from output
            let size_bytes = self.extract_size_from_diskutil(&info_str)?;
            let name = self.extract_name_from_diskutil(&info_str)?;

            let device = UsbDevice {
                path: PathBuf::from(disk_path),
                name,
                size_bytes,
                size_display: format_size(size_bytes),
                filesystem: None,
                vendor: None,
                model: None,
                is_removable: true,
                is_mounted: false,
                mount_points: Vec::new(),
                partitions: Vec::new(),
            };

            devices.push(device);
        }

        Ok(devices)
    }

    #[cfg(target_os = "macos")]
    fn extract_size_from_diskutil(&self, output: &str) -> Result<u64> {
        for line in output.lines() {
            if line.contains("Disk Size:") {
                // Extract size in bytes from line like:
                // "Disk Size: 32.0 GB (32010928128 Bytes)"
                if let Some(start) = line.find('(') {
                    if let Some(end) = line.find(" Bytes") {
                        let size_str = &line[start+1..end];
                        return size_str.parse().context("Failed to parse disk size");
                    }
                }
            }
        }
        Ok(0)
    }

    #[cfg(target_os = "macos")]
    fn extract_name_from_diskutil(&self, output: &str) -> Result<String> {
        for line in output.lines() {
            if line.contains("Device / Media Name:") {
                return Ok(line.split(':').nth(1)
                    .unwrap_or("Unknown")
                    .trim()
                    .to_string());
            }
        }
        Ok("Unknown Device".to_string())
    }

    #[cfg(target_os = "windows")]
    fn list_usb_devices_windows(&self) -> Result<Vec<UsbDevice>> {
        // Windows implementation would use WMI or PowerShell
        // For now, return empty list
        Ok(Vec::new())
    }

    /// Unmount a USB device
    pub fn unmount_device(&self, device: &UsbDevice) -> Result<()> {
        if !device.is_mounted {
            return Ok(());
        }

        #[cfg(target_os = "linux")]
        {
            for mount_point in &device.mount_points {
                Command::new("umount")
                    .arg(mount_point)
                    .status()
                    .context("Failed to unmount device")?;
            }
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("diskutil")
                .arg("unmountDisk")
                .arg(&device.path)
                .status()
                .context("Failed to unmount device")?;
        }

        Ok(())
    }

    /// Eject a USB device safely
    pub fn eject_device(&self, device: &UsbDevice) -> Result<()> {
        self.unmount_device(device)?;

        #[cfg(target_os = "linux")]
        {
            Command::new("eject")
                .arg(&device.path)
                .status()
                .context("Failed to eject device")?;
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("diskutil")
                .arg("eject")
                .arg(&device.path)
                .status()
                .context("Failed to eject device")?;
        }

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