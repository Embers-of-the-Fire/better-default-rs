//! This crate is an internal implementation of the `fancy-default` library.
//! 
//! See the [`fancy-default`] library's documentation for instructions on how to use it.

use proc_macro::TokenStream;

mod default;
mod variant_default;

/// Derive the [`core::default::Default`] trait.
/// 
/// This derive macro actually implements
/// [`fancy_default::traits::Default`] to prevent naming pollution.
#[proc_macro_derive(Default, attributes(default))]
pub fn derive_default(input: TokenStream) -> TokenStream {
    match default::process_default_derive(input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derive the [`fancy_default::traits::ConstDefault`] trait.
#[proc_macro_derive(ConstDefault, attributes(default))]
pub fn derive_const_default(input: TokenStream) -> TokenStream {
    match default::process_const_default_derive(input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derive no traits, but implement functions/associated constants for the type.
#[proc_macro_derive(VariantDefault, attributes(variant, default))]
pub fn derive_variant_default(input: TokenStream) -> TokenStream {
    match variant_default::process_variant_default(input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
