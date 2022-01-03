//! File-based [`Source`]s

use std::{ffi::OsStr, path::Path};

use serde::de::DeserializeOwned;

use super::Source;
use crate::Layered;

/// A configuration source which reads [TOML](https://github.com/toml-lang/toml) files
#[derive(Debug)]
pub struct Toml<'a> {
    path: &'a Path,
}

impl<'a> Toml<'a> {
    /// # Example
    ///
    /// ```
    /// use config2::source::file::Toml;
    ///
    /// let file = Toml::new("some/path/to/file.toml");
    /// ```
    pub fn new<S: AsRef<OsStr> + ?Sized>(path: &'a S) -> Self {
        Self {
            path: Path::new(path),
        }
    }
}

impl<'a, T> Source<T> for Toml<'a>
where
    T: Layered,
    T::Layer: DeserializeOwned,
{
    type Err = Error;

    fn fetch(&self) -> Result<T::Layer, Self::Err> {
        let content = std::fs::read_to_string(self.path)?;
        Ok(toml::from_str(&content)?)
    }
}

/// Errors which can occur when reading configuration from files
#[derive(Debug, thiserror::Error)]
#[error("File source error")]
pub enum Error {
    /// File IO error
    Io(#[from] std::io::Error),

    /// TOML deserialisation errors
    Toml(#[from] toml::de::Error),
}
