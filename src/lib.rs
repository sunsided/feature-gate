//! Simple documented feature gates
//!
//! This crates provides the [`feature_gate`] and [`feature_gate_ex`]
//! macros for simple `#[cfg(feature = "...")]` macros that are properly
//! documented on docs.rs.
//!
//! ## Stable Rust
//!
//! Note that for it to work properly on stable Rust, the following needs to be
//! added to `Cargo.toml` for the time being (see [Metadata for custom builds](https://docs.rs/about/metadata)):
//!
//! ```toml
//! [package.metadata.docs.rs]
//! all-features = true
//! rustdoc-args = ["--cfg", "docsrs"]
//! ```
//!
//! ## Example
//!
//! The `feature_gate` macro allows the specification of a single feature:
//!
//! ```
//! use feature_gate::feature_gate;
//!
//! #[feature_gate("test")]
//! struct FeatureGated;
//!
//! #[test]
//! fn it_works() {
//!     let _ = FeatureGated {};
//! }
//! ```
//!
//! The `feature_gate_ex` macro allows the specification of a complex set of requirements:
//!
//! ```
//! use feature_gate::feature_gate_ex;
//!
//! #[feature_gate_ex(any(test, feature = "test"))]
//! struct FeatureGated;
//!
//! #[test]
//! fn it_works() {
//!     let _ = FeatureGated {};
//! }
//! ```

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

/// The `#[feature_gate(...)]` macro conditionally compiles the element it
/// is applied to if the provided feature is enabled.
///
/// It behaves as if `cfg!(feature = ...)` is applied to the element.
/// See [`feature_gate_ex`] if more complex configuration is required.
///
/// It addition, it conditionally assigns the `#[doc(cfg(...))]` attribute
/// to help docs.rs document the feature dependency.
/// See [doc(cfg)] for more information.
///
/// [doc(cfg)]: https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
///
/// ## Example
///
/// ```
/// use feature_gate::feature_gate;
///
/// #[feature_gate("test")]
/// struct Test;
///
/// #[test]
/// fn it_works() {
///     let _ = Test {};
/// }
/// ```
#[proc_macro_attribute]
pub fn feature_gate(args: TokenStream, input: TokenStream) -> TokenStream {
    if args.is_empty() {
        panic!("No feature requirements were specified")
    }

    // For a single-string input.
    let args: syn::Expr = match syn::parse(args) {
        Ok(args) => args,
        Err(_) => panic!("Expected a single feature name as input"),
    };

    let input = parse_macro_input!(input as DeriveInput);

    // See https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
    let tokens = quote! {
        #[cfg(feature = #args)]
        #[cfg_attr(any(docsrs /*, dox */), doc(cfg(feature = #args)))]
        #input
    };

    tokens.into()
}

/// The `#[feature_gate_ex(...)]` macro conditionally compiles the element it
/// is applied to if the provided configureation is enabled.
///
/// It behaves as if `cfg!(...)` is applied to the element. See
/// [`feature_gate`] for a simpler version of the macro.
///
/// It addition, it conditionally assigns the `#[doc(cfg(...))]` attribute
/// to help docs.rs document the feature dependency.
/// See [doc(cfg)] for more information.
///
/// [doc(cfg)]: https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
///
/// ## Example
///
/// The following example fails to compile because the `test` feature
/// is not declared:
///
/// ```
/// use feature_gate::feature_gate_ex;
///
/// #[feature_gate_ex(test)]
/// struct Test;
///
/// #[feature_gate_ex(feature = "test")]
/// struct Test2;
///
/// #[feature_gate_ex(any(test, feature = "test"))]
/// struct Test3;
///
/// #[test]
/// fn it_works() {
///     let _ = Test {};
///     let _ = Test2 {};
///     let _ = Test3 {};
/// }
/// ```
#[proc_macro_attribute]
pub fn feature_gate_ex(args: TokenStream, input: TokenStream) -> TokenStream {
    if args.is_empty() {
        panic!("No feature requirements were specified.")
    }

    let args: Meta = match syn::parse(args) {
        Ok(args) => args,
        Err(_) => panic!("Expected a configuration specification as input"),
    };

    let input = parse_macro_input!(input as DeriveInput);

    // See https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
    let tokens = quote! {
        #[cfg(#args)]
        #[cfg_attr(any(docsrs /*, dox */), doc(cfg(#args)))]
        #input
    };

    tokens.into()
}
