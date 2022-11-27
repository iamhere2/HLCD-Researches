use proc_macro2::TokenStream;
use quote::quote;

use crate::hlcd::component::{Component, parser::state_section::state_part::StatePart};

pub(super) fn gen_constructor(component: &Component) -> TokenStream {

    let struct_name = super::component_struct::struct_name(component);

    let dependency_ref_name_type_list = component.requires.interfaces.iter().map(|r| {
        let ref_name = &r.ref_name;
        let interface_ref_name = syn::Ident::new(&format!("{}Ref", r.interface_name), r.interface_name.span());
        quote! { 
            #ref_name : & #interface_ref_name
        }
    }).collect::<Vec<_>>();

    let dependency_ref_assignments = component.requires.interfaces.iter().map(|r| {
        let ref_name = &r.ref_name;
        quote! { 
            #ref_name : ::std::rc::Rc::clone(#ref_name) 
        }
    }).collect::<Vec<_>>();

    let state_initial_assignments = component.state.parts.iter().map(|StatePart { field, initial_value, .. }| {
        let field = &field.ident;
        quote!{ 
            #field : #initial_value
        }
    }).collect::<Vec<_>>();

    quote! {
        pub fn new( 
            #( #dependency_ref_name_type_list , )*
        ) -> Self {
            #struct_name { 
                #( #dependency_ref_assignments , )*
                #( #state_initial_assignments , )*
            }
        }
    }
}