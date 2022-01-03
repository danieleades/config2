#![allow(dead_code)]
use config2::{source, Layered};

#[derive(Debug, Layered)]
struct Test {
    field_a: i32,
    field_b: String,
    field_c: Option<bool>,
}

#[test]
fn main() -> anyhow::Result<()> {
    let toml_file = std::env::current_dir()
        .unwrap()
        .join("tests/config/test.toml");

    let _config = Test::builder()
        .with_source(&source::file::Toml::new(&toml_file))?
        .build()
        .unwrap();

    Ok(())
}
