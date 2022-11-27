pub mod state_part;

use syn::{parse::{Parse, ParseStream}, braced, punctuated::Punctuated, token, Token};

use state_part::StatePart;

pub mod kw {
    syn::custom_keyword!(state);
}

#[derive(Debug, Default)]
pub struct StateSection {
    _state: kw::state,
    _braced: token::Brace,
    pub parts: Punctuated<StatePart, Token![,]>
}

impl Parse for StateSection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(StateSection {
            _state: input.parse()?,
            _braced: braced!(content in input),
            parts: content.parse_terminated(StatePart::parse)?,
        })
    }
}
