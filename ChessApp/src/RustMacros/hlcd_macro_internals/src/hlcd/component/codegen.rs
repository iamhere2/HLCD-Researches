mod component_struct;
mod constructor;
mod required_interface_model;
mod provider_trait_impl;

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use self::provider_trait_impl::gen_provider_trait_impls;
use self::required_interface_model::RequiredInterfaceModel;

use super::Component;
use super::parser::interface_impl::InterfaceImplementation;

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream) {

        let Component {
            name,
            requires,
            interface_impls,
            private_impl_items,
            ..
        } = self;

        let requires: Vec::<_> = requires.interfaces.iter().map(|r| r.into()).collect();

        let component_struct_name = syn::Ident::new(&format!("{}", name), name.span());
        let component_ref_name = syn::Ident::new(&format!("{}InstanceRef", name), name.span());

        let dependency_accessors = requires.iter().map(
            |RequiredInterfaceModel { port_name, interface_name, .. }| {
                let port_name_mut = syn::Ident::new(&format!("{}_mut", port_name), interface_name.span());
                let interface_trait_name = syn::Ident::new(&format!("{}Interface", interface_name), interface_name.span());
                quote! { 
                    fn #port_name(&self) -> std::cell::Ref<dyn #interface_trait_name> {
                        std::cell::RefCell::borrow(&self.#port_name)
                    } 

                    fn #port_name_mut(&self) -> std::cell::RefMut<dyn #interface_trait_name> {
                        std::cell::RefCell::borrow_mut(&self.#port_name)
                    } 
                }
            }).collect::<Vec<_>>();

        let component_ref = quote! {
            pub type #component_ref_name = std::rc::Rc<std::cell::RefCell<#component_struct_name>>;
        };

        let component_struct = component_struct::gen_struct(self);

        let provider_trait_impls = gen_provider_trait_impls(self);

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

        let constructor = constructor::gen_constructor(self);

        let private_impl = quote! {
            impl #component_struct_name {
                #constructor
                #( #dependency_accessors )*
                #( #private_impl_items )*
            }
        };

        let component = quote! {
            #component_ref
            #component_struct
            #private_impl
            #( #provider_trait_impls )*
            #( #self_interface_impls )*
        };
        
        tokens.extend(component)
    }
}
