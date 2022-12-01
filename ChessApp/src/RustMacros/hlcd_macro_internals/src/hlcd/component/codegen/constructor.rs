use proc_macro2::TokenStream;
use quote::quote;
use crate::hlcd::component::{Component, parser::{state_section::state_part::StatePart, children_section::child_component::PortLink}};
use super::{required_interface_model::{RequiredInterfaceModel, gen_port_name}, component_struct::struct_name, provider_trait_impl::provider_trait_name};

pub(super) fn gen_constructor(component: &Component) -> TokenStream {

    let requires: Vec::<_> = component.requires.interfaces.iter().map(|r| r.into()).collect();

    let dependency_ref_name_type_list = requires.iter().map(
        |RequiredInterfaceModel { port_name, interface_ref_name, .. }| {
            quote! { 
                #port_name : &#interface_ref_name
            }
        }).collect::<Vec<_>>();

    let dependency_ref_assignments = requires.iter().map(
        |RequiredInterfaceModel { port_name, .. }| {
            let local_name = port_name.clone();
            quote! { 
                #port_name: ::std::rc::Rc::clone(&#local_name) 
            }
        }).collect::<Vec<_>>();

    let dependency_ref_locals_creations = requires.iter().map(
        |RequiredInterfaceModel { port_name, .. }| {
            quote! { 
                let #port_name = ::std::rc::Rc::clone(#port_name) 
            }
        }).collect::<Vec<_>>();
    
    let state_initial_assignments = component.state.parts.iter().map(
        |StatePart { field, initial_value, .. }| {
            let field = &field.ident;
            quote!{ 
                #field : #initial_value
            }
        }).collect::<Vec<_>>();

    let children_local_assignments = component.children.components.iter().map(
        |c| {
            let local_name = &c.child_name.clone();
            quote!{ 
                #local_name: #local_name
            }
        }).collect::<Vec<_>>();

    let children_local_creations = component.children.components.iter().map(
        |c| {
            let child_struct_name = struct_name(&c.component_type); 
            let local_name = &c.child_name.clone();
            let port_args = c.ports.iter().map(|p| {
                match p {
                    PortLink::Explicit(p) => {
                        // get interface from children with provider
                        let child_local_name = p.child_name.clone();
                        let provider_trait_name = provider_trait_name(&p.interface_name);
                        quote! { 
                            &#provider_trait_name::get(::std::rc::Rc::clone(&#child_local_name))
                        }
                    },
                    PortLink::Implicit(p) => {
                        // just clone from required interface local
                        let local_interface_ref_name = gen_port_name(&p.interface_name);
                        quote! { 
                            &::std::rc::Rc::clone(&#local_interface_ref_name) 
                        }
                    }
                }
            }).collect::<Vec<_>>();

            quote! {
                let #local_name = ::std::rc::Rc::new(::std::cell::RefCell::new(#child_struct_name::new(
                    #( #port_args , )*
                ))) 
            }
        }).collect::<Vec<_>>();
    
    quote! {
        pub fn new( 
            #( #dependency_ref_name_type_list , )*
        ) -> Self {

            #( #dependency_ref_locals_creations ; )*
            #( #children_local_creations ; )*

            Self { 
                #( #dependency_ref_assignments , )*
                #( #state_initial_assignments , )*
                #( #children_local_assignments , )*
            }
        }
    }
}