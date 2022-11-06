use std::{rc::Rc, cell::RefCell, fmt::Display};
use super::{super::data::*, super::data::command::Command};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error(pub String); 

impl std::error::Error for Error{
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait CommandParserInterface {
    fn parse(&self, s: &str) -> Result<Command, Error>;
    fn get_help(&self) -> String;
}

pub trait CommandParserProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn CommandParserInterface>>; 
}
