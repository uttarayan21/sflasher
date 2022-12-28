use hidapi::DeviceInfo;

use crate::error::{Error, ErrorKind};

pub struct FlashingOptions {
    pub device_type: Sonix,
    pub offset: Option<u32>,
}

impl FlashingOptions {
    pub fn offset(&self) -> u32 {
        if let Some(offset) = self.offset {
            offset
        } else {
            self.device_type.offset()
        }
    }
    pub fn with_offset(&mut self, offset: Option<u32>) -> &mut Self {
        self.offset = offset;
        self
    }
}

impl TryFrom<&DeviceInfo> for FlashingOptions {
    type Error = Error;

    fn try_from(device_info: &DeviceInfo) -> Result<Self, Self::Error> {
        let device_type = Sonix::try_from(device_info.product_id())?;
        Ok(Self {
            device_type,
            offset: None,
        })
    }
}

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum Sonix {
    SN32F248 = 0x7040,
    SN32F248B = 0x7900,
    SN32F260 = 0x7010,
}

impl Sonix {
    pub const fn offset(self) -> u32 {
        match self {
            Sonix::SN32F248 => 0x0,
            Sonix::SN32F248B => 0x0,
            Sonix::SN32F260 => 0x200,
        }
    }

    pub const fn pid(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for Sonix {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x7040 => Ok(Sonix::SN32F248),
            0x7900 => Ok(Sonix::SN32F248B),
            0x7010 => Ok(Sonix::SN32F260),
            _ => Err(ErrorKind::InvalidDevice.into()),
        }
    }
}
