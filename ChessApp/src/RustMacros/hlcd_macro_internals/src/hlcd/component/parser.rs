pub mod requires_section;
pub mod state_section;
pub mod provides_section;
pub mod children_section;
pub mod interface_impl;   
// pub mod private_impl;

use syn::{parse::{Parse, ParseStream}, Ident, braced};
use super::Component;
use requires_section::RequiresSection;
use state_section::StateSection;
use provides_section::ProvidesSection;
use interface_impl::InterfaceImplementation;
use children_section::ChildrenSection;

pub mod kw {
    syn::custom_keyword!(component);
    syn::custom_keyword!(state);
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::component>()?;
        let name: Ident = input.parse()?;

        let mut requires: RequiresSection = Default::default();
        let mut provides: ProvidesSection = Default::default();
        let mut state: StateSection = Default::default();
        let mut children: ChildrenSection = Default::default();
        let mut interface_impls = vec![];
        let mut private_impl_items = vec![];

        let content;
        braced!(content in input);

        while !content.is_empty() {
            let la = content.lookahead1();

            if la.peek(requires_section::kw::requires) {
                requires = content.parse()?;
            } else if la.peek(provides_section::kw::provides) {
                provides = content.parse()?;
            } else if la.peek(state_section::kw::state) {
                state = content.parse()?;
            } else if la.peek(children_section::kw::children) {
                children = content.parse()?;
            } else if la.peek(syn::token::Impl) {
                content.parse::<syn::token::Impl>()?;

                let lookahead = content.lookahead1();
                if lookahead.peek(syn::token::Brace) {
                    
                    let impl_content;
                    braced!(impl_content in content);

                    while !impl_content.is_empty() {
                        private_impl_items.push(
                            impl_content.parse()?    
                        )    
                    }
                    
                } else if lookahead.peek(syn::Ident) {
                    
                    let interface_name = content.parse::<Ident>()?;

                    let impl_content;
                    braced!(impl_content in content);

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
            } else { 
                return Err(la.error()) 
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
