mod parser;
mod codegen;

use syn::{Ident, ImplItem};

pub use parser::kw;

use parser::{requires_section::RequiresSection, child_component::ChildComponent, interface_impl::InterfaceImplementation, provided_interface::ProvidedInterface};

use self::parser::state_section::StateSection;

#[derive(Debug)]
pub struct Component {
    pub name: Ident,
    pub requires: RequiresSection,
    pub provides: Vec<ProvidedInterface>,
    pub children: Vec<ChildComponent>,
    pub state: StateSection,
    pub interface_impls: Vec<InterfaceImplementation>,
    pub private_impl_items: Vec<ImplItem>,
}


