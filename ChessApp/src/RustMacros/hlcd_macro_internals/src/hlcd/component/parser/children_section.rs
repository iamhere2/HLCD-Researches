pub mod child_component;

use syn::{parse::{Parse, ParseStream}, braced, punctuated::Punctuated, token, Token};
use child_component::ChildComponent;

pub mod kw {
    syn::custom_keyword!(children);
}

#[derive(Debug, Default)]
pub struct ChildrenSection {
    _children: kw::children,
    _braced: token::Brace,
    pub components: Punctuated<ChildComponent, Token![,]>
}

impl Parse for ChildrenSection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ChildrenSection {
            _children: input.parse()?,
            _braced: braced!(content in input),
            components: content.parse_terminated(ChildComponent::parse, Token![,])?,
        })
    }
}
