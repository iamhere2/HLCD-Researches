use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;

use crate::hlcd::component::parser::state_section::state_part::StatePart;
use super::super::Component;

pub(super) fn struct_name(component: &Component) -> Ident {
    syn::Ident::new(&format!("{}", component.name), component.name.span())
} 

pub(super) fn gen_struct(component: &Component) -> TokenStream {

    let struct_name = struct_name(component);
    
    let state_fields = component.state.parts.iter().map(|StatePart{ field, .. }| {
        quote! { 
            #field 
        }
    }).collect::<Vec<_>>();

    let dependency_ref_fields = component.requires.interfaces.iter().map(|r| {
        let ref_name = &r.ref_name;
        let interface_ref_name = syn::Ident::new(&format!("{}Ref", r.interface_name), r.interface_name.span());

        quote! {
            #ref_name : #interface_ref_name
        }

    }).collect::<Vec<_>>();

    quote! {
        pub struct #struct_name {
            #( #dependency_ref_fields , )*
            #( #state_fields , )*
        }
    }
}