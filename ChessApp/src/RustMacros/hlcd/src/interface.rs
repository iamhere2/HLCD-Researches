use syn::{Ident, parse::{Parse, ParseStream}, braced, TraitItemMethod};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub mod kw {
    syn::custom_keyword!(interface);
}

#[derive(Debug)]
pub struct Interface {
    pub name: Ident,
    pub items: Vec<TraitItemMethod>
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<kw::interface>()?;
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(Interface {
            name,
            items
        })
    }
}

impl ToTokens for Interface {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Interface {
            name: base_interface_name,
            items
        } = self;
        
        let interface_trait_name = syn::Ident::new(&format!("{}Interface", base_interface_name), base_interface_name.span());
        let interface_trait = quote! {
            pub trait #interface_trait_name {
                #( #items )*
            }
        };

        let interface_ref_name = syn::Ident::new(&format!("{}Ref", base_interface_name), base_interface_name.span());
        let interface_ref_type = quote! {
            pub type #interface_ref_name = std::rc::Rc<std::cell::RefCell<dyn #interface_trait_name>>;
        };

        let provider_trait_name = syn::Ident::new(&format!("{}Provider", base_interface_name), base_interface_name.span());
        let provider_trait = quote! {
            pub trait #provider_trait_name {
                fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> #interface_ref_name; 
            }
        };

        let interface = quote! {
            #interface_trait
            #interface_ref_type
            #provider_trait
        };

        tokens.extend(interface)
    }
}
