pub mod cli;
pub mod constants;
pub mod devices;
pub mod error;
pub mod firmware;
pub mod flash;
pub mod traits;
use cli::Command;
pub use error::Result;

use crate::flash::FlashingOptions;

use self::cli::FirmwareCommand;
use self::devices::{Bootloader, Devices};
use self::error::{Error, ErrorKind};
use self::firmware::{Firmware, UnsafeFirmware};

fn main() -> Result<(), main_error::MainError> {
    let args = <cli::Args as clap::Parser>::parse();
    // dbg!(&args);
    match args.command {
        Command::List {
            verbose,
            bootloader,
            normal,
            all,
        } => {
            if all {
                if let Ok(n_devices) = devices::Devices::<devices::Normal>::get() {
                    if !n_devices.is_empty() {
                        println!("Normal devices:");
                        n_devices.print(verbose);
                    }
                };
                if let Ok(b_devices) = devices::Devices::<devices::Bootloader>::get() {
                    if !b_devices.is_empty() {
                        println!("Bootloader devices:");
                        b_devices.print(verbose);
                    }
                }
                return Err(ErrorKind::NoDevicesFound.into());
            } else if normal {
                if let Ok(n_devices) = devices::Devices::<devices::Normal>::get() {
                    n_devices.print(verbose);
                }
            } else if bootloader {
                if let Ok(b_devices) = devices::Devices::<devices::Bootloader>::get() {
                    b_devices.print(verbose);
                }
            } else {
                if let Ok(n_devices) = devices::Devices::<devices::Normal>::get() {
                    if !n_devices.is_empty() {
                        println!("Normal devices:");
                        n_devices.print(verbose);
                    }
                };
                if let Ok(b_devices) = devices::Devices::<devices::Bootloader>::get() {
                    if !b_devices.is_empty() {
                        println!("Bootloader devices:");
                        b_devices.print(verbose);
                    }
                }
                return Err(ErrorKind::NoDevicesFound.into());
            }
        }
        Command::Firmware { command } => match command {
            FirmwareCommand::Check { path } => {
                UnsafeFirmware::from(std::fs::File::open(path)?).check()?;
                println!("The Firmware is valid");
            }
        },
        Command::Flash {
            firmware,
            keyboard,
            offset,
        } => {
            let devices = Devices::<Bootloader>::get()?;
            let d = devices.decide::<String>(keyboard)?;
            println!("device: {:#?}", d);

            let firmware =
                Firmware::try_from(UnsafeFirmware::from(std::fs::File::open(firmware)?))?;

            let mut keyboard = devices::Keyboard::<Bootloader>::connect(d)?;

            let mut options = FlashingOptions::try_from(d)?;
            options.with_offset(offset);
            keyboard.flash(firmware, options)?;
        }
        Command::Reboot { keyboard } => {
            let devices = Devices::<Bootloader>::get()?;
            let d = devices.decide::<String>(keyboard)?;
            let mut keyboard = devices::Keyboard::<Bootloader>::connect(d)?;
            keyboard.reboot()?;
        }
    }
    Ok(())
}
