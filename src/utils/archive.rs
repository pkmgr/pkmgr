use anyhow::{Context, Result};
use std::path::Path;
use tar::Archive;
use zip::ZipArchive;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use std::fs::File;

pub enum ArchiveType {
    TarGz,
    TarBz2,
    Tar,
    Zip,
    Unknown,
}

impl ArchiveType {
    pub fn from_path(path: &Path) -> Self {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            ArchiveType::TarGz
        } else if name.ends_with(".tar.bz2") || name.ends_with(".tbz2") {
            ArchiveType::TarBz2
        } else if name.ends_with(".tar") {
            ArchiveType::Tar
        } else if name.ends_with(".zip") {
            ArchiveType::Zip
        } else {
            ArchiveType::Unknown
        }
    }
}

pub struct Extractor;

impl Extractor {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract(&self, archive_path: &Path, dest_dir: &Path) -> Result<()> {
        tokio::fs::create_dir_all(dest_dir).await
            .context("Failed to create destination directory")?;

        let archive_type = ArchiveType::from_path(archive_path);

        match archive_type {
            ArchiveType::TarGz => self.extract_tar_gz(archive_path, dest_dir).await,
            ArchiveType::TarBz2 => self.extract_tar_bz2(archive_path, dest_dir).await,
            ArchiveType::Tar => self.extract_tar(archive_path, dest_dir).await,
            ArchiveType::Zip => self.extract_zip(archive_path, dest_dir).await,
            ArchiveType::Unknown => {
                // Try to detect by file content
                self.extract_auto(archive_path, dest_dir).await
            }
        }
    }

    async fn extract_tar_gz(&self, archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path)
            .context("Failed to open archive")?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        archive.unpack(dest_dir)
            .context("Failed to extract tar.gz archive")?;

        Ok(())
    }

    async fn extract_tar_bz2(&self, archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path)
            .context("Failed to open archive")?;
        let decoder = BzDecoder::new(file);
        let mut archive = Archive::new(decoder);

        archive.unpack(dest_dir)
            .context("Failed to extract tar.bz2 archive")?;

        Ok(())
    }

    async fn extract_tar(&self, archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path)
            .context("Failed to open archive")?;
        let mut archive = Archive::new(file);

        archive.unpack(dest_dir)
            .context("Failed to extract tar archive")?;

        Ok(())
    }

    async fn extract_zip(&self, archive_path: &Path, dest_dir: &Path) -> Result<()> {
        let file = File::open(archive_path)
            .context("Failed to open archive")?;
        let mut archive = ZipArchive::new(file)
            .context("Failed to read zip archive")?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .context("Failed to access file in zip")?;

            let outpath = dest_dir.join(file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .context("Failed to create directory")?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .context("Failed to create parent directory")?;
                    }
                }

                let mut outfile = File::create(&outpath)
                    .context("Failed to create output file")?;

                std::io::copy(&mut file, &mut outfile)
                    .context("Failed to extract file")?;
            }

            // Set permissions on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                        .context("Failed to set permissions")?;
                }
            }
        }

        Ok(())
    }

    async fn extract_auto(&self, archive_path: &Path, dest_dir: &Path) -> Result<()> {
        // Try different extraction methods
        if self.extract_tar_gz(archive_path, dest_dir).await.is_ok() {
            return Ok(());
        }

        if self.extract_zip(archive_path, dest_dir).await.is_ok() {
            return Ok(());
        }

        if self.extract_tar(archive_path, dest_dir).await.is_ok() {
            return Ok(());
        }

        anyhow::bail!("Unable to determine archive format")
    }

    pub async fn extract_single_binary(&self, archive_path: &Path, binary_name: &str, dest: &Path) -> Result<()> {
        let temp_dir = tempfile::tempdir()
            .context("Failed to create temp directory")?;

        self.extract(archive_path, temp_dir.path()).await?;

        // Find the binary in the extracted files
        let binary_path = self.find_binary(temp_dir.path(), binary_name)?;

        // Copy the binary to the destination
        tokio::fs::copy(binary_path, dest).await
            .context("Failed to copy binary")?;

        // Make it executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = tokio::fs::metadata(dest).await?.permissions();
            perms.set_mode(0o755);
            tokio::fs::set_permissions(dest, perms).await?;
        }

        Ok(())
    }

    fn find_binary(&self, dir: &Path, name: &str) -> Result<std::path::PathBuf> {
        use walkdir::WalkDir;

        for entry in WalkDir::new(dir) {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str == name || file_name_str.starts_with(name) {
                        // Check if it's an executable file (not a directory)
                        if path.is_file() {
                            return Ok(path.to_path_buf());
                        }
                    }
                }
            }
        }

        anyhow::bail!("Binary '{}' not found in archive", name)
    }
}