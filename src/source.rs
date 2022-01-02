mod file;
pub use file::File;

use crate::Layered;

pub trait Source<T>
where
    T: Layered,
{
    type Err: std::error::Error;
    fn fetch(&self) -> Result<T::Layer, Self::Err>;
}
