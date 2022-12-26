use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A command line tool to flash qmk firmware to SN32F* based keyboards
#[derive(Parser, Clone, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// List the connected keyboards supported by this tool
    List {
        #[arg(short, long)]
        verbose: bool,
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
    },
    Reboot {
        keyboard: Option<String>,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum FirmwareCommand {
    Check { path: PathBuf },
}
