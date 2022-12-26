pub mod cli;
pub mod constants;
pub mod devices;
pub mod error;
pub mod firmware;
pub mod flash;
pub mod traits;
use cli::Command;
pub use error::Result;

use crate::devices::Normal;

use self::cli::FirmwareCommand;
use self::devices::{Bootloader, Devices};
use self::firmware::{Firmware, UnsafeFirmware};

fn main() -> Result<(), main_error::MainError> {
    let args = <cli::Args as clap::Parser>::parse();
    // dbg!(&args);
    match args.command {
        Command::List { verbose: v } => {
            let n_devices = devices::Devices::<devices::Normal>::get();
            let b_devices = devices::Devices::<devices::Bootloader>::get();
            // if v {
            //     println!("Normal devices: \n{n_devices:#?}");
            //     println!("Bootloader devices: \n{b_devices:#?}");
            // } else {
            //     println!("Normal devices: \n{n_devices}");
            //     println!("Bootloader devices: \n{b_devices}");
            // }
            match (n_devices, b_devices, v) {
                (Ok(n), Ok(b), true) => {
                    println!("Normal devices: \n{n:#?}");
                    println!("Bootloader devices: \n{b:#?}");
                }
                (Ok(n), Ok(b), false) => {
                    println!("Normal devices: \n{n}");
                    println!("Bootloader devices: \n{b}");
                }
                (Ok(n), Err(_), true) => {
                    println!("Normal devices: \n{n:#?}");
                }
                (Ok(n), Err(_), false) => {
                    println!("Normal devices: \n{n}");
                }
                (Err(_), Ok(b), true) => {
                    println!("Bootloader devices: \n{b:#?}");
                }
                (Err(_), Ok(b), false) => {
                    println!("Bootloader devices: \n{b}");
                }
                (Err(e), Err(_), _) => {
                    return Err(e.into());
                }
            }
        }
        Command::Firmware { command } => match command {
            FirmwareCommand::Check { path } => {
                UnsafeFirmware::from(std::fs::File::open(path)?).check()?;
                println!("The Firmware is valid");
            }
        },
        Command::Flash { firmware, keyboard } => {
            let devices = Devices::<Bootloader>::get()?;
            let d = devices.decide::<String>(keyboard)?;
            println!("device: {:#?}", d);

            let _firmware =
                Firmware::try_from(UnsafeFirmware::from(std::fs::File::open(firmware)?))?;

            let keyboard = devices::Keyboard::<Bootloader>::connect(d);
            todo!();
        }
        Command::Reboot { keyboard } => {
            let devices = Devices::<Normal>::get()?;
            let d = devices.decide::<String>(keyboard)?;
            let mut keyboard = devices::Keyboard::<Normal>::connect(d)?;
            keyboard.reboot()?;
        }
    }
    Ok(())
}
