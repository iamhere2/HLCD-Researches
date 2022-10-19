use std::{rc::Rc, cell::RefCell};
use super::{super::data::*, data::command::Command};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error(pub String); 

pub(super) trait CommandParserInterface {
    fn parse(&self, s: &str) -> Result<Command, Error>;
    fn get_help(&self) -> String;
}

pub(super) trait CommandParserProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn CommandParserInterface>>; 
}
