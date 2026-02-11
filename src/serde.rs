use std::{fs::File, io, path::PathBuf, result};

use serde_json::error::Category;

use crate::BusinessCalendar;

pub enum Error {
    FileNotFound,
    Data,
    Io,
    Syntax,
    Eof,
    Serde,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => Self::FileNotFound,
            _ => Self::Io,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        match value.classify() {
            Category::Data => Self::Data,
            Category::Eof => Self::Eof,
            Category::Io => Self::Io,
            Category::Syntax => Self::Syntax,
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

// serde-related methods for BusinessCalendar
impl BusinessCalendar {
    pub fn from_json(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        serde_json::from_reader(file).map_err(Error::from)
    }

    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text).map_err(Error::from)
    }

    pub fn to_json_str(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(Error::from)
    }
}
