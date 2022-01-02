#![allow(dead_code)]
use config2::{source, Layered};

#[derive(Debug, Default, Layered)]
struct Test {
    field_a: i32,
    field_b: String,
    field_c: Option<bool>,
}

#[test]
fn main() -> anyhow::Result<()> {
    let _config = Test::builder()
        .with_default()
        .with_source(source::File)?
        .build()
        .unwrap();

    Ok(())
}
