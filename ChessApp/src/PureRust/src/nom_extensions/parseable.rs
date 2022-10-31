use std::{error::Error, fmt::Display};

use nom::Finish;

type NomStrError = nom::error::Error<String>;

pub struct ParseError {
    source: NomStrError,
    message: &'static str
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.message == other.message
    }
}

impl ParseError {
    fn new(source: NomStrError, message: &'static str) -> Self {
        ParseError { source, message }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.message)
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, nom error: {:?}", self, self.source)
    }
}

pub trait Parseable {
    const MESSAGE: &'static str;

    fn nom_parse(input: &str) -> nom::IResult<&str, Self>
        where Self: Sized;
    
    fn parse<'a>(input: &str) -> Result<Self, ParseError>
        where Self: Sized 
    {
        Parseable::nom_parse(input).finish()
            .map(|(_, v)| v)
            .map_err(|e| ParseError::new(
                NomStrError::new(e.input.to_string(), e.code), 
                Self::MESSAGE))
    }
}

