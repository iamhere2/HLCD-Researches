pub mod provided_interface;

use syn::{parse::{Parse, ParseStream}, braced, punctuated::Punctuated, token, Token};
use provided_interface::ProvidedInterface;

pub mod kw {
    syn::custom_keyword!(provides);
}

#[derive(Debug, Default)]
pub struct ProvidesSection {
    _provides: kw::provides,
    _braced: token::Brace,
    pub interfaces: Punctuated<ProvidedInterface, Token![,]>
}

impl Parse for ProvidesSection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ProvidesSection {
            _provides: input.parse()?,
            _braced: braced!(content in input),
            interfaces: content.parse_terminated(ProvidedInterface::parse)?,
        })
    }
}
