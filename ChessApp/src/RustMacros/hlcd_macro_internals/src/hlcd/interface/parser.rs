use syn::{parse::{Parse, ParseStream}, Ident, braced};

use super::Interface;

pub mod kw {
    syn::custom_keyword!(interface);
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::interface>()?;
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(Interface {
            name,
            items
        })
    }
}
