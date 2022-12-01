use syn::{parse::{Parse, ParseStream}, Ident, Token};

#[derive(Debug)]
pub enum RequiredInterface { 
    Named(NamedInterfacePort),
    Unnamed(UnnamedInterfacePort)
}

impl Parse for RequiredInterface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first_ident: Ident = input.parse()?;
        let la = input.lookahead1();
        if la.peek(Token![:]) {
            Ok(RequiredInterface::Named(NamedInterfacePort {
                port_name: first_ident,
                _colon: input.parse()?,
                interface_name: input.parse()?
            }))
        } else {
            Ok(RequiredInterface::Unnamed(UnnamedInterfacePort {
                interface_name: first_ident
            }))
        }
    }
}

#[derive(Debug)]
pub struct NamedInterfacePort { 
    pub port_name: Ident,
    _colon: Token![:],
    pub interface_name: Ident
}

#[derive(Debug)]
pub struct UnnamedInterfacePort { 
    pub interface_name: Ident
}