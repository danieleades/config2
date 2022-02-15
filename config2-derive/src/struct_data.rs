use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, Parser},
    Field, Type,
};

pub struct Data<'a> {
    name: &'a syn::Ident,
    generics: &'a syn::Generics,
    fields: &'a syn::Fields,
}

impl<'a> Data<'a> {
    pub fn new(name: &'a syn::Ident, generics: &'a syn::Generics, fields: &'a syn::Fields) -> Self {
        Self {
            name,
            generics,
            fields,
        }
    }

    fn partial_name(&self) -> syn::Ident {
        format_ident!("Partial{}", self.name)
    }

    fn optional_fields(&self) -> impl Iterator<Item = TokenStream2> + 'a {
        self.fields.iter().map(transform_field)
    }

    fn partial_struct_block(&self) -> TokenStream2 {
        let name = self.partial_name();

        let generics = self.generics;
        let fields = self.optional_fields();

        quote! {
            #[derive(Debug, Default, config2::__private::serde_derive::Deserialize)]
            struct #name #generics {
                #(#fields,)*
            }
        }
    }

    fn layered_impl_block(&self) -> TokenStream2 {
        let name = self.name;
        let partial_name = self.partial_name();
        let generics = self.generics;

        quote! {
            impl #generics config2::Layered for #name #generics {
                type Layer = #partial_name #generics;
            }
        }
    }

    fn partial_impl_block(&self) -> TokenStream2 {
        let name = self.name;
        let partial_name = self.partial_name();
        let generics = self.generics;
        let merge_fields = self.merge_fields();
        let build_fields = self.build_fields();

        quote! {
            impl #generics config2::Partial for #partial_name #generics {
                type T = #name #generics;
                fn merge(&mut self, other: Self) { #(#merge_fields)* }
                fn build(self) -> Result<<Self as config2::Partial>::T, config2::Error> {
                    Ok(
                        Self::T {
                            #(#build_fields)*
                        }
                    )
                }
            }
        }
    }

    fn merge_fields(&self) -> impl Iterator<Item = TokenStream2> + 'a {
        self.fields.iter().map(merge_call)
    }

    fn build_fields(&self) -> impl Iterator<Item = TokenStream2> + 'a {
        self.fields.iter().map(build_call)
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_fields(&self) -> impl Iterator<Item = TokenStream2> + 'a {
        self.fields.iter().map(|field| {
            let name = field.ident.as_ref().unwrap();
            quote! {
                #name: value.#name.into(),
            }
        })
    }

    #[allow(clippy::wrong_self_convention)]
    fn from_impl_block(&self) -> TokenStream2 {
        let name = self.name;
        let partial_name = self.partial_name();
        let generics = self.generics;
        let fields = self.from_fields();

        quote! {
            impl #generics From<#name #generics> for #partial_name #generics {
                fn from(value: #name #generics) -> Self {
                    Self {
                        #(#fields)*
                    }
                }
            }
        }
    }

    pub fn build(&self) -> TokenStream2 {
        let struct_block = self.partial_struct_block();
        let layered_impl_block = self.layered_impl_block();
        let partial_impl_block = self.partial_impl_block();
        let from_impl = self.from_impl_block();

        quote! {
            #struct_block
            #layered_impl_block
            #partial_impl_block
            #from_impl
        }
    }
}

fn is_option(ty: &syn::Type) -> bool {
    match &ty {
        Type::Path(ty) => {
            let last_segment = ty.path.segments.clone().into_iter().next_back();
            last_segment.unwrap().ident == "Option"
        }
        _ => false,
    }
}

fn transform_field(field: &Field) -> TokenStream2 {
    if is_option(&field.ty) {
        quote! {#field}
    } else {
        let old_ty = &field.ty;
        let new_ty = syn::Type::parse
            .parse2(quote! {
                Option<#old_ty>
            })
            .unwrap();

        let mut new_field = field.clone();
        new_field.ty = new_ty;

        quote! {#new_field}
    }
}

fn merge_call(field: &Field) -> TokenStream2 {
    let name = field.ident.as_ref().unwrap();

    quote! {self.#name.merge(other.#name);}
}

fn build_call(field: &Field) -> TokenStream2 {
    let name = field.ident.as_ref().unwrap();

    if is_option(&field.ty) {
        quote! {#name: self.#name.build().ok(),}
    } else {
        quote! {#name: self.#name.build()?,}
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse::Parser, Field};
    use test_case::test_case;

    #[test_case("field_a: bool" => false ; "not an option")]
    #[test_case("field_b: Option<bool>" => true ; "is an option")]
    fn is_option(input: &str) -> bool {
        let parser = Field::parse_named;
        let field = parser.parse_str(input).expect("couldn't parse field");
        super::super::is_option(&field.ty)
    }

    #[test_case("field_b: Option<bool>", "field_b: Option<bool>" ; "is an option already")]
    #[test_case("field_b: bool", "field_b: Option<bool>" ; "is not an option")]
    #[test_case("field_b: std::vec::Vec<bool>", "field_b: Option<std::vec::Vec<bool>>" ; "vec")]
    fn transform_field(input: &str, expected: &str) {
        let parser = Field::parse_named;
        let expected = parser.parse_str(expected).expect("couldn't parse field");

        let input_field = parser.parse_str(input).expect("couldn't parse field");
        let result = parser
            .parse2(super::super::transform_field(&input_field))
            .unwrap();

        assert_eq!(result, expected);
    }
}
