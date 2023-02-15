extern crate failure;
extern crate failure_derive;

use failure::{Error, Fail};

#[derive(Debug, Fail)]
pub enum KvsError {
    #[fail(display = "key not found")]
    KeyNoFound,
}
