use crate::cli::{HumanReadable, Output};
use crate::device::Kindle;
use crate::error::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusOutput {
    pub connected: bool,
    pub model: String,
    pub free_bytes: u64,
    pub total_bytes: u64,
}

impl HumanReadable for StatusOutput {
    fn to_human(&self) -> String {
        if !self.connected {
            return "No Kindle connected".to_string();
        }
        let free_gb = self.free_bytes as f64 / 1_000_000_000.0;
        let total_gb = self.total_bytes as f64 / 1_000_000_000.0;
        format!(
            "{} connected - {:.1}GB free of {:.1}GB",
            self.model, free_gb, total_gb
        )
    }
}

pub fn run_status(output: &Output) -> Result<()> {
    let kindle = Kindle::detect()?;
    let info = kindle.info();
    let storage = kindle.storage_info()?;

    let status = StatusOutput {
        connected: true,
        model: info.friendly_name,
        free_bytes: storage.free_bytes,
        total_bytes: storage.total_bytes,
    };

    output.print(&status);
    Ok(())
}
