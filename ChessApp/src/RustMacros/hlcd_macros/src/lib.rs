#![feature(proc_macro_quote)]
#![feature(proc_macro_diagnostic)]
#![feature(extend_one)]

mod interface_definition;
use interface_definition::*;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;
//use proc_macro2::Span;
// use quote::quote;
// use syn::parse::Nothing;
// use syn::{parse_macro_input, DeriveInput};


#[allow(unused)]
#[proc_macro]
pub fn interface(input: TokenStream) -> TokenStream {
    let interface_definition = parse_macro_input!(input as InterfaceDefinition);
    
    let InterfaceDefinition {
        base_interface_name,
        items
    } = interface_definition;
    
    // eprintln!("base_name: [{:?}]", base_interface_name);
    // eprintln!("items: [{:?}]", items);

    // Diagnostic::new(Level::Warning, "This method is deprecated").emit();    

    let interface_trait_name = syn::Ident::new(&format!("{}Interface", base_interface_name), base_interface_name.span());
    let interface_trait = quote! {
        pub trait #interface_trait_name {
            #( #items )*
        }
    };

    let interface_ref_name = syn::Ident::new(&format!("{}Ref", base_interface_name), base_interface_name.span());
    let interface_ref_type = quote! {
        type #interface_ref_name = std::rc::Rc<std::cell::RefCell<dyn #interface_trait_name>>;
    };

    let provider_trait_name = syn::Ident::new(&format!("{}Provider", base_interface_name), base_interface_name.span());
    let provider_trait = quote! {
        pub trait #provider_trait_name {
            fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> #interface_ref_name; 
        }
    };

    let interface = quote! {
        #interface_trait
        #interface_ref_type
        #provider_trait
    };

    TokenStream::from(interface)

    // let syn::parse(input).unwrap();

    // let mut out = vec![];
    // let mut input = input.clone().into_iter();

    // out.push(TokenTree::Ident( Ident::new("pub", Span::mixed_site())));
    // out.push(TokenTree::Ident( Ident::new("trait", Span::mixed_site())));

    // let base_name = match input.next() {
    //     Some(TokenTree::Ident(base_name)) => base_name,
    //     _ => panic!("Interface identifier is expected")         
    // };


    // let interface_trait_name = syn::Ident::new(format!("{}Interface", base_name).as_str(), Span::mixed_site());
    // out.push(TokenTree::Ident(interface_trait_name));

    // for t in input {
    //     out.push(t)
    // }

    // let provider_trait_name = format!("{}Provider", base_name.to_string());
    // let provider_definition = quote! {
    //     pub trait #provider_trait_name {
    //         fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn #interface_trait_name>>; 
    //     }
    // };

    // let out = [out, provider_definition.into_iter().collect() ].concat();

    // out.into_iter().collect()

}