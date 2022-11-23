use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub struct RequiredInterface { }

impl Parse for RequiredInterface {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(RequiredInterface { })
    }
}