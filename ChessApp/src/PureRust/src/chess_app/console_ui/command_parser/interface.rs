use std::{rc::Rc, cell::RefCell};
use super::{super::data::*, super::data::command::Command};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error(pub String); 

pub trait CommandParserInterface {
    fn parse(&self, s: &str) -> Result<Command, Error>;
    fn get_help(&self) -> String;
}

pub trait CommandParserProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn CommandParserInterface>>; 
}
