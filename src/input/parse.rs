
extern crate url;

use std::error::Error;
use std::num::ParseIntError;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;
use self::url::{Url, ParseError};
use input::query::Query;

#[derive(Debug)]
pub enum InputError {
    Io(io::Error),
    Url(ParseError),
    NumNotValid(ParseIntError),
    LineNotValid(String),
    NotFound,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::Io(ref err) => err.fmt(f),
            InputError::Url(ref err) => err.fmt(f),
            InputError::NumNotValid(ref err) => err.fmt(f),
            InputError::LineNotValid(ref line) => write!(f, "{}", line),
            InputError::NotFound => write!(f, "No file was given"),
        }
    }
}

impl Error for InputError {
    fn description(&self) -> &str {
        match *self {
            InputError::Io(ref err) => err.description(),
            InputError::Url(ref err) => err.description(),
            InputError::NumNotValid(ref err) => err.description(),
            InputError::LineNotValid(ref line) => line,
            InputError::NotFound => "not found",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InputError::Io(ref err) => Some(err),
            InputError::Url(ref err) => Some(err),
            InputError::NumNotValid(ref err) => Some(err),
            // The custom error doesn't have an underlying cause,
            // but we could modify it so that it does.
            InputError::LineNotValid(ref line) => None,
            InputError::NotFound => None,
        }
    }
}

impl From<io::Error> for InputError {
    fn from(err: io::Error) -> InputError {
        InputError::Io(err)
    }
}

impl From<ParseError> for InputError {
    fn from(err: ParseError) -> InputError {
        InputError::Url(err)
    }
}

impl From<ParseIntError> for InputError {
    fn from(err: ParseIntError) -> InputError {
        InputError::NumNotValid(err)
    }
}

pub fn read_file<P: AsRef<Path>>(file_path: &Option<P>, num_times: u32)
    -> Result<Vec<Query>, InputError> {

    let mut input: Box<io::Read> = match *file_path {
        Option::Some(ref path) => Box::new(try!(File::open(path))),
        Option::None => return Err(InputError::NotFound),
    };

    let mut buffer = String::new();
    try!(input.read_to_string(&mut buffer));

    let mut result: Vec<Query> = vec![];
    for line in buffer.lines() {
        if line.contains(" ") {

            let v: Vec<&str> = line.split(" ").collect();
            if v.len() < 2 || v.len() > 3 {
                InputError::LineNotValid(line.to_string());
            }

            let url = try!(Url::parse(v[1]));
            let times = match v.get(2) {
                Some(times) => try!(times.parse::<u32>()),
                None => num_times
            };

            result.push(Query::new(v[0].to_string(), url, times));

        } else {
            InputError::LineNotValid(line.to_string());
        }
    }

    if result.is_empty() {
        Err(InputError::NotFound)
    } else {
        Ok(result)
    }
}