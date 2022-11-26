mod required_interface;
mod provided_interface;
mod child_component;
mod state_part;
mod interface_impl;
mod private_impl;

use syn::{Ident, parse::{Parse, ParseStream}, braced, Token, punctuated::Punctuated, ImplItem};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use required_interface::*;
use provided_interface::*;
use child_component::*;
use state_part::*;
use interface_impl::*;

pub mod kw {
    syn::custom_keyword!(component);
    syn::custom_keyword!(provides);
    syn::custom_keyword!(requires);
    syn::custom_keyword!(children);
    syn::custom_keyword!(state);
}

#[derive(Debug)]
pub struct Component {
    pub name: Ident,
    pub requires: Vec<RequiredInterface>,
    pub provides: Vec<ProvidedInterface>,
    pub children: Vec<ChildComponent>,
    pub state: Vec<StatePart>,
    pub interface_impls: Vec<InterfaceImplementation>,
    pub private_impl_items: Vec<ImplItem>,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::component>()?;
        let name: Ident = input.parse()?;

        let mut requires = vec![];
        let mut provides = vec![];
        let mut state = vec![];
        let children = vec![];
        let mut interface_impls = vec![];
        let mut private_impl_items = vec![];

        let component_content;
        braced!(component_content in input);

        while !component_content.is_empty() {
            let lookahead = component_content.lookahead1();

            if lookahead.peek(kw::requires) {
                component_content.parse::<kw::requires>()?;

                let requires_content;
                braced!(requires_content in component_content);

                let punctuated: Punctuated<_, Token![,]> = 
                    requires_content.parse_terminated(RequiredInterface::parse)?;

                requires.extend(punctuated.into_iter());

            } else if lookahead.peek(kw::provides) {
                component_content.parse::<kw::provides>()?;

                let provides_content;
                braced!(provides_content in component_content);

                let punctuated: Punctuated<ProvidedInterface, Token![,]> = 
                    provides_content.parse_terminated(ProvidedInterface::parse)?;

                provides.extend(punctuated.into_iter());

            } else if lookahead.peek(kw::state) {
                component_content.parse::<kw::state>()?;

                let state_content;
                braced!(state_content in component_content);

                let punctuated: Punctuated<StatePart, Token![,]> = 
                    state_content.parse_terminated(StatePart::parse)?;

                state.extend(punctuated.into_iter());
            } else if lookahead.peek(syn::token::Impl) {
                component_content.parse::<syn::token::Impl>()?;

                let lookahead = component_content.lookahead1();
                if lookahead.peek(syn::token::Brace) {
                    
                    let impl_content;
                    braced!(impl_content in component_content);

                    let punctuated: Punctuated<ImplItem, Token![;]> = 
                        impl_content.parse_terminated(ImplItem::parse)?;

                    private_impl_items.extend(punctuated.into_iter());

                    
                } else if lookahead.peek(syn::Ident) {
                    
                    let interface_name = component_content.parse::<Ident>()?;

                    let impl_content;
                    braced!(impl_content in component_content);

                    let punctuated: Punctuated<ImplItem, Token![;]> = 
                        impl_content.parse_terminated(ImplItem::parse)?;

                    let interface_impl = InterfaceImplementation { 
                        interface_name, 
                        items: punctuated.into_iter().collect()
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

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream) {

        let Component {
            name,
            // requires,
            provides,
            // children,
            state,
            interface_impls,
            private_impl_items,
            ..
        } = self;

        let component_struct_name = syn::Ident::new(&format!("{}", name), name.span());

        let state_fields = state.iter().map(|_s| quote!{
            // field: type
        }).collect::<Vec<_>>();

        let component_struct = quote! {
            pub struct #component_struct_name {
                #( #state_fields )*
            }
        };

        // let interface_ref_name = syn::Ident::new(&format!("{}Ref", base_interface_name), base_interface_name.span());
        // let interface_ref_type = quote! {
        //     pub type #interface_ref_name = std::rc::Rc<std::cell::RefCell<dyn #interface_trait_name>>;
        // };

        // let provider_trait_name = syn::Ident::new(&format!("{}Provider", base_interface_name), base_interface_name.span());
        // let provider_trait = quote! {
        //     pub trait #provider_trait_name {
        //         fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> #interface_ref_name; 
        //     }
        // };

        let self_interface_impls_providers = provides.iter().map(|p| {
            let provider_trait_name = syn::Ident::new(&format!("{}Provider", p.name), p.name.span());
            let interface_ref_name = syn::Ident::new(&format!("{}Ref", p.name), p.name.span());

            quote! {
                impl #provider_trait_name for #component_struct_name {
                    fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> #interface_ref_name {
                        it
                    }
                }
            }
        }).collect::<Vec<_>>();

        let self_interface_impls = interface_impls.iter().map(|imp| {
            let InterfaceImplementation {
                interface_name,
                items
            } = imp;

            let interface_trait_name = syn::Ident::new(&format!("{}Interface", interface_name), interface_name.span());
            
            quote! {
                impl #interface_trait_name for #component_struct_name {
                    #( #items )*
                }
            }

        }).collect::<Vec<_>>();

        let private_impl = quote! {
            impl #component_struct_name {
                #( #private_impl_items )*
            }
        };

        let component = quote! {
            #component_struct
            #private_impl
            #( #self_interface_impls_providers )*
            #( #self_interface_impls )*
        };
        
        // #private_impl
        // #self_interface_impls
        // #delegated_interface_providers

        tokens.extend(component)
    }
}
