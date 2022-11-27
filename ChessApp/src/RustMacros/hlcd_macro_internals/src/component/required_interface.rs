use syn::{parse::{Parse, ParseStream}, Ident, Token};

#[derive(Debug)]
pub struct RequiredInterface { 
    pub ref_name: Ident,
    _colon: Token![:],
    pub interface_name: Ident
}

impl Parse for RequiredInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ref_name = input.parse()?;
        let _colon = input.parse()?;
        let interface_name = input.parse()?;
        Ok(RequiredInterface {
            ref_name,
            _colon,
            interface_name
        })
    }
}