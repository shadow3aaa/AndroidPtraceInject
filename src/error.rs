use std::{io, ffi::NulError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InjectError {
    #[error("Failed to inject")]
    Failed(#[from] io::Error),
    Io {
        #[from]
        source: io::Error,
    },
    NullError {
        #[from]
        source: NulError,
    },
    #[error("unknown data store error")]
    Unknown,
}