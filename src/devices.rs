use crate::constants::{CMD_INIT, CMD_PREPARE, CMD_REBOOT, EXPECTED_STATUS};
use crate::error::ErrorKind;
use crate::firmware::Firmware;
use crate::flash::FlashingOptions;
use crate::traits::buffer::SizedBuffer;
use crate::traits::empty::EmptyOrElse;
use crate::traits::hex::FromHex;
use crate::Result;
use hidapi::{DeviceInfo, HidApi, HidDevice};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufReader, Read};

macro_rules! devices {
    ($map: expr, $(($vid:literal, $pid:literal): $name:expr),* $(,)?) => {
        $(
                $map.insert(($vid, $pid), $name);
        )*
    };
}

static BOOTLOADER_DEVICES: Lazy<HashMap<(u16, u16), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    devices!(m,
        (0x0c45, 0x7010): "SN32F268F (bootloader)",  // 0x200
        (0x0c45, 0x7040): "SN32F248B (bootloader)",  // 0x0
        (0x0c45, 0x7900): "SN32F248 (bootloader)",   // 0x0
    );
    m
});

static NORMAL_DEVICES: Lazy<HashMap<(u16, u16), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    devices!(m,
        (0x05ac, 0x024f): "Apple Keyboard / Keychron / Flashquark Horizon Z",
        (0x05ac, 0x0256): "Apple Keyboard / Ajazz K870T / RAKK Lam-Ang Pro / Miller GM807",
        (0x0c45, 0x652f): "Glorious GMMK / Tecware Phantom",
        (0x0c45, 0x5004): "Redragon",
        (0x0c45, 0x5104): "Redragon",
        (0x0c45, 0x766b): "Kemove",
        (0x0c45, 0x7698): "Womier",
        (0x0C45, 0x7903): "Ajazz",
        (0x0C45, 0x8006): "Sharkoon SGK50 S4",
        (0x0C45, 0x8508): "SPCGear",
        (0x0C45, 0x8513): "Sharkoon",
        (0x320f, 0x5013): "Akko",
        (0x320f, 0x5041): "Designed By GG",
        (0x3299, 0x4E58): "SPCGear",
        (0x3434, 0xfe00): "Keychron K1 ANSI",
        (0x3434, 0xfe01): "Keychron K1 ISO",
        (0x3434, 0xfe02): "Keychron K2 ANSI",
        (0x3434, 0xfe03): "Keychron K2 ISO",
        (0x3434, 0xfe04): "Keychron K3 ANSI",
        (0x3434, 0xfe05): "Keychron K3 ISO",
        (0x3434, 0xfe06): "Keychron K4 ANSI",
        (0x3434, 0xfe07): "Keychron K4 ISO",
        (0x3434, 0xfe08): "Keychron K5 ANSI",
        (0x3434, 0xfe09): "Keychron K5 ISO",
        (0x3434, 0xfe0a): "Keychron K6 ANSI",
        (0x3434, 0xfe0b): "Keychron K6 ISO",
        (0x3434, 0xfe0c): "Keychron K7 ANSI",
        (0x3434, 0xfe0d): "Keychron K7 ISO",
        (0x3434, 0xfe0e): "Keychron K8 ANSI",
        (0x3434, 0xfe0f): "Keychron K8 ISO",
        (0x3434, 0xfe10): "Keychron K9 ANSI",
        (0x3434, 0xfe11): "Keychron K9 ISO",
        (0x3434, 0xfe12): "Keychron K10 ANSI",
        (0x3434, 0xfe13): "Keychron K10 ISO",
        (0x3434, 0xfe14): "Keychron K11 ANSI",
        (0x3434, 0xfe15): "Keychron K11 ISO",
        (0x3434, 0xfe16): "Keychron K12 ANSI",
        (0x3434, 0xfe17): "Keychron K12 ISO",
        (0x3434, 0xfe18): "Keychron K13 ANSI",
        (0x3434, 0xfe19): "Keychron K13 ISO",
        (0x3434, 0xfe1a): "Keychron K14 ANSI",
        (0x3434, 0xfe1b): "Keychron K14 ISO",
        (0x3434, 0xfe1c): "Keychron K15 ANSI",
        (0x3434, 0xfe1d): "Keychron K15 ISO",
        (0x3434, 0xfe1e): "Keychron K16 ANSI",
        (0x3434, 0xfe1f): "Keychron K16 ISO",
        (0x3434, 0xfe20): "Keychron C1 ANSI",
        (0x3434, 0xfe21): "Keychron C1 ISO",
        (0x3434, 0xfe22): "Keychron C2 ANSI",
        (0x3434, 0xfe23): "Keychron C2 ISO",
        (0x3434, 0xfe24): "Keychron C3 ANSI",
        (0x3434, 0xfe25): "Keychron C3 ISO",
        (0x3434, 0xfe26): "Keychron C4 ANSI",
        (0x3434, 0xfe27): "Keychron C4 ISO",
        (0x3434, 0xfe28): "Keychron C5 ANSI",
        (0x3434, 0xfe29): "Keychron C5 ISO",
        (0x3434, 0xfe2a): "Keychron C6 ANSI",
        (0x3434, 0xfe2b): "Keychron C6 ISO",
        (0x3434, 0xfe2c): "Keychron C7 ANSI",
        (0x3434, 0xfe2d): "Keychron C7 ISO",
        (0x3434, 0xfe2e): "Keychron C8 ANSI",
        (0x3434, 0xfe2f): "Keychron C8 ISO",
        (0x3434, 0xfe30): "Keychron C9 ANSI",
        (0x3434, 0xfe31): "Keychron C9 ISO",
        (0x3434, 0xfe32): "Keychron C10 ANSI",
        (0x3434, 0xfe33): "Keychron C10 ISO",
        (0x3434, 0xfe34): "Keychron C11 ANSI",
        (0x3434, 0xfe35): "Keychron C11 ISO",
        (0x3434, 0xfe36): "Keychron C12 ANSI",
        (0x3434, 0xfe37): "Keychron C12 ISO",
        (0x3434, 0xfe38): "Keychron C13 ANSI",
        (0x3434, 0xfe39): "Keychron C13 ISO",
        (0x3434, 0xfe3a): "Keychron C14 ANSI",
        (0x3434, 0xfe3b): "Keychron C14 ISO",
        (0x3434, 0xfe3c): "Keychron C15 ANSI",
        (0x3434, 0xfe3d): "Keychron C15 ISO",
        (0x3434, 0xfe3e): "Keychron C16 ANSI",
        (0x3434, 0xfe3f): "Keychron C16 ISO",
    );
    m
});

#[derive(Debug, Clone)]
pub enum Normal {}
#[derive(Debug, Clone)]
pub enum Bootloader {}

impl Bootloader {}
pub trait Mode {
    fn devices() -> &'static HashMap<(u16, u16), &'static str>;
}
impl Mode for Bootloader {
    fn devices() -> &'static HashMap<(u16, u16), &'static str> {
        &BOOTLOADER_DEVICES
    }
}
impl Mode for Normal {
    fn devices() -> &'static HashMap<(u16, u16), &'static str> {
        &NORMAL_DEVICES
    }
}

#[derive(Clone)]
pub struct Devices<T: Mode> {
    pub devices: Vec<hidapi::DeviceInfo>,
    __marker: std::marker::PhantomData<T>,
}

impl<T: Mode> Debug for Devices<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct CustomDevice<'cd>(&'cd hidapi::DeviceInfo);
        impl<'cd> Debug for CustomDevice<'cd> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut l = f.debug_struct("DeviceInfo");
                l.field("path", &self.0.path());
                l.field("vendor_id", &self.0.vendor_id());
                l.field("product_id", &self.0.product_id());
                l.field("serial_number", &self.0.serial_number());
                l.field("release_number", &self.0.release_number());
                l.field("manufacturer_string", &self.0.manufacturer_string());
                l.field("product_string", &self.0.product_string());
                l.field("usage_page", &self.0.usage_page());
                l.field("usage", &self.0.usage());
                l.field("interface_number", &self.0.interface_number());
                l.finish()
            }
        }
        let mut l = f.debug_list();
        l.entries(self.devices.iter().map(CustomDevice));
        l.finish()
    }
}

impl<T: Mode> Display for Devices<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for device in &self.devices {
            let name = match T::devices().get(&(device.vendor_id(), device.product_id())) {
                Some(name) => name,
                None => "Unknown",
            };
            writeln!(
                f,
                "{}: {:x}:{:x}",
                name,
                device.vendor_id(),
                device.product_id()
            )?;
        }
        Ok(())
    }
}

impl<T: Mode> Devices<T> {
    pub fn get() -> Result<Self> {
        let api = HidApi::new()?;
        Self::get_with_api(&api)
    }

    pub fn get_with_api(api: &HidApi) -> Result<Self> {
        Ok(Self {
            devices: api
                .device_list()
                .filter(|device| {
                    let key = (device.vendor_id(), device.product_id());
                    T::devices().contains_key(&key)
                })
                .unique_by(|d| (d.vendor_id(), d.product_id()))
                .cloned()
                .collect::<Vec<DeviceInfo>>()
                .empty_or_else(|| ErrorKind::NoDevicesFound.into())?,
            __marker: std::marker::PhantomData,
        })
    }
    pub fn find(&self, vendor_id: u16, product_id: u16) -> Result<&DeviceInfo> {
        let device = self
            .devices
            .iter()
            .find(|d| d.vendor_id() == vendor_id && d.product_id() == product_id)
            .ok_or(ErrorKind::DeviceNotFound)?;
        Ok(device)
    }

    pub fn decide<I: AsRef<str>>(&self, identifier: impl Into<Option<I>>) -> Result<&DeviceInfo> {
        if self.devices.is_empty() {
            Err(ErrorKind::NoDevicesFound)?;
        }
        let identifier = identifier.into();
        let device = match identifier {
            Some(identifier) => {
                let identifier = identifier.as_ref();
                let (v, p) = identifier
                    .split(':')
                    .take(2)
                    .tuples()
                    .next()
                    .ok_or_else(|| ErrorKind::InvalidIdentifier(identifier.into()))?;
                let vendor_id = u16::from_hex(v)?;
                let product_id = u16::from_hex(p)?;

                self.find(vendor_id, product_id)?
            }
            None => {
                if self.devices.is_empty() {
                    return Err(ErrorKind::NoDevicesFound.into());
                } else if self.devices.len() == 1 {
                    &self.devices[0]
                } else {
                    return Err(ErrorKind::UnspecifiedDevice.into());
                }
            }
        };
        Ok(device)
    }
}

#[derive(Debug)]
pub struct Keyboard<Mode> {
    name: &'static str,
    device: HidDevice,
    init: bool,
    __marker: std::marker::PhantomData<Mode>,
}

impl<Mode: self::Mode> Keyboard<Mode> {
    pub fn connect(info: &DeviceInfo) -> Result<Self> {
        // NOTE: Maybe somehow reuse the HidApi created on the Devices::get() call
        let hidapi = HidApi::new()?;
        let device = info.open_device(&hidapi)?;
        let name = Mode::devices()
            .get(&(info.vendor_id(), info.product_id()))
            .cloned()
            .unwrap_or("Unknown");
        Ok(Self {
            name,
            device,
            init: false,
            __marker: std::marker::PhantomData,
        })
    }

    fn set_feature(&mut self, report: impl AsRef<[u8]>) -> Result<()> {
        let report = report.as_ref();
        if report.len() > 64 {
            return Err(ErrorKind::InvalidReportLength(report.len()).into());
        }
        let mut buf = [0u8; 65];
        // add 00 at start for hidapi report id (No clue what this does)
        // TODO: Figure out what the report id does
        // max(report.len()) is 64, so this is safe
        // since the buffer is 65 and the max index is 64
        buf[1..=report.len()].copy_from_slice(report);
        self.device.send_feature_report(&buf)?;
        Ok(())
    }

    // # strip 00 at start for hidapi report id
    // return dev.get_feature_report(0, RESPONSE_LEN + 1)[1:]
    fn get_feature(&mut self) -> Result<[u8; 64]> {
        // The report id is in buf[0] and the data is in buf[1..]
        let mut buf = [0u8; 65];
        self.device.get_feature_report(&mut buf)?;
        let [_report, data @ ..] = buf;
        Ok(data)
    }

    pub fn init(&mut self) -> Result<()> {
        self.set_feature(CMD_INIT.to_le_bytes())?;
        let resp = self.get_feature()?;
        let (cmd, _) = resp.split_at(4);

        if u32::from_le_bytes(dbg!(cmd).try_into()?) != CMD_INIT {
            return Err(ErrorKind::FailedToInitialize.into());
        }
        self.init = true;
        Ok(())
    }

    pub fn reboot(&mut self) -> Result<()> {
        // dbg!(self.device.write(&CMD_REBOOT.to_le_bytes()))?;
        self.init()?;
        dbg!(self.write(CMD_REBOOT.to_le_bytes(), None::<&[u8]>))?;
        Ok(())
    }

    pub fn write<C: AsRef<[u8]>, S: AsRef<[u8]>>(
        &mut self,
        command: C,
        status: Option<S>,
    ) -> Result<()> {
        let command = command.as_ref();
        let status = status.as_ref().map(|s| s.as_ref());
        self.set_feature(command)?;
        let resp = self.get_feature()?;
        if let Some(status) = status {
            let (cmd, rest) = resp.split_at(command.len());
            if cmd != command {
                return Err(ErrorKind::FailedToWrite.into());
            }
            let (s, _) = rest.split_at(status.len());
            if s != status {
                return Err(ErrorKind::FailedToWrite.into());
            }
        } else {
            let (cmd, _) = resp.split_at(command.len());
            if cmd != command {
                return Err(ErrorKind::FailedToWrite.into());
            }
        }
        Ok(())
    }
}

impl Keyboard<Bootloader> {
    fn flash<T: SizedBuffer>(
        &mut self,
        firmawre: Firmware<T>,
        options: FlashingOptions,
    ) -> Result<()> {
        self.init()?;
        self.write(
            [
                CMD_PREPARE.to_le_bytes(),
                options.offset.to_le_bytes(),
                (firmawre.len()? as u32).to_le_bytes(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>(),
            Some(EXPECTED_STATUS.to_le_bytes()),
        )?;

        let firmware = BufReader::new(firmawre.into_inner());
        // let mut buf = [0u8; 64];
        for bytes in &firmware.bytes().chunks(64) {
            let bytes = bytes.collect::<Result<Vec<u8>, std::io::Error>>()?;
            // let mut buf = [0u8; 64];
            // buf[..bytes.len()].copy_from_slice(&bytes);
            self.set_feature(&bytes)?;
        }
        self.reboot()?;

        Ok(())
    }
}
