// Internal
use crate::shared::lib::HandlerType;

// External
use openssl::{
    pkey::{
        PKey,
        Private  
    },
    sign::{
        Signer,
        Verifier
    }
};
use log::{
    warn
};
use interprocess::local_socket::{
    LocalSocketListener
};
use failure::Error;

