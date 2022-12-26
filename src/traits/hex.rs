use std::num::ParseIntError;

pub trait FromHex: Sized {
    /// The error type returned when parsing fails.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Parse a hex string into this type.
    fn from_hex<T: AsRef<str>>(hex: T) -> Result<Self, Self::Error>;
}

// pub trait ToHex {
//     /// The error type returned when parsing fails.
//     type Error: std::error::Error + Send + Sync + 'static;

//     /// Parse a hex string into this type.
//     fn to_hex<T: AsMut<[u8]>>(&self, hex: T) -> Result<usize, Self::Error>;
// }

impl FromHex for u16 {
    type Error = ParseIntError;

    fn from_hex<T: AsRef<str>>(hex: T) -> Result<Self, Self::Error> {
        Ok(if let Some(hex) = hex.as_ref().strip_prefix("0x") {
            u16::from_str_radix(hex, 16)?
        } else {
            u16::from_str_radix(hex.as_ref(), 16)?
        })
    }
}
