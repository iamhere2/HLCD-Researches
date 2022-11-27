pub mod required_interface;

use syn::{parse::{Parse, ParseStream}, braced, punctuated::Punctuated, token, Token};

use required_interface::RequiredInterface;

pub mod kw {
    syn::custom_keyword!(requires);
}

#[derive(Debug, Default)]
pub struct RequiresSection {
    _requires: kw::requires,
    _braced: token::Brace,
    pub interfaces: Punctuated<RequiredInterface, Token![,]>
}

impl Parse for RequiresSection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(RequiresSection {
            _requires: input.parse()?,
            _braced: braced!(content in input),
            interfaces: content.parse_terminated(RequiredInterface::parse)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse()
    {
        let model: RequiresSection = syn::parse_str(r#"
            requires {
                depA: InterfaceA,
                depB: InterfaceB,
                depC: InterfaceC
            }
        "#).unwrap();

        let result = model.interfaces.into_iter()
            .map(|it| format!("{}:{}", it.ref_name, it.interface_name))
            .collect::<Vec<_>>().join("|");

        assert_eq!(result, "depA:InterfaceA|depB:InterfaceB|depC:InterfaceC");
    }
}