use std::path::PathBuf;

use clap::{ArgGroup, Parser, Subcommand};

/// A command line tool to flash qmk firmware to SN32F* based keyboards
#[derive(Parser, Clone, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// List the connected keyboards supported by this tool
    #[command(group(
            ArgGroup::new("mode")
                .args(["bootloader", "normal", "all"]),
        ))]
    List {
        /// Print the devices in verbose mode
        #[arg(short, long)]
        verbose: bool,
        /// Show devices in bootloader mode
        #[arg(short, long, group = "mode")]
        bootloader: bool,
        /// Show devices in normal mode
        #[arg(short, long, group = "mode")]
        normal: bool,
        /// Show devices in any mode
        #[arg(short, long, group = "mode")]
        all: bool,
    },
    /// Operation on a specific keyboard
    Firmware {
        #[command(subcommand)]
        command: FirmwareCommand,
    },
    Flash {
        /// The path to the firmware file
        firmware: PathBuf,
        /// The path to the keyboard
        #[arg(short, long)]
        keyboard: Option<String>,
        /// The offset to flash from
        #[arg(short, long)]
        offset: Option<u32>,
    },
    /// Reboot the keyboard
    Reboot {
        /// The identifier for the keyboard
        #[arg(short, long)]
        keyboard: Option<String>,
        #[command(subcommand)]
        bootloader: Bootloader,
        // normal: bool,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum FirmwareCommand {
    Check { path: PathBuf },
}

#[derive(Clone, Debug, Subcommand)]
pub enum Bootloader {
    EVision,
    HFD,
}

impl Bootloader {
    pub fn commands(&self) -> [u8; 8] {
        let mut res = [0; 8];
        match self {
            Bootloader::EVision => {
                res[..4].copy_from_slice(&0x5AA555AA_u32.to_le_bytes());
                res[4..].copy_from_slice(&0xCC3300FF_u32.to_le_bytes());
            }
            Bootloader::HFD => {
                res[..4].copy_from_slice(&0x5A8942AA_u32.to_le_bytes());
                res[4..].copy_from_slice(&0xCC6271FF_u32.to_le_bytes());
            }
        }
        res
    }
}
