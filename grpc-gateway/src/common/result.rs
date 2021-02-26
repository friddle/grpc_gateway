use std::result;

use crate::common::error::Error;

pub type Result<T> = result::Result<T, Error>;
