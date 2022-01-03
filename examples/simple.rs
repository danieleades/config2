use anyhow::Result;
use config2::{source, Error, Layered, Partial};
use serde::Deserialize;

#[derive(Debug)]
struct Config {
    field_a: i32,
    field_b: String,
}

impl Layered for Config {
    type Layer = PartialConfig;
}

#[derive(Default, Deserialize)]
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

    fn build(self) -> Result<Self::T, Error> {
        Ok(Self::T {
            field_a: self.field_a.build()?,
            field_b: self.field_b.build()?,
        })
    }
}

impl From<Config> for PartialConfig {
    fn from(config: Config) -> Self {
        Self {
            field_a: Some(config.field_a),
            field_b: Some(config.field_b),
        }
    }
}

fn main() -> Result<()> {
    let config = Config::builder()
        .with_source(&source::file::Toml::new("unimplimented"))?
        .build()?;

    println!("config: {:#?}", &config);

    Ok(())
}
