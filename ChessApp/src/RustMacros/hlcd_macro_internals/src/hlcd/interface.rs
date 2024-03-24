mod parser;
mod codegen;

use syn::{Ident, TraitItemFn};

pub use parser::kw;

#[derive(Debug)]
pub struct Interface {
    pub name: Ident,
    pub items: Vec<TraitItemFn>
}
