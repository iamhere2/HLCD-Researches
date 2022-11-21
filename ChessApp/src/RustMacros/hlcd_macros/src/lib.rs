#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]

mod interface;
use interface::*;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[allow(unused)]
#[proc_macro]
pub fn interface(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as Interface).generate()
}