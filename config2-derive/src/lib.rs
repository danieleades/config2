use proc_macro::TokenStream;
use syn::DeriveInput;

mod partial;

#[proc_macro_derive(Layered)]
pub fn derive_layered(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let data = partial::Data::from(&ast);

    data.build().into()
}
