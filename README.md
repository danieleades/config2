
prototype inspired by the excellent [config-rs](https://github.com/mehcode/config-rs) crate.

This crate provides a mechanism for layered config, whereby multiple sources of configuration can be combined in order of precedence to provide application configuration.

Compared to [config-rs](https://github.com/mehcode/config-rs), [config2](https://github.com/danieleades/config2) is strongly-typed. It relies on some macro magic rather than use a loosely-typed approach.

Each source of configuration need not provide a valid configuration alone, so long as between them they provide a valid configuration.

```rust no_run
use config2::{source::file, Layered};

#[derive(Debug, Default, Layered)]
struct Config {
    field_a: i32,
    field_b: String,
    field_c: Option<bool>,
}

fn main() -> anyhow::Result<()> {
    let config = Config::builder()
        .with_default()
        .with_source(&file::Toml::new("some/path/to/config.toml"))?
        .build()?;

    println!("{:#?}", config);

    Ok(())
}
```