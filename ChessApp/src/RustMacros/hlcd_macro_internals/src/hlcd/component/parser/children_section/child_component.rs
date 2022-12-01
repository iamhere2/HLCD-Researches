use syn::{Ident, Token, punctuated::Punctuated, token::Paren, parse::{Parse, ParseStream}, parenthesized};

#[derive(Debug)]
pub struct ChildComponent { 
    pub child_name: Ident,
    _colon: Token![:],
    pub component_type: Ident,
    _paren: Paren,
    pub ports: Punctuated<PortLink, Token![,]>
}

#[derive(Debug)]
pub enum PortLink { 
    Explicit(ExplicitPortLink),
    Implicit(ImplicitPortLink)
}

#[derive(Debug)]
pub struct ExplicitPortLink { 
    pub child_name: Ident,
    _for: Token![for],
    pub interface_name: Ident
}

impl Parse for ExplicitPortLink {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ExplicitPortLink { 
            child_name: input.parse()?,
            _for: input.parse()?,
            interface_name: input.parse()? 
        })
    }
}

#[derive(Debug)]
pub struct ImplicitPortLink { 
    pub interface_name: Ident
}

impl Parse for ImplicitPortLink {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ImplicitPortLink { 
            interface_name: input.parse()? 
        })
    }
}

impl Parse for PortLink {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first_ident: Ident = input.parse()?;
        let la = input.lookahead1();
        if la.peek(Token![for]) {
            Ok(PortLink::Explicit(ExplicitPortLink {
                child_name: first_ident,
                _for: input.parse()?,
                interface_name: input.parse()?
            }))
        } else {
            Ok(PortLink::Implicit(ImplicitPortLink {
                interface_name: first_ident
            }))
        }
    }
}

impl Parse for ChildComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ChildComponent {
            child_name: input.parse()?,
            _colon: input.parse()?,
            component_type: input.parse()?,
            _paren: parenthesized!(content in input),
            ports: content.parse_terminated(PortLink::parse)?
        })
    }
}