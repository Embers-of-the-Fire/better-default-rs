//! # `fancy-default`
//! 
//! This library provides enhancements to the standard library's `Default` derive macro.
//! 
//! ![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/fancy-default?logo=rust&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ffancy-default)
//! ![Crates.io License](https://img.shields.io/crates/l/fancy-default)
//! ![Crates.io Version](https://img.shields.io/crates/v/fancy-default)
//! 
//! ## Table of Contents
//! 
//! - [Generic default value configuration](#generic-default-value-configuration)
//! - [the `Default` macro](#fancy_defaultderivedefault)
//! - [the `ConstDefault` macro](#fancy_defaultderiveconstdefault)
//! - [the `VariantDefault` macro](#fancy_defaultderivevariantdefault)
//! - [License & MSRV](#msrv)
//! 
//! ## Generic default value configuration
//! 
//! All default value configurations in this library use the same syntax.
//! 
//! - Field configuration:
//!   - `#[default]`: Calls `core::default::Default` and uses it as the default value.
//! 
//!     **Note**: Currently `core::default::Default` is not a constant trait,
//!     so a default value must be specified when using `ConstDefault`.
//!   - `#[default = <expr>]`: Use `<expr>` as the default value for this field.
//! 
//!     **Note**: `better_default` does not use string literals to parse expressions,
//!     so you can write expressions with default values directly,
//!     like: `#[default = "foobar".to_owned()]`.
//!   - `#[default(expr = <expr>)]`: Same meaning as the previous format.
//! - Variant configuration(enum only):
//!   - `#[default]`: Set the variant as the default variant of the enum.  
//!     This attribute works the same as the standard library's `#[default]`.
//! 
//! ## `fancy_default::derive::Default`
//! 
//! **Basic Usage:**
//! 
//! ```rust
//! use fancy_default::Default;
//! 
//! #[derive(Debug, Default, PartialEq, Eq)]
//! struct Person {
//!     #[default(expr = "no-name".to_owned())]
//!     name: String,
//!     #[default]
//!     id: usize,
//!     #[default(expr = Some("unknown".to_owned()))]
//!     tag: Option<String>,
//! }
//! 
//! assert_eq!(
//!     Person::default(),
//!     Person {
//!         name: "no-name".to_owned(),
//!         id: 0,
//!         tag: Some("unknown".to_owned()),
//!     }
//! );
//! ```
//! 
//! ## `fancy_default::derive::ConstDefault`
//! 
//! **Basic Usage:**
//! 
//! ```rust
//! // this imports both `fancy_default::derive::ConstDefault`
//! // and `fancy_default::traits::ConstDefault`.
//! use fancy_default::ConstDefault;
//! 
//! #[derive(Debug, ConstDefault, PartialEq, Eq)]
//! struct Person<'a> {
//!     #[default = "no-name"]
//!     name: &'a str,
//!     #[default = 0]
//!     id: usize,
//!     #[default(expr = Some("unknown"))]
//!     tag: Option<&'a str>,
//! }
//! 
//! assert_eq!(
//!     Person::DEFAULT,
//!     Person {
//!         name: "no-name",
//!         id: 0,
//!         tag: Some("unknown"),
//!     }
//! );
//! ```
//! 
//! ## `fancy_default::derive::VariantDefault`
//! 
//! Set default values for each variant of the enumeration.This derive macro uses an additional attribute `variant`, to set how the default value are generated.
//! 
//! **Config Syntax:**
//! 
//! - `#[variant(<config>)]`:
//!   - `const`/`const = <bool>`: Whether to generate constant default values.
//!   The corresponding constant name is the UPPER_CASE version of the current enumeration.  
//!   Default: `false`.
//!   Alias: `constant`.
//!   - `func`/`func = <bool>`: Whether to generate static methods that return default values.
//!   The corresponding constant name is the snake_case version of the current enumeration and has a `default_` prefix.  
//!   Default: `true`.
//!   Alias: `fn`, `function`.
//! 
//! **Note:** This attribute can be added to an enum body or to a single variant.
//! If added to the enum body, it will override the default generated configuration.
//! 
//! **Basic Usage:**
//! 
//! ```rust
//! use fancy_default::VariantDefault;
//! 
//! #[derive(Debug, VariantDefault, PartialEq, Eq)]
//! #[variant(const)]
//! enum Enum {
//!     Plain,
//!     #[variant(const = false)]
//!     Struct {
//!         #[default(expr = "123".to_owned())]
//!         name: String,
//!     },
//!     Tuple(#[default = 10] usize),
//! }
//! 
//! 
//! assert_eq!(Enum::PLAIN, Enum::Plain);
//! assert_eq!(
//!     Enum::default_struct(),
//!     Enum::Struct {
//!         name: "123".to_owned()
//!     }
//! );
//! ```
//! 
//! ## MSRV
//! 
//! The theoretical minimum rust version of this derived macro is 1.34,
//! which allows passing `TokenStream` to `MetaList` from that version onwards.
//! 
//! ## License
//! 
//! This library is licensed under the MIT license or the Apache v2.0 license.


#![no_std]

/// Derive macros provided by the library.
pub mod derive;
/// `Default`-like traits implemented by the derive macros.
pub mod traits;

pub use derive::{ConstDefault, Default, VariantDefault};
pub use traits::ConstDefault;
