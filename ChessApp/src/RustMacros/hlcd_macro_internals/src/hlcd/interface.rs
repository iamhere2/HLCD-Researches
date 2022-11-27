mod parser;
mod codegen;

use syn::{Ident, TraitItemMethod};

pub use parser::kw;

#[derive(Debug)]
pub struct Interface {
    pub name: Ident,
    pub items: Vec<TraitItemMethod>
}
