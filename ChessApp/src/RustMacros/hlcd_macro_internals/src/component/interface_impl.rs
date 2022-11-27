use syn::{Ident, ImplItem};

#[derive(Debug)]
pub struct InterfaceImplementation { 
    pub interface_name: Ident,
    pub items: Vec<ImplItem>
}