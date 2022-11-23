use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug)]
pub struct StatePart { }

impl ToTokens for StatePart {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        todo!()
    }
}