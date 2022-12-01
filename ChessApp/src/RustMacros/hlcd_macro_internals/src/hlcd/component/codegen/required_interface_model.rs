use convert_case::{Casing, Case};
use syn::Ident;

use crate::hlcd::component::parser::requires_section::required_interface::{RequiredInterface, NamedInterfacePort, UnnamedInterfacePort};

pub struct RequiredInterfaceModel {
    pub port_name: Ident,
    pub interface_name: Ident,
    pub interface_ref_name: Ident
}

impl From<&RequiredInterface> for RequiredInterfaceModel {
    fn from(r: &RequiredInterface) -> Self {
        let (port_name, interface_name) = match r {
            RequiredInterface::Named(NamedInterfacePort { port_name, interface_name, .. }) 
                => (port_name.clone(), interface_name.clone()),
            RequiredInterface::Unnamed(UnnamedInterfacePort { interface_name }) 
                => (gen_port_name(interface_name), interface_name.clone())
        };
        let interface_ref_name = syn::Ident::new(&format!("{}Ref", interface_name), interface_name.span());
        Self { port_name, interface_name, interface_ref_name }
    }
}

pub fn gen_port_name(interface_name: &Ident) -> Ident {
    syn::Ident::new(&interface_name.to_string().to_case(Case::Snake) , interface_name.span())
}