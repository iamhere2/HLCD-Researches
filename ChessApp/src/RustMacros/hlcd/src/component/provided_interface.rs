use syn::{parse::{Parse, ParseStream}, Ident};

#[derive(Debug)]
pub struct ProvidedInterface { 
    pub name: Ident
}

impl Parse for ProvidedInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        Ok(ProvidedInterface {
            name
        })
    }
}