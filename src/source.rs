//! Configuration [`Source`]s

#[cfg(feature = "env")]
mod env;
#[cfg(feature = "env")]
pub use env::Env;
pub mod file;

use crate::Layered;

/// A [`Source`] supplies configuration 'layers' to the application.
///
/// A single source need not provide a valid application alone, so long as the
/// chosen layers collectively provide a valid configuration.
pub trait Source<T>
where
    T: Layered,
{
    /// Errors that can occur during config reading
    ///
    /// Implementors may define their own error types.
    type Err: std::error::Error;

    /// Retrieve a layer of configuration
    ///
    /// # Errors
    ///
    /// this method may be fallible. It's up to implementors to determine the
    /// appropriate failure modes
    fn fetch(&self) -> Result<T::Layer, Self::Err>;
}
