use crate::error::{Error, Result};
use libmtp_rs::device::raw::detect_raw_devices;
use libmtp_rs::device::MtpDevice;
use libmtp_rs::object::filetypes::Filetype;
use libmtp_rs::object::Object;
use libmtp_rs::storage::Parent;

const AMAZON_VENDOR_ID: u16 = 0x1949;

#[derive(Debug, Clone)]
pub struct KindleInfo {
    pub manufacturer: String,
    pub model: String,
    pub serial: String,
    pub friendly_name: String,
}

#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub description: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_folder: bool,
    #[allow(dead_code)]
    pub id: u32,
}

pub struct Kindle {
    device: MtpDevice,
}

impl Kindle {
    pub fn detect() -> Result<Self> {
        let raw_devices = detect_raw_devices().map_err(|e| {
            let err_str = format!("{}", e);
            if err_str.contains("NoDeviceAttached") {
                Error::DeviceNotFound
            } else {
                Error::Mtp(err_str)
            }
        })?;

        let kindle_raw = raw_devices
            .into_iter()
            .find(|d| d.device_entry().vendor_id == AMAZON_VENDOR_ID)
            .ok_or(Error::DeviceNotFound)?;

        let device = kindle_raw.open_uncached().ok_or(Error::DeviceNotFound)?;

        Ok(Self { device })
    }

    pub fn info(&self) -> KindleInfo {
        KindleInfo {
            manufacturer: self
                .device
                .manufacturer_name()
                .unwrap_or_else(|_| "Unknown".to_string()),
            model: self
                .device
                .model_name()
                .unwrap_or_else(|_| "Unknown".to_string()),
            serial: self
                .device
                .serial_number()
                .unwrap_or_else(|_| "".to_string()),
            friendly_name: self
                .device
                .get_friendly_name()
                .unwrap_or_else(|_| "Kindle".to_string()),
        }
    }

    pub fn storage_info(&self) -> Result<StorageInfo> {
        let storage_pool = self.device.storage_pool();
        let (_, storage) = storage_pool
            .iter()
            .next()
            .ok_or_else(|| Error::Mtp("No storage found".to_string()))?;

        Ok(StorageInfo {
            description: storage.description().unwrap_or("Internal Storage").to_string(),
            total_bytes: storage.maximum_capacity(),
            free_bytes: storage.free_space_in_bytes(),
        })
    }

    pub fn list_files(&self, path: &str) -> Result<Vec<FileEntry>> {
        let storage_pool = self.device.storage_pool();
        let (_, storage) = storage_pool
            .iter()
            .next()
            .ok_or_else(|| Error::Mtp("No storage found".to_string()))?;

        let parent = if path == "/" || path.is_empty() {
            Parent::Root
        } else {
            let obj_id = self.resolve_path(path)?;
            Parent::Folder(obj_id)
        };

        let files = storage.files_and_folders(parent);
        Ok(files
            .into_iter()
            .map(|f| FileEntry {
                name: f.name().to_string(),
                size: f.size(),
                is_folder: matches!(f.ftype(), Filetype::Folder),
                id: f.id(),
            })
            .collect())
    }

    pub fn resolve_path(&self, path: &str) -> Result<u32> {
        let storage_pool = self.device.storage_pool();
        let (_, storage) = storage_pool
            .iter()
            .next()
            .ok_or_else(|| Error::Mtp("No storage found".to_string()))?;

        let path = path.trim_start_matches('/');
        if path.is_empty() {
            return Err(Error::InvalidPath("Cannot resolve root path to ID".to_string()));
        }

        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current_parent = Parent::Root;

        for (i, part) in parts.iter().enumerate() {
            let files = storage.files_and_folders(current_parent);
            let found = files.into_iter().find(|f| f.name() == *part);

            match found {
                Some(f) => {
                    if i == parts.len() - 1 {
                        return Ok(f.id());
                    }
                    if !matches!(f.ftype(), Filetype::Folder) {
                        return Err(Error::InvalidPath(format!(
                            "'{}' is not a directory",
                            part
                        )));
                    }
                    current_parent = Parent::Folder(f.id());
                }
                None => {
                    return Err(Error::FileNotFound(format!("'{}' not found in path", part)));
                }
            }
        }

        Err(Error::InvalidPath("Path resolution failed".to_string()))
    }

    pub fn download_file(&self, remote_path: &str, local_path: &std::path::Path) -> Result<()> {
        let file_id = self.resolve_path(remote_path)?;

        let storage_pool = self.device.storage_pool();
        let (_, storage) = storage_pool
            .iter()
            .next()
            .ok_or_else(|| Error::Mtp("No storage found".to_string()))?;

        storage
            .get_file_to_path(file_id, local_path)
            .map_err(|e| Error::TransferFailed(format!("{}", e)))?;

        Ok(())
    }

}
