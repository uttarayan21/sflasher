use std::io::{Read, SeekFrom};

use crate::constants::MAX_FIRMWARE;
use crate::error::ErrorKind;
use crate::traits::buffer::SizedBuffer;
use crate::Result;

pub struct UnsafeFirmware<T: SizedBuffer> {
    pub inner: T,
}

impl<T: SizedBuffer> UnsafeFirmware<T> {
    fn header(&mut self) -> Result<[u32; 4]> {
        let mut buf = [0; 16];
        self.inner.read_exact(&mut buf)?;
        self.inner.seek(SeekFrom::Current(-(buf.len() as i64)))?;
        Ok([
            u32::from_le_bytes(buf[0..4].try_into()?),
            u32::from_le_bytes(buf[4..8].try_into()?),
            u32::from_le_bytes(buf[8..12].try_into()?),
            u32::from_le_bytes(buf[12..16].try_into()?),
        ])
    }

    pub fn len(&self) -> Result<usize> {
        Ok(self.inner.try_len()?)
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    pub fn check(&mut self) -> Result<()> {
        if self.len()? < 0x100 && self.len()? > MAX_FIRMWARE {
            return Err(ErrorKind::InvalidFirmware.into());
        }

        let [sp, vecs @ ..] = self.header()?;
        if !(0x20000000..=0x20000800).contains(&sp)
            || vecs[0] & 1 != 1
            || vecs[1] & 1 != 1
            || vecs[2] & 1 != 1
        {
            return Err(ErrorKind::InvalidFirmware.into());
        }
        Ok(())
    }
}

impl<T: SizedBuffer> From<T> for UnsafeFirmware<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SizedBuffer> TryFrom<UnsafeFirmware<T>> for Firmware<T> {
    type Error = crate::error::Error;

    fn try_from(value: UnsafeFirmware<T>) -> Result<Self, Self::Error> {
        let mut value = value;
        value.check()?;
        Ok(Self { inner: value.inner })
    }
}

// Cannot be solved currently due to the following issue
// https://github.com/rust-lang/rust/issues/50133#issuecomment-488512355
// impl<T: SizedBuffer> TryFrom<T> for Firmware<T> {
//     type Error = anyhow::Error;

//     fn try_from(value: T) -> Result<Self, Self::Error> {
//         let mut value = UnsafeFirmware::from(value);
//         value.check()?;
//         Ok(Self { inner: value.inner })
//     }
// }

// pub enum Safe {}
// pub enum Unsafe {}

// pub trait FirmwareType {

// }

// pub struct Firm<FirmwareType, T: Sized> {
//     pub inner: T,
// }

pub struct Firmware<T: SizedBuffer> {
    pub inner: T,
}

impl<T: SizedBuffer> Firmware<T> {
    pub fn len(&self) -> Result<usize> {
        Ok(self.inner.try_len()?)
    }

    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}
