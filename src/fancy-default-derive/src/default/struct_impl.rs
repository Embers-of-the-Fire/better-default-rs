use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_quote, Attribute, DataStruct, Expr, Fields, FieldsNamed, FieldsUnnamed,
    Generics, LitBool, Meta, Token, Type,
};

pub(crate) fn impl_struct_const(
    data: &DataStruct,
    name: &Ident,
    generics: &Generics,
) -> syn::Result<TokenStream> {
    let block = match &data.fields {
        Fields::Named(fields) => impl_named_struct(fields, name),
        Fields::Unnamed(fields) => impl_unnamed_struct(fields, name),
        Fields::Unit => Ok(quote! { #name }),
    }?;

    let (impl_g, type_g, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_g ::fancy_default::ConstDefault for #name #type_g #where_clause {
            const DEFAULT: Self = #block;
        }
    })
}

pub(crate) fn impl_struct(
    data: &DataStruct,
    name: &Ident,
    generics: &Generics,
) -> syn::Result<TokenStream> {
    let block = match &data.fields {
        Fields::Named(fields) => impl_named_struct(fields, name),
        Fields::Unnamed(fields) => impl_unnamed_struct(fields, name),
        Fields::Unit => Ok(quote! { #name }),
    }?;

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
struct FieldConfig {
    expr: Expr,

    /// reserved for future usage
    #[allow(dead_code)]
    constant: bool,
}

impl FieldConfig {
    fn parse_attr(attrs: &[Attribute]) -> syn::Result<FieldConfig> {
        let mut constant = false;
        let mut expr: Expr = parse_quote! {::fancy_default::traits::Default::default()};

        for attr in attrs.iter().filter(|a| a.meta.path().is_ident("default")) {
            match &attr.meta {
                Meta::List(meta_list) => meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("constant") {
                        if meta.input.peek(Token![=]) {
                            let val: LitBool = meta.value()?.parse()?;
                            constant = val.value();
                        } else {
                            constant = true;
                        }

                        return Ok(());
                    }

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

        Ok(FieldConfig { constant, expr })
    }
}

fn impl_named_struct(fields: &FieldsNamed, name: &Ident) -> syn::Result<TokenStream> {
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
                FieldConfig::parse_attr(&field.attrs).map_err(|e| e.to_compile_error())?,
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

        #name {
            #(#ident,)*
        }
    }})
}

fn impl_unnamed_struct(fields: &FieldsUnnamed, name: &Ident) -> syn::Result<TokenStream> {
    let result = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(idx, field)| -> Result<_, TokenStream> {
            Ok((
                format_ident!("field_{}", idx),
                &field.ty,
                FieldConfig::parse_attr(&field.attrs).map_err(|e| e.to_compile_error())?,
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

        #name(#(#ident),*)
    }})
}
