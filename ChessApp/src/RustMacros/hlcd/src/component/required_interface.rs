use syn::{parse::{Parse, ParseStream}, Ident, Token};

#[derive(Debug)]
pub struct RequiredInterface { 
    pub ref_name: Ident,
    pub interface_name: Ident
}

impl Parse for RequiredInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ref_name = input.parse()?;
        input.parse::<Token![:]>()?;
        let interface_name = input.parse()?;

        Ok(RequiredInterface {
            ref_name,
            interface_name,
        })
    }
}