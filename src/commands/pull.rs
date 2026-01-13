use crate::cli::{HumanReadable, Output};
use crate::device::Kindle;
use crate::error::{Error, Result};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct PullOutput {
    pub remote: String,
    pub local: String,
    pub bytes: u64,
}

impl HumanReadable for PullOutput {
    fn to_human(&self) -> String {
        format!("Downloaded {} -> {} ({} bytes)", self.remote, self.local, self.bytes)
    }
}

pub fn run_pull(output: &Output, remote: &str, local: &str, recursive: bool) -> Result<()> {
    if recursive {
        return Err(Error::Mtp("Recursive download not yet implemented".to_string()));
    }

    let kindle = Kindle::detect()?;

    // Determine the local file path
    let local_path = Path::new(local);
    let dest_path = if local_path.is_dir() {
        // Extract filename from remote path
        let filename = remote
            .rsplit('/')
            .next()
            .ok_or_else(|| Error::InvalidPath("Invalid remote path".to_string()))?;
        local_path.join(filename)
    } else {
        local_path.to_path_buf()
    };

    kindle.download_file(remote, &dest_path)?;

    // Get file size for output
    let bytes = std::fs::metadata(&dest_path)
        .map(|m| m.len())
        .unwrap_or(0);

    let pull_output = PullOutput {
        remote: remote.to_string(),
        local: dest_path.display().to_string(),
        bytes,
    };

    output.print(&pull_output);
    Ok(())
}
