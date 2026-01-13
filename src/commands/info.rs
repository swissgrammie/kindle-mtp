use crate::cli::{HumanReadable, Output};
use crate::device::Kindle;
use crate::error::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct InfoOutput {
    pub device: String,
    pub manufacturer: String,
    pub model: String,
    pub serial: String,
    pub storage_description: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
}

impl HumanReadable for InfoOutput {
    fn to_human(&self) -> String {
        let free_gb = self.free_bytes as f64 / 1_000_000_000.0;
        let total_gb = self.total_bytes as f64 / 1_000_000_000.0;
        format!(
            "Device: {}\n\
             Manufacturer: {}\n\
             Model: {}\n\
             Serial: {}\n\
             Storage: {} ({:.2}GB)\n\
             Free: {:.2}GB",
            self.device,
            self.manufacturer,
            self.model,
            if self.serial.is_empty() {
                "(not available)"
            } else {
                &self.serial
            },
            self.storage_description,
            total_gb,
            free_gb
        )
    }
}

pub fn run_info(output: &Output) -> Result<()> {
    let kindle = Kindle::detect()?;
    let info = kindle.info();
    let storage = kindle.storage_info()?;

    let info_output = InfoOutput {
        device: info.friendly_name,
        manufacturer: info.manufacturer,
        model: info.model,
        serial: info.serial,
        storage_description: storage.description,
        total_bytes: storage.total_bytes,
        free_bytes: storage.free_bytes,
    };

    output.print(&info_output);
    Ok(())
}
