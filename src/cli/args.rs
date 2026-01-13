use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kindle-mtp")]
#[command(about = "Manage Kindle files via MTP over USB")]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show connection status and quick device info
    Status,

    /// Show detailed device information
    Info,

    /// List directory contents
    Ls {
        /// Path to list (default: root)
        #[arg(default_value = "/")]
        path: String,

        /// Long format with sizes
        #[arg(short, long)]
        long: bool,
    },

    /// Download file(s) from device
    Pull {
        /// Remote path on Kindle
        remote: String,

        /// Local destination path
        #[arg(default_value = ".")]
        local: String,

        /// Recursive download
        #[arg(short, long)]
        recursive: bool,
    },

}
