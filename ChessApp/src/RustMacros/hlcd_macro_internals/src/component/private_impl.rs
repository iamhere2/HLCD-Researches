use syn::ImplItem;

#[derive(Debug)]
pub struct PrivateImplementation { 
    pub items: Vec<ImplItem>
}