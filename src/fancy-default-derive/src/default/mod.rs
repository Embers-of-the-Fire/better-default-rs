use proc_macro2::TokenStream;
use syn::{parse2, DeriveInput};

pub(super) mod enum_impl;
pub(super) mod struct_impl;

pub(crate) fn process_default_derive(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;
    match &input.data {
        syn::Data::Struct(struct_data) => {
            struct_impl::impl_struct(struct_data, &input.ident, &input.generics)
        }
        syn::Data::Enum(enum_data) => {
            enum_impl::impl_enum(enum_data, &input.ident, &input.generics)
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn process_const_default_derive(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;
    match &input.data {
        syn::Data::Struct(struct_data) => {
            struct_impl::impl_struct_const(struct_data, &input.ident, &input.generics)
        },
        syn::Data::Enum(enum_data) => {
            enum_impl::impl_enum_const(enum_data, &input.ident, &input.generics)
        }
        _ => unimplemented!()
    }
}
