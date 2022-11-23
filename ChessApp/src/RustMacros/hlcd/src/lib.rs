#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]

mod hlcd;
mod interface;
mod component;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use hlcd::*;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    parse_macro_input!(input as Hlcd).to_tokens(&mut tokens);
    tokens.into()
}