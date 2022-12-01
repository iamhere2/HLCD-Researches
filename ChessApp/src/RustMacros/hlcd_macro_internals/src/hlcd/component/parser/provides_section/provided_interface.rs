use syn::{parse::{Parse, ParseStream}, Ident};

mod kw {
    syn::custom_keyword!(by);
}

#[derive(Debug)]
pub struct ProvidedInterface { 
    pub interface_name: Ident,
    _via: Option<kw::by>,
    pub child_component_name: Option<Ident>
}

impl Parse for ProvidedInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let interface_name = input.parse()?;
        let mut _via = None;
        let mut child_component_name = None;

        let lookahead = input.lookahead1();

        if lookahead.peek(kw::by) {
            _via = input.parse()?;
            child_component_name = input.parse()?;
        } 

        Ok(ProvidedInterface {
            interface_name,
            _via,
            child_component_name
        })
    }
}