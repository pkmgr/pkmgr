use anyhow::{Context, Result, bail};
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Instant, Duration};
use crate::ui::output::Output;
use super::UsbDevice;

pub struct UsbWriter {
    output: Output,
}

impl UsbWriter {
    pub fn new(output: Output) -> Self {
        Self { output }
    }

    /// Write an ISO to a USB device (dd-style)
    pub async fn write_iso(&self, iso_path: &Path, device: &UsbDevice, verify: bool) -> Result<()> {
        // Safety checks
        if !device.is_removable {
            bail!("Device {} is not removable. Refusing to write for safety.", device.path.display());
        }

        if !super::is_device_safe(&device.path)? {
            bail!("Device {} is not safe to write to.", device.path.display());
        }

        // Check ISO exists and get size
        let iso_metadata = tokio::fs::metadata(iso_path).await
            .context("Failed to read ISO file")?;
        let iso_size = iso_metadata.len();

        // Check device has enough space
        if iso_size > device.size_bytes {
            bail!(
                "ISO size ({}) exceeds device capacity ({})",
                format_size(iso_size),
                format_size(device.size_bytes)
            );
        }

        self.output.info(&format!(
            "Writing {} ({}) to {}",
            iso_path.display(),
            format_size(iso_size),
            device.path.display()
        ));

        // Progress tracking
        let bytes_written = Arc::new(AtomicU64::new(0));
        let should_stop = Arc::new(AtomicBool::new(false));

        // Start progress display task
        let (progress_tx, mut progress_rx) = mpsc::channel::<ProgressUpdate>(32);

        let progress_handle = {
            let output = self.output.clone();
            let total_bytes = iso_size;

            tokio::spawn(async move {
                let mut last_update = Instant::now();
                let mut last_bytes = 0u64;

                while let Some(update) = progress_rx.recv().await {
                    match update {
                        ProgressUpdate::Progress { bytes, force } => {
                            let now = Instant::now();
                            if force || now.duration_since(last_update) >= Duration::from_millis(100) {
                                let speed = if last_update.elapsed().as_secs() > 0 {
                                    (bytes - last_bytes) as f64 / last_update.elapsed().as_secs_f64()
                                } else {
                                    0.0
                                };

                                let percent = (bytes as f64 / total_bytes as f64 * 100.0) as u32;
                                let eta = if speed > 0.0 {
                                    Some(((total_bytes - bytes) as f64 / speed) as u64)
                                } else {
                                    None
                                };

                                output.progress(&format!(
                                    "Writing: {}% ({}/{}) | {} | {}",
                                    percent,
                                    format_size(bytes),
                                    format_size(total_bytes),
                                    format_speed(speed),
                                    eta.map_or("calculating...".to_string(), format_eta)
                                ));

                                last_update = now;
                                last_bytes = bytes;
                            }
                        }
                        ProgressUpdate::Complete => break,
                        ProgressUpdate::Error(msg) => {
                            output.error(&msg);
                            break;
                        }
                    }
                }
            })
        };

        // Perform the actual write in a blocking task
        let iso_path = iso_path.to_path_buf();
        let iso_path_for_verify = iso_path.clone();
        let device_path = device.path.clone();
        let bytes_written_clone = bytes_written.clone();
        let should_stop_clone = should_stop.clone();
        let progress_tx_clone = progress_tx.clone();

        let write_result = tokio::task::spawn_blocking(move || {
            write_iso_blocking(
                &iso_path,
                &device_path,
                bytes_written_clone,
                should_stop_clone,
                progress_tx_clone,
            )
        }).await?;

        // Signal completion
        let _ = progress_tx.send(ProgressUpdate::Complete).await;
        progress_handle.await?;

        write_result?;

        // Sync to ensure all data is written
        self.output.progress("Syncing data to device...");
        #[cfg(unix)]
        {
            use std::process::Command;
            Command::new("sync")
                .status()
                .context("Failed to sync data")?;
        }

        if verify {
            self.output.progress("Verifying written data...");
            self.verify_write(&iso_path_for_verify, &device.path, iso_size).await?;
            self.output.success("Verification complete");
        }

        self.output.success(&format!(
            "Successfully wrote ISO to {}",
            device.path.display()
        ));

        Ok(())
    }

    /// Verify that the ISO was written correctly
    async fn verify_write(&self, iso_path: &Path, device_path: &Path, size: u64) -> Result<()> {
        use sha2::{Sha256, Digest};

        let iso_path = iso_path.to_path_buf();
        let device_path = device_path.to_path_buf();

        // Calculate checksums in parallel
        let iso_checksum_task = tokio::task::spawn_blocking(move || {
            calculate_checksum(&iso_path, size)
        });

        let device_checksum_task = tokio::task::spawn_blocking(move || {
            calculate_checksum(&device_path, size)
        });

        let iso_checksum = iso_checksum_task.await??;
        let device_checksum = device_checksum_task.await??;

        if iso_checksum != device_checksum {
            bail!("Verification failed: Checksums do not match");
        }

        Ok(())
    }

    /// Erase a USB device completely
    pub async fn erase_device(&self, device: &UsbDevice, filesystem: &str) -> Result<()> {
        if !device.is_removable {
            bail!("Device {} is not removable. Refusing to erase for safety.", device.path.display());
        }

        if !super::is_device_safe(&device.path)? {
            bail!("Device {} is not safe to erase.", device.path.display());
        }

        self.output.warn(&format!(
            "Erasing {} will permanently delete all data!",
            device.path.display()
        ));

        // Zero out the first and last MB to clear partition tables
        self.output.progress("Wiping partition tables...");

        let device_path = device.path.clone();
        let size = device.size_bytes;

        tokio::task::spawn_blocking(move || {
            wipe_device_headers(&device_path, size)
        }).await??;

        // Create new filesystem
        self.output.progress(&format!("Creating {} filesystem...", filesystem.to_uppercase()));

        #[cfg(unix)]
        {
            use std::process::Command;

            let mkfs_cmd = match filesystem {
                "fat32" | "vfat" => "mkfs.vfat",
                "exfat" => "mkfs.exfat",
                "ext4" => "mkfs.ext4",
                "ntfs" => "mkfs.ntfs",
                _ => bail!("Unsupported filesystem: {}", filesystem),
            };

            let status = Command::new(mkfs_cmd)
                .arg("-F") // Force
                .arg(&device.path)
                .status()
                .context("Failed to create filesystem")?;

            if !status.success() {
                bail!("Failed to create {} filesystem", filesystem);
            }
        }

        self.output.success(&format!(
            "Successfully erased and formatted {} as {}",
            device.path.display(),
            filesystem.to_uppercase()
        ));

        Ok(())
    }
}

enum ProgressUpdate {
    Progress { bytes: u64, force: bool },
    Complete,
    Error(String),
}

fn write_iso_blocking(
    iso_path: &Path,
    device_path: &Path,
    bytes_written: Arc<AtomicU64>,
    should_stop: Arc<AtomicBool>,
    progress_tx: mpsc::Sender<ProgressUpdate>,
) -> Result<()> {
    let iso_file = File::open(iso_path)
        .context("Failed to open ISO file")?;
    let mut reader = BufReader::with_capacity(4 * 1024 * 1024, iso_file); // 4MB buffer

    let device_file = OpenOptions::new()
        .write(true)
        .open(device_path)
        .context("Failed to open USB device for writing")?;
    let mut writer = BufWriter::with_capacity(4 * 1024 * 1024, device_file); // 4MB buffer

    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB chunks
    let mut total_written = 0u64;
    let mut last_progress_update = Instant::now();

    loop {
        if should_stop.load(Ordering::Relaxed) {
            bail!("Write operation cancelled");
        }

        let bytes_read = reader.read(&mut buffer)
            .context("Failed to read from ISO")?;

        if bytes_read == 0 {
            break; // EOF
        }

        writer.write_all(&buffer[..bytes_read])
            .context("Failed to write to USB device")?;

        total_written += bytes_read as u64;
        bytes_written.store(total_written, Ordering::Relaxed);

        // Send progress update
        if last_progress_update.elapsed() >= Duration::from_millis(100) {
            let _ = progress_tx.blocking_send(ProgressUpdate::Progress {
                bytes: total_written,
                force: false,
            });
            last_progress_update = Instant::now();
        }
    }

    // Flush remaining data
    writer.flush()
        .context("Failed to flush data to USB device")?;

    // Send final progress
    let _ = progress_tx.blocking_send(ProgressUpdate::Progress {
        bytes: total_written,
        force: true,
    });

    Ok(())
}

fn wipe_device_headers(device_path: &Path, device_size: u64) -> Result<()> {
    let mut device = OpenOptions::new()
        .write(true)
        .open(device_path)
        .context("Failed to open device for wiping")?;

    // Zero out first 1MB
    let zeros = vec![0u8; 1024 * 1024];
    device.write_all(&zeros)
        .context("Failed to wipe device header")?;

    // Zero out last 1MB
    use std::io::Seek;
    if device_size > 1024 * 1024 {
        device.seek(std::io::SeekFrom::End(-(1024 * 1024)))
            .context("Failed to seek to end of device")?;
        device.write_all(&zeros)
            .context("Failed to wipe device footer")?;
    }

    device.flush()
        .context("Failed to flush device")?;

    Ok(())
}

fn calculate_checksum(path: &Path, size: u64) -> Result<String> {
    use sha2::{Sha256, Digest};

    let mut file = File::open(path)
        .context("Failed to open file for checksum")?;

    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer
    let mut total_read = 0u64;

    while total_read < size {
        let to_read = std::cmp::min(buffer.len(), (size - total_read) as usize);
        let bytes_read = file.read(&mut buffer[..to_read])
            .context("Failed to read file for checksum")?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
        total_read += bytes_read as u64;
    }

    Ok(format!("{:x}", hasher.finalize()))
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

fn format_speed(bytes_per_second: f64) -> String {
    if bytes_per_second < 1024.0 {
        format!("{:.0} B/s", bytes_per_second)
    } else if bytes_per_second < 1024.0 * 1024.0 {
        format!("{:.1} KB/s", bytes_per_second / 1024.0)
    } else if bytes_per_second < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1} MB/s", bytes_per_second / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB/s", bytes_per_second / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_eta(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}