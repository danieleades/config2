//! A framework for layered config for applications.

#![deny(clippy::all, missing_debug_implementations, missing_docs)]
#![warn(clippy::pedantic)]

use serde::de::DeserializeOwned;
use source::Source;

pub mod source;

pub use config2_derive::Layered;

/// A [`Partial`] struct is a version of a [`Layered`] struct for which all its
/// fields are optional
pub trait Partial: Sized + Default + From<Self::T> + DeserializeOwned {
    /// The [`Layered`] struct which corresponds to this [`Partial`] struct
    type T;

    /// Combine two partial structs.
    ///
    /// The 'other' struct will overlay 'self' in the case that both [`Partial`]
    /// structs define a parameter.
    fn merge(&mut self, other: Self);

    /// Attempt to convert this [`Partial`] struct into the corresponding
    /// [`Layered`] struct
    ///
    /// # Errors
    ///
    /// this method can fail if the [`Partial`] struct is missing any fields
    /// which are required in the [`Layered`] struct
    fn build(self) -> Result<Self::T, Error>;
}

/// A struct which implements [`Layered`] can be built up out of multiple
/// [`Partial`] layers.
pub trait Layered: Sized {
    /// the 'partial' version of this struct
    type Layer: Partial<T = Self>;

    /// Construct a [`ConfigBuilder`]
    fn builder() -> ConfigBuilder<Self> {
        ConfigBuilder::new()
    }
}

/// The [`ConfigBuilder`] is used for combining multiple layers of configuration
/// into a single struct.
#[derive(Debug, Clone)]
#[must_use]
pub struct ConfigBuilder<T>
where
    T: Layered,
{
    config: T::Layer,
}

impl<T> ConfigBuilder<T>
where
    T: Layered,
    T::Layer: Partial<T = T>,
{
    fn new() -> Self {
        let config = T::Layer::default();
        Self { config }
    }

    /// Attempt to construct the wrapped struct.
    ///
    /// # Errors
    ///
    /// This can fail if the layered config is missing fields which are required
    /// in the final struct.
    pub fn build(self) -> Result<T, Error> {
        self.config.build()
    }

    /// Add a configuration source
    ///
    /// # Errors
    ///
    /// The error type is determined by the [`Source`] implementation
    pub fn with_source<S>(mut self, source: &S) -> Result<Self, S::Err>
    where
        S: Source<T>,
    {
        self.config.merge(source.fetch()?);
        Ok(self)
    }
}

impl<T> Partial for Option<T>
where
    T: DeserializeOwned,
{
    type T = T;

    fn merge(&mut self, other: Self) {
        if let Some(x) = other {
            *self = Some(x);
        }
    }

    fn build(self) -> Result<Self::T, Error> {
        self.ok_or(Error)
    }
}

impl<T> ConfigBuilder<T>
where
    T: Layered + Default,
    T::Layer: Partial<T = T>,
{
    /// use the struct default as a layer
    pub fn with_default(mut self) -> Self {
        self.config.merge(T::default().into());
        self
    }
}

/// Errors that occur when constructing a [`Layered`] struct from [`Partial`]
/// layers.
#[derive(Debug, thiserror::Error)]
#[error("failed to build config")]
pub struct Error;

#[doc(hidden)]
pub mod __private {
    pub mod serde_derive {
        pub use ::serde::Deserialize;
    }
}
