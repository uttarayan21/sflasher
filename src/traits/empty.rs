use crate::error::Error;

pub trait EmptyOrElse: Sized {
    type Error;
    fn empty_or_else<F: FnOnce() -> Self::Error>(self, f: F) -> Result<Self, Self::Error>;
}

impl<T> EmptyOrElse for Vec<T> {
    type Error = Error;
    fn empty_or_else<F: FnOnce() -> Self::Error>(self, f: F) -> Result<Self, Self::Error> {
        if self.is_empty() {
            Err(f())
        } else {
            Ok(self)
        }
    }
}
