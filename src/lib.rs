extern crate proc_macro;
use proc_macro::TokenStream;
use std::any::{Any, TypeId};

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

#[proc_macro_attribute]
pub fn feature_gate(args: TokenStream, input: TokenStream) -> TokenStream {
    if args.is_empty() {
        panic!("No feature requirements were specified.")
    }

    let args: syn::Expr = syn::parse(args).expect("that wasn't a String");

    // let args: Meta = syn::parse(args).expect("that wasn't a Meta");

    // let args = parse_macro_input!(args as Meta);
    /*
       match args {
           Meta::Path(_) => panic!("args are a path"),
           Meta::List(_) => panic!("args are a list"),
           Meta::NameValue(_) => panic!("args are NameValue"),
       }

    */

    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #[cfg(feature = #args)]
        // #[cfg_attr(docsrs, doc(cfg(#args)))]
        #input
    };

    tokens.into()
}
