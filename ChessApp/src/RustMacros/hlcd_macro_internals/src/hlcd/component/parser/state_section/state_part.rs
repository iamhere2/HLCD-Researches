use syn::{parse::Parse, Field, Token, Expr};

#[derive(Debug)]
pub struct StatePart {
    pub field: Field,
    _equal_token: Token![=],
    pub initial_value: Expr
}

impl Parse for StatePart {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(StatePart {
            field: Field::parse_named(input)?,
            _equal_token: input.parse()?,
            initial_value: input.parse()?,
        })
    }
}