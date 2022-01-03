//! Provides the derive macro for the `config2` crate

#![deny(clippy::all, missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use proc_macro::TokenStream;
use syn::DeriveInput;

mod struct_data;

#[proc_macro_derive(Layered)]
pub fn derive_layered(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let generics = &ast.generics;

    match &ast.data {
        syn::Data::Struct(data) => {
            let fields = &data.fields;
            let data = struct_data::Data::new(name, generics, fields);
            data.build().into()
        }
        _ => panic!("'Layered' can only be derived for structs"),
    }
}
