use anyhow::Result;
use config2::{source, Layered};

#[derive(Debug, Layered)]
struct Config {
    field_a: i32,
    field_b: String,
}

fn config_factory() -> anyhow::Result<Config> {
    Ok(Config::builder()
        .with_source(&source::file::Toml::new("unimplimented"))?
        .build()?)
}

fn main() -> Result<()> {
    println!("config: {:#?}", &config_factory()?);

    Ok(())
}
