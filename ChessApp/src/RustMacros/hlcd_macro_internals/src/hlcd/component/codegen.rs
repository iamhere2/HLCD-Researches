mod component_struct;
mod constructor;

use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use super::Component;
use super::parser::interface_impl::InterfaceImplementation;

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream) {

        let Component {
            name,
            requires,
            provides,
            // children,
            // state,
            interface_impls,
            private_impl_items,
            ..
        } = self;

        let component_struct_name = syn::Ident::new(&format!("{}", name), name.span());
        let component_ref_name = syn::Ident::new(&format!("{}InstanceRef", name), name.span());

        let dependency_accessors = requires.interfaces.iter().map(|r| {
            let ref_name = &r.ref_name;
            let ref_name_mut = syn::Ident::new(&format!("{}_mut", ref_name), ref_name.span());
            let interface_trait_name = syn::Ident::new(&format!("{}Interface", r.interface_name), r.interface_name.span());
            quote! { 
                fn #ref_name(&self) -> std::cell::Ref<dyn #interface_trait_name> {
                    std::cell::RefCell::borrow(&self.#ref_name)
                } 

                fn #ref_name_mut(&self) -> std::cell::RefMut<dyn #interface_trait_name> {
                    std::cell::RefCell::borrow_mut(&self.#ref_name)
                } 
            }

        }).collect::<Vec<_>>();

        let component_ref = quote! {
            pub type #component_ref_name = std::rc::Rc<std::cell::RefCell<#component_struct_name>>;
        };

        let component_struct = component_struct::gen_struct(self);

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
            #( #self_interface_impls_providers )*
            #( #self_interface_impls )*
        };
        
        // #self_interface_impls
        // #delegated_interface_providers

        tokens.extend(component)
    }
}
