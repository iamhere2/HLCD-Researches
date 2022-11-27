pub mod requires_section;
pub mod state_section;
pub mod provided_interface;
pub mod child_component;
pub mod interface_impl;   
pub mod private_impl;

use syn::{parse::{Parse, ParseStream}, Ident, braced, punctuated::Punctuated, Token};
use super::Component;
use requires_section::RequiresSection;
use state_section::StateSection;
use provided_interface::ProvidedInterface;
use interface_impl::InterfaceImplementation;
use child_component::ChildComponent;

pub mod kw {
    syn::custom_keyword!(component);
    syn::custom_keyword!(provides);
    syn::custom_keyword!(children);
    syn::custom_keyword!(state);
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::component>()?;
        let name: Ident = input.parse()?;

        let mut requires: RequiresSection = Default::default();
        let mut provides = vec![];
        let mut state: StateSection = Default::default();
        let children: Vec<ChildComponent> = vec![];
        let mut interface_impls = vec![];
        let mut private_impl_items = vec![];

        let component_content;
        braced!(component_content in input);

        while !component_content.is_empty() {
            let lookahead = component_content.lookahead1();

            if lookahead.peek(requires_section::kw::requires) {
                
                requires = component_content.parse()?;

            } else if lookahead.peek(kw::provides) {
                component_content.parse::<kw::provides>()?;

                let provides_content;
                braced!(provides_content in component_content);

                let punctuated: Punctuated<ProvidedInterface, Token![,]> = 
                    provides_content.parse_terminated(ProvidedInterface::parse)?;

                provides.extend(punctuated.into_iter());

            } else if lookahead.peek(state_section::kw::state) {

                state = component_content.parse()?;

            } else if lookahead.peek(syn::token::Impl) {
                component_content.parse::<syn::token::Impl>()?;

                let lookahead = component_content.lookahead1();
                if lookahead.peek(syn::token::Brace) {
                    
                    let impl_content;
                    braced!(impl_content in component_content);

                    while !impl_content.is_empty() {
                        private_impl_items.push(
                            impl_content.parse()?    
                        )    
                    }
                    
                } else if lookahead.peek(syn::Ident) {
                    
                    let interface_name = component_content.parse::<Ident>()?;

                    let impl_content;
                    braced!(impl_content in component_content);

                    let mut items = vec![]; 
                    while !impl_content.is_empty() {
                        items.push(
                            impl_content.parse()?    
                        )    
                    }

                    let interface_impl = InterfaceImplementation { 
                        interface_name, 
                        items: items.into_iter().collect()
                    };

                    interface_impls.push(interface_impl);


                } else {
                    return Err(lookahead.error()) 
                }
            // } else if lookahead.peek(kw::children) {
            //     todo!()
            } else { 
                return Err(lookahead.error()) 
            }
        }

        Ok(Component {
            name,
            requires,
            provides,
            children,
            state,
            interface_impls,
            private_impl_items,
        })
    }
}
