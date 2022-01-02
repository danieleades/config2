use super::Source;
use crate::Layered;

pub struct File;

impl<T> Source<T> for File
where
    T: Layered,
{
    type Err = Error;

    fn fetch(&self) -> Result<T::Layer, Self::Err> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("File source error")]
pub struct Error;
