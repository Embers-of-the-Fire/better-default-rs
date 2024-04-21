use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse2, spanned::Spanned, Attribute, DataEnum, Expr, ExprLit, Fields, FieldsNamed,
    FieldsUnnamed, Generics, Lit, LitBool, Meta, MetaNameValue, Token, Type, Variant,
};

pub(crate) fn impl_enum_const(
    data: &DataEnum,
    name: &Ident,
    generics: &Generics,
) -> syn::Result<TokenStream> {
    let configs = data
        .variants
        .iter()
        .map(|v| VariantConfig::parse_attr(&v.attrs).map(|c| (v, c)))
        .filter(|v| v.is_err() || v.as_ref().is_ok_and(|(_, c)| c.default))
        .collect::<Result<Vec<_>, _>>()?;

    let variant = if let Some((var, _)) = configs.first() {
        var
    } else if configs.len() > 1 {
        let mut err = syn::Error::new(Span::call_site(), "more than one variant are set `default`");
        configs.iter().for_each(|(v, _)| {
            err.combine(syn::Error::new(
                v.ident.span(),
                "more than one variant are set `default`",
            ))
        });
        return Err(err);
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "at least one variant should be set `default`",
        ));
    };

    let block = impl_enum_variant(variant, name)?;
    let (impl_g, type_g, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_g ::fancy_default::ConstDefault for #name #type_g #where_clause {
            const DEFAULT: Self = #block;
        }
    })
}

pub(crate) fn impl_enum(
    data: &DataEnum,
    name: &Ident,
    generics: &Generics,
) -> syn::Result<TokenStream> {
    let configs = data
        .variants
        .iter()
        .map(|v| VariantConfig::parse_attr(&v.attrs).map(|c| (v, c)))
        .filter(|v| v.is_err() || v.as_ref().is_ok_and(|(_, c)| c.default))
        .collect::<Result<Vec<_>, _>>()?;

    let variant = if let Some((var, _)) = configs.first() {
        var
    } else if configs.len() > 1 {
        let mut err = syn::Error::new(Span::call_site(), "more than one variant are set `default`");
        configs.iter().for_each(|(v, _)| {
            err.combine(syn::Error::new(
                v.ident.span(),
                "more than one variant are set `default`",
            ))
        });
        return Err(err);
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "at least one variant should be set `default`",
        ));
    };

    let block = impl_enum_variant(variant, name)?;
    let (impl_g, type_g, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_g ::fancy_default::traits::Default for #name #type_g #where_clause {
            fn default() -> Self {
                #block
            }
        }
    })
}

#[derive(Clone)]
#[non_exhaustive]
struct VariantFieldConfig {
    expr: Expr,
}

impl VariantFieldConfig {
    fn parse_attr(attrs: &[Attribute]) -> syn::Result<VariantFieldConfig> {
        let mut expr: Expr = parse2(quote! {::fancy_default::traits::Default::default()})?;

        for attr in attrs.iter().filter(|a| a.meta.path().is_ident("default")) {
            match &attr.meta {
                Meta::List(meta_list) => meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("expr") {
                        let val: Expr = meta.value()?.parse()?;
                        expr = val;
                        return Ok(());
                    }

                    Err(meta.error("unrecognized default parameter"))
                })?,
                Meta::NameValue(nv) => expr = nv.value.clone(),
                _ => {}
            }
        }

        Ok(VariantFieldConfig { expr })
    }
}

pub(crate) fn impl_enum_variant(variant: &Variant, name: &Ident) -> syn::Result<TokenStream> {
    let fields = &variant.fields;
    let ident = &variant.ident;
    let tokens = match fields {
        Fields::Named(field) => impl_named_struct(field, ident, name),
        Fields::Unnamed(field) => impl_unnamed_struct(field, ident, name),
        Fields::Unit => Ok(quote! { #name::#ident }),
    }?;

    Ok(tokens)
}

fn impl_named_struct(
    fields: &FieldsNamed,
    variant_name: &Ident,
    enum_name: &Ident,
) -> syn::Result<TokenStream> {
    let result = fields
        .named
        .iter()
        .map(|field| -> Result<_, TokenStream> {
            Ok((
                field
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| panic!("unexpected internal error: unnamed field")),
                &field.ty,
                VariantFieldConfig::parse_attr(&field.attrs).map_err(|e| e.to_compile_error())?,
            ))
        })
        .collect::<Result<Vec<_>, _>>();

    let result = match result {
        Ok(r) => r,
        Err(e) => return Ok(e),
    };

    let (ident, ty, expr): (Vec<&Ident>, Vec<&Type>, Vec<Expr>) = result
        .into_iter()
        .map(|(a, b, c)| (a, b, c.expr))
        .multiunzip();

    Ok(quote! {{
        #(
            let #ident: #ty = #expr;
        )*

        #enum_name::#variant_name {
            #(#ident,)*
        }
    }})
}

fn impl_unnamed_struct(
    fields: &FieldsUnnamed,
    variant_name: &Ident,
    enum_name: &Ident,
) -> syn::Result<TokenStream> {
    let result = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(idx, field)| -> Result<_, TokenStream> {
            Ok((
                format_ident!("field_{}", idx),
                &field.ty,
                VariantFieldConfig::parse_attr(&field.attrs).map_err(|e| e.to_compile_error())?,
            ))
        })
        .collect::<Result<Vec<_>, _>>();

    let result = match result {
        Ok(r) => r,
        Err(e) => return Ok(e),
    };

    let (ident, ty, expr): (Vec<Ident>, Vec<&Type>, Vec<Expr>) = result
        .into_iter()
        .map(|(a, b, c)| (a, b, c.expr))
        .multiunzip();

    Ok(quote! {{
        #(
            let #ident: #ty = #expr;
        )*

        #enum_name::#variant_name(#(#ident),*)
    }})
}

#[derive(Clone)]
#[non_exhaustive]
struct VariantConfig {
    default: bool,
}

impl VariantConfig {
    fn parse_attr(attrs: &[Attribute]) -> syn::Result<VariantConfig> {
        let mut default = false;

        for attr in attrs.iter().filter(|a| a.meta.path().is_ident("default")) {
            match &attr.meta {
                Meta::List(meta_list) => meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") {
                        if meta.input.peek(Token![=]) {
                            let val: LitBool = meta.value()?.parse()?;
                            default = val.value();
                        } else {
                            default = true;
                        }
                        return Ok(());
                    }

                    Err(meta.error("unrecognized `default` parameter"))
                })?,
                Meta::NameValue(mv) => {
                    if let MetaNameValue {
                        value:
                            Expr::Lit(ExprLit {
                                lit: Lit::Bool(b), ..
                            }),
                        ..
                    } = mv
                    {
                        default = b.value();
                    } else {
                        Err(syn::Error::new(
                            attr.meta.span(),
                            "unrecognized `default` parameter",
                        ))?;
                    }
                }
                Meta::Path(_) => default = true,
            }
        }

        Ok(VariantConfig { default })
    }
}
