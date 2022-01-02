#![allow(unused)]

use anyhow::Result;
use config2::{source, Error, Layered, Partial};

#[derive(Debug)]
struct Config {
    field_a: i32,
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
        todo!()
    }

    fn try_build(self) -> Result<Self::T, Error> {
        todo!()
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

fn config_factory() -> anyhow::Result<Config> {
    Ok(Config::builder().with_source(source::File)?.build()?)
}

fn main() -> Result<()> {
    println!("config: {:#?}", &config_factory()?);

    Ok(())
}
