use syn::{Ident, parse::{Parse, ParseStream}, braced, TraitItemMethod};

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub base_interface_name: Ident,
    pub items: Vec<TraitItemMethod>
}

impl Parse for InterfaceDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let base_interface_name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(InterfaceDefinition {
            base_interface_name,
            items
        })
    }
}