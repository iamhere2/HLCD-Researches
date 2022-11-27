// use proc_macro2::TokenStream;
// use quote::ToTokens;
use syn::parse::Parse;

#[derive(Debug)]
pub struct StatePart { }

impl Parse for StatePart {
    fn parse(_input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(StatePart { })
    }
}