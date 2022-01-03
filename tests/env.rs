#![cfg(feature = "env")]

use config2::{source, Layered};

#[derive(Debug, Layered, PartialEq)]
struct Config {
    field_a: String,
}

#[test]
fn env() {
    std::env::set_var("FIELD_A", "value a");

    let config = Config::builder()
        .with_source(&source::Env::default())
        .unwrap()
        .build()
        .unwrap();

    let expected = Config {
        field_a: "value a".to_string(),
    };

    assert_eq!(config, expected);
}

#[test]
fn env_prefix() {
    std::env::set_var("PREFIX_FIELD_A", "prefixed value a");

    let config = Config::builder()
        .with_source(&source::Env::default().with_prefix("PREFIX_"))
        .expect("failed to deserialise from environment")
        .build()
        .expect("missing required fields");

    let expected = Config {
        field_a: "prefixed value a".to_string(),
    };

    assert_eq!(config, expected);
}
