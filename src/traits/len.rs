use std::fs::File;
use std::io::{BufReader, Cursor};
// use std::io::BufReader;

#[allow(clippy::len_without_is_empty)]
pub trait Len<Output> {
    fn len(&self) -> Output;
}

pub trait TryLen<Output> {
    type Error: std::error::Error + Send + Sync + 'static + Into<crate::error::ErrorKind>;
    fn try_len(&self) -> Result<Output, Self::Error>;
}

impl Len<usize> for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len<usize> for [u8] {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len<usize> for str {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len<usize> for String {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Len<U>, U> Len<U> for BufReader<T> {
    fn len(&self) -> U {
        self.get_ref().len()
    }
}

impl<T: Len<U>, U> Len<U> for Cursor<T> {
    fn len(&self) -> U {
        self.get_ref().len()
    }
}

impl<T: Len<U>, U> Len<U> for std::io::Take<T> {
    fn len(&self) -> U {
        self.get_ref().len()
    }
}

impl Len<Result<u64, std::io::Error>> for File {
    fn len(&self) -> Result<u64, std::io::Error> {
        self.metadata().map(|m| m.len())
    }
}

#[cfg(target_pointer_width = "64")]
impl TryLen<usize> for File {
    type Error = std::io::Error;
    fn try_len(&self) -> Result<usize, std::io::Error> {
        self.metadata().map(|m| {
            m.len()
                .try_into()
                .expect("Shouldn't fail since we're on a 64-bit system and usize should be u64")
        })
    }
}

impl<T: Len<Output>, Output> TryLen<Output> for T {
    type Error = std::convert::Infallible;
    fn try_len(&self) -> Result<Output, Self::Error> {
        Ok(self.len())
    }
}
