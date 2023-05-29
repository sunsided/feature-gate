extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn feature_gate(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    if args.is_empty() {
        panic!("No feature name was specified")
    }

    let input = parse_macro_input!(input as DeriveInput);
    let attributes = format!("{args}");

    let tokens = quote! {
        #[cfg(not(feature = #attributes))]
        #[cfg_attr(docsrs, doc(cfg(feature = #attributes)))]
        #input
    };

    tokens.into()
}
