//Jerome M. St.Martin
//May, 2022

use std::fmt;

//-------------------------------------------
//------------ Custom Err Type ------------
//-------------- & Err Codes --------------
//-------------------------------------------

#[derive(Debug)]
pub enum Gremlin {
    //add variants as needed
    //Internal Errs
    InvalidInput,
    UnknownGameError,

    //Outside Errs w/ Source Fields
    IOErr(std::io::Error),
}

impl fmt::Display for Gremlin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> std::error::Error for Gremlin {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Gremlin::IOErr(source) => Some(source),
            _ => None,
        }
    }
}

impl<'a> From<std::io::Error> for Gremlin {
    fn from(item: std::io::Error) -> Self {
        Gremlin::IOErr(item)
    }
}
