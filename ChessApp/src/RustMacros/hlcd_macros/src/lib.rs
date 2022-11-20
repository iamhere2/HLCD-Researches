use proc_macro::{TokenStream, TokenTree, Ident, Span};
// use quote::quote;
// use syn::parse::Nothing;
// use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn interface(input: TokenStream) -> TokenStream {
    let mut out = vec![];
    let mut input = input.clone().into_iter();

    out.push(TokenTree::Ident( Ident::new("pub", Span::mixed_site())));
    out.push(TokenTree::Ident( Ident::new("trait", Span::mixed_site())));

    let base_name = match input.next() {
        Some(TokenTree::Ident(base_name)) => base_name,
        _ => panic!("Interface identifier is expected")         
    };

    eprintln!("base_name: [{base_name}]");

    let interface_trait_name = format!("{}Interface", base_name.to_string());
    out.push(TokenTree::Ident(Ident::new(dbg!(interface_trait_name.as_str()), Span::mixed_site())));

    for t in input {
        out.push(t)
    }

    out.into_iter().collect()
}