use anyhow::Result;
use config2::{source, Error, Layered, Partial};

#[derive(Debug)]
struct Config {
    field_a: Option<i32>,
    field_b: String,
}

impl Layered for Config {
    type Layer = PartialConfig;
}

#[derive(Default)]
struct PartialConfig {
    field_a: Option<i32>,
    field_b: Option<String>,
}

impl Partial for PartialConfig {
    type T = Config;

    fn merge(&mut self, other: Self) {
        self.field_a.merge(other.field_a);
        self.field_b.merge(other.field_b);
    }

    fn try_build(self) -> Result<Self::T, Error> {
        Ok(Self::T {
            field_a: self.field_a.try_build().ok(),
            field_b: self.field_b.try_build()?,
        })
    }
}

impl From<Config> for PartialConfig {
    fn from(config: Config) -> Self {
        Self {
            field_a: config.field_a,
            field_b: Some(config.field_b),
        }
    }
}

fn main() -> Result<()> {
    let config = Config::builder().with_source(source::File)?.build()?;

    println!("config: {:#?}", &config);

    Ok(())
}
