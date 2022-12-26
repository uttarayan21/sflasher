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
    Reboot {
        keyboard: Option<String>,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum FirmwareCommand {
    Check { path: PathBuf },
}
