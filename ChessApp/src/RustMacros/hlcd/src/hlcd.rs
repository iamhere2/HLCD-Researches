use syn::parse::{Parse, ParseStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use super::interface;
use super::interface::*;

#[derive(Debug)]
pub struct Hlcd {
    pub items: Vec<HlcdItem>
}

#[derive(Debug)]
pub enum HlcdItem {
    Interface(Interface)
}

impl Parse for Hlcd {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }

        Ok(Hlcd { items })
    }
}

impl Parse for HlcdItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let lookahead = input.lookahead1();
        if lookahead.peek(interface::kw::interface) {
            let interface: Interface = input.parse()?;
            Ok(HlcdItem::Interface(interface))
        } else {
            Err(lookahead.error())
        }
    }
}


impl ToTokens for Hlcd {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Hlcd { items } = self;

        let content = quote! {
            #( #items )*
        };

        tokens.extend(content)
    }
}

impl ToTokens for HlcdItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HlcdItem::Interface(interface) => interface.to_tokens(tokens)
        }
    }
}
