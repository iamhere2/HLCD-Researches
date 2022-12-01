use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;

use crate::hlcd::component::parser::state_section::state_part::StatePart;
use super::{super::Component, required_interface_model::RequiredInterfaceModel};

pub(super) fn struct_name(component_name: &Ident) -> Ident {
    syn::Ident::new(&format!("{}", component_name), component_name.span())
} 

pub(super) fn gen_struct(component: &Component) -> TokenStream {

    let struct_name = struct_name(&component.name);
    
    let state_fields = component.state.parts.iter().map(|StatePart{ field, .. }| {
        quote! { 
            #field 
        }
    }).collect::<Vec<_>>();

    let dependency_ref_fields = component.requires.interfaces.iter().map(|r| {
        let RequiredInterfaceModel { 
            port_name, 
            interface_ref_name,
            .. 
        } = r.into();

        quote! {
            #port_name : #interface_ref_name
        }

    }).collect::<Vec<_>>();

    let children_component_fields = component.children.components.iter().map(|c| {
        let field_name = c.child_name.clone();
        let instance_ref_name = instance_ref_name(&c.component_type);

        quote! {
            #field_name : #instance_ref_name
        }

    }).collect::<Vec<_>>();

    quote! {
        #[allow(unused)]
        pub struct #struct_name {
            #( #dependency_ref_fields , )*
            #( #state_fields , )*
            #( #children_component_fields , )*
        }
    }
}

pub fn instance_ref_name(component_name: &Ident) -> Ident {
    syn::Ident::new(&format!("{}InstanceRef", component_name), component_name.span())
}