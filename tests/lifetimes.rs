#![allow(dead_code)]

use config2_derive::Layered;

#[derive(Debug, Layered)]
struct Test<'a> {
    field_a: i32,
    field_b: &'a str,
    field_c: Option<bool>,
}
