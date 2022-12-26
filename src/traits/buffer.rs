use std::io::{Read, Seek};

use super::len::TryLen;

pub trait SizedBuffer<Output = usize>: Read + Seek + TryLen<Output> {}

impl<T> SizedBuffer<usize> for T where T: Read + Seek + TryLen<usize> {}
