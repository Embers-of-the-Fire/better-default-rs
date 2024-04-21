use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse2, spanned::Spanned, Attribute, DataEnum, DeriveInput, Expr, ExprLit, Generics, Lit,
    LitBool, Meta, Token,
};

use crate::default::enum_impl;

pub(crate) fn process_variant_default(input: TokenStream) -> syn::Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;
    match &input.data {
        syn::Data::Enum(enum_data) => {
            impl_enum_variant(&input.attrs, enum_data, &input.ident, &input.generics)
        }
        _ => unimplemented!(),
    }
}

#[derive(Clone, Copy)]
pub struct VariantConfig {
    constant: bool,
    function: bool,
}

impl VariantConfig {
    pub(crate) fn parse_attr(
        default: VariantConfig,
        attrs: &[Attribute],
    ) -> syn::Result<VariantConfig> {
        let mut var_cfg = default;

        for attr in attrs.iter().filter(|a| a.path().is_ident("variant")) {
            var_cfg.function = true;
            
            match &attr.meta {
                Meta::List(ml) => ml.parse_nested_meta(|meta| {
                    if meta.path.is_ident("const") || meta.path.is_ident("constant") {
                        if meta.input.peek(Token![=]) {
                            let val: LitBool = meta.value()?.parse()?;
                            var_cfg.constant = val.value();
                        } else {
                            var_cfg.constant = true;
                        }
                        return Ok(());
                    }

                    if meta.path.get_ident().is_some_and(|s| {
                        matches!(s.to_string().as_str(), "fn" | "func" | "function")
                    }) {
                        if meta.input.peek(Token![=]) {
                            let val: LitBool = meta.value()?.parse()?;
                            var_cfg.function = val.value();
                        } else {
                            var_cfg.function = true;
                        }
                        return Ok(());
                    }

                    Err(meta.error("unrecognized variant-default parameter"))
                })?,
                Meta::NameValue(nv) => {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Bool(b), ..
                    }) = &nv.value
                    {
                        if b.value() {
                            var_cfg = default;
                        } else {
                            var_cfg = VariantConfig {
                                function: false,
                                constant: false,
                            };
                        }
                    } else {
                        Err(syn::Error::new(
                            nv.span(),
                            "unknown variant-default parameter",
                        ))?
                    }
                }
                _ => {},
            }
        }

        Ok(var_cfg)
    }

    pub(crate) fn should_generate(&self) -> bool {
        self.constant || self.function
    }
}

pub(crate) fn impl_enum_variant(
    meta_attr: &[Attribute],
    data: &DataEnum,
    name: &Ident,
    generics: &Generics,
) -> syn::Result<TokenStream> {
    let meta_config = VariantConfig::parse_attr(
        if meta_attr.iter().any(|a| a.meta.path().is_ident("variant")) {
            VariantConfig {
                function: true,
                constant: false,
            }
        } else {
            VariantConfig {
                function: false,
                constant: false,
            }
        },
        meta_attr,
    )?;

    let (impl_g, type_g, where_clause) = generics.split_for_impl();

    let mut expanded = TokenStream::new();

    for variant in &data.variants {
        let cfg = VariantConfig::parse_attr(meta_config, &variant.attrs)?;
        if !cfg.should_generate() {
            continue;
        }

        let default_block = enum_impl::impl_enum_variant(variant, name)?;
        let variant_name = variant.ident.to_string();

        if cfg.function {
            let fn_name = format_ident!("default_{}", variant_name.to_snake());
            expanded.extend(quote! {
                fn #fn_name() -> Self {
                    #default_block
                }
            })
        }

        if cfg.constant {
            let const_name = format_ident!("{}", variant_name.to_snake().to_ascii_uppercase());
            expanded.extend(quote! {
                const #const_name: Self = #default_block;
            })
        }
    }

    Ok(quote! {
        impl #impl_g #name #type_g #where_clause {
            #expanded
        }
    })
}
