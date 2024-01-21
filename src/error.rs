use std::{ffi, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InjectError {
    #[error("Failed to inject")]
    Failed,
    #[error("I/O error: {source:?}")]
    Io {
        #[from]
        source: io::Error,
    },
    #[error("Null error: {source:?}")]
    NullError {
        #[from]
        source: ffi::NulError,
    },
    #[error("unknown data store error")]
    Unknown,
}
