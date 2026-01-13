use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No Kindle device found")]
    DeviceNotFound,

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[allow(dead_code)]
    #[error("Permission denied")]
    PermissionDenied,

    #[allow(dead_code)]
    #[error("Storage full")]
    StorageFull,

    #[error("Transfer failed: {0}")]
    TransferFailed(String),

    #[error("MTP error: {0}")]
    Mtp(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path error: {0}")]
    InvalidPath(String),
}

impl Error {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::DeviceNotFound => ExitCode::from(2),
            Self::FileNotFound(_) => ExitCode::from(3),
            Self::PermissionDenied => ExitCode::from(4),
            Self::StorageFull => ExitCode::from(5),
            Self::TransferFailed(_) => ExitCode::from(6),
            Self::Mtp(_) | Self::Io(_) | Self::InvalidPath(_) => ExitCode::from(1),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
