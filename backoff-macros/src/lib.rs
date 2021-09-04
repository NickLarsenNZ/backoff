// #![feature(log_syntax)]
// log_syntax!(true);

extern crate proc_macro;
extern crate proc_macro2;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

// Todo docs
#[proc_macro_attribute]
pub fn retry(attr: TokenStream, function: TokenStream) -> TokenStream {
    // Implementer must supply the backoff strategy
    assert!(!attr.is_empty());

    #[allow(unused_mut)]
    let mut function = parse_macro_input!(function as ItemFn);
    let attr = parse_macro_input!(attr as AttributeArgs);
    let attr = attr.into_iter().next();
    let vis = function.vis;
    let sig = function.sig;
    let block = function.block;

    return quote! {
        //log_syntax! {
            #vis #sig {
                ::backoff::retry(#attr::default(), || #block)
            }
        //}
    }
    .into();
}
