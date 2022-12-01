use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;
use crate::hlcd::component::Component;
use super::component_struct::struct_name;

pub(super) fn gen_provider_trait_impls(component: &Component) -> Vec<TokenStream> {
    let mut trait_impls = vec![];
    for p in component.provides.interfaces.iter() {
        if p.child_component_name.is_some() {
            trait_impls.push(gen_delegated_interface_impl(component, &p.interface_name, p.child_component_name.as_ref().unwrap()) );
        } else {
            trait_impls.push(gen_self_interface_impl(component, &p.interface_name) );
        }
    }
    trait_impls
}

pub fn provider_trait_name(interface_name: &Ident) -> Ident {
    syn::Ident::new(&format!("{}Provider", interface_name), interface_name.span())
}

fn gen_self_interface_impl(component: &Component, interface_name: &Ident) -> TokenStream {
    let provider_trait_name = provider_trait_name(interface_name);
    let interface_ref_name = syn::Ident::new(&format!("{}Ref", interface_name), interface_name.span());
    let struct_name = struct_name(&component.name);

    quote! {
        impl #provider_trait_name for #struct_name {
            fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> #interface_ref_name {
                it
            }
        }
    }
}

fn gen_delegated_interface_impl(
    component: &Component, 
    interface_name: &Ident,
    child_component_name: &Ident) -> TokenStream {

    let struct_name = struct_name(&component.name);
    let interface_ref_name = syn::Ident::new(&format!("{}Ref", interface_name), interface_name.span());
    let provider_trait_name = syn::Ident::new(&format!("{}Provider", interface_name), interface_name.span());

    quote! {
        impl #provider_trait_name for #struct_name {
            fn get(it: ::std::rc::Rc<::std::cell::RefCell<#struct_name>>) -> #interface_ref_name { 
                #provider_trait_name::get(::std::rc::Rc::clone(&::std::cell::RefCell::borrow(&it).#child_component_name)) 
            }
        }
    }
}