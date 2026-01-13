use crate::cli::{HumanReadable, Output};
use crate::device::{Kindle, FileEntry};
use crate::error::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct LsOutput {
    pub path: String,
    pub entries: Vec<LsEntry>,
}

#[derive(Serialize)]
pub struct LsEntry {
    pub name: String,
    pub size: u64,
    pub is_folder: bool,
}

impl From<FileEntry> for LsEntry {
    fn from(f: FileEntry) -> Self {
        Self {
            name: f.name,
            size: f.size,
            is_folder: f.is_folder,
        }
    }
}

impl HumanReadable for LsOutput {
    fn to_human(&self) -> String {
        if self.entries.is_empty() {
            return "(empty)".to_string();
        }
        self.entries
            .iter()
            .map(|e| {
                if e.is_folder {
                    format!("{}/", e.name)
                } else {
                    e.name.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Serialize)]
pub struct LsOutputLong(pub LsOutput);

impl HumanReadable for LsOutputLong {
    fn to_human(&self) -> String {
        if self.0.entries.is_empty() {
            return "(empty)".to_string();
        }
        self.0
            .entries
            .iter()
            .map(|e| {
                let type_char = if e.is_folder { "d" } else { "-" };
                let size_str = if e.is_folder {
                    "-".to_string()
                } else {
                    format_size(e.size)
                };
                format!("{} {:>10}  {}", type_char, size_str, e.name)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1}G", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.1}M", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1}K", bytes as f64 / 1_000.0)
    } else {
        format!("{}B", bytes)
    }
}

pub fn run_ls(output: &Output, path: &str, long: bool) -> Result<()> {
    let kindle = Kindle::detect()?;
    let files = kindle.list_files(path)?;

    let ls_output = LsOutput {
        path: path.to_string(),
        entries: files.into_iter().map(LsEntry::from).collect(),
    };

    if long {
        if output.is_json() {
            output.print(&ls_output);
        } else {
            output.print(&LsOutputLong(ls_output));
        }
    } else {
        output.print(&ls_output);
    }

    Ok(())
}
