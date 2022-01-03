use super::Source;
use crate::Layered;

/// A source which deserialises config from the environment
#[must_use]
#[derive(Debug, Default)]
pub struct Env<'a> {
    prefix: Option<&'a str>,
}

impl<'a> Env<'a> {
    /// add a namespace to deserialised variables
    pub fn with_prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }
}

impl<'a, T> Source<T> for Env<'a>
where
    T: Layered,
{
    type Err = envy::Error;

    fn fetch(&self) -> Result<T::Layer, Self::Err> {
        if let Some(prefix) = self.prefix {
            envy::prefixed(prefix).from_env()
        } else {
            envy::from_env()
        }
    }
}
