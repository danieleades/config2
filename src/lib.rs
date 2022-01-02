use source::Source;

pub mod source;

pub use config2_derive::Layered;

pub trait Partial: Sized + Default + From<Self::T> {
    type T;
    fn merge(&mut self, other: Self);
    fn try_build(self) -> Result<Self::T, Error>;
}

pub trait Layered: Sized {
    type Layer: Partial<T = Self>;

    fn builder() -> ConfigBuilder<Self> {
        ConfigBuilder::new()
    }
}

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

    pub fn build(self) -> Result<T, Error> {
        self.config.try_build()
    }

    pub fn with_source<S>(mut self, source: S) -> Result<Self, S::Err>
    where
        S: Source<T>,
    {
        self.config.merge(source.fetch()?);
        Ok(self)
    }
}

impl<T> Partial for Option<T> {
    type T = T;

    fn merge(&mut self, other: Self) {
        if let Some(x) = other {
            *self = Some(x);
        }
    }

    fn try_build(self) -> Result<Self::T, Error> {
        self.ok_or(Error)
    }
}

impl<T> ConfigBuilder<T>
where
    T: Layered + Default,
    T::Layer: Partial<T = T>,
{
    pub fn with_default(mut self) -> Self {
        self.config.merge(T::default().into());
        self
    }
}

#[derive(Debug, thiserror::Error)]
#[error("failed to build config")]
pub struct Error;
