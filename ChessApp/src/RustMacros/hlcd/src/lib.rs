#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]

use proc_macro::TokenStream;
use syn::parse_macro_input;
use hlcd_macro_internals::hlcd::Hlcd;
use quote::ToTokens;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    parse_macro_input!(input as Hlcd).to_tokens(&mut tokens);
    tokens.into()
}