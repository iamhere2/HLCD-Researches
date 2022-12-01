mod parser;
mod codegen;

use syn::{Ident, ImplItem};
pub use parser::kw;
use parser::{requires_section::RequiresSection, interface_impl::InterfaceImplementation};
use self::parser::{state_section::StateSection, provides_section::ProvidesSection, children_section::ChildrenSection};

#[derive(Debug)]
pub struct Component {
    pub name: Ident,
    pub requires: RequiresSection,
    pub provides: ProvidesSection,
    pub children: ChildrenSection,
    pub state: StateSection,
    pub interface_impls: Vec<InterfaceImplementation>,
    pub private_impl_items: Vec<ImplItem>,
}


