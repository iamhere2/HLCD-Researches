use std::fmt::Display;
use super::super::data::command::Command;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error(pub String); 

impl std::error::Error for Error{
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

hlcd::define! {
    interface CommandParser {
        fn parse(&self, s: &str) -> Result<Command, Error>;
        fn get_help(&self) -> String;
    }
}