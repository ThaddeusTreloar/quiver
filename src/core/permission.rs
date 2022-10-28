// Internal
use crate::shared::error::AuthenticationError;

// External
use std::{
    io::{
        prelude::*, 
    }
};
use openssl::{
    pkey::{
        Public,
        PKey,
    },
    sign::{
        Verifier
    },
};
use interprocess::local_socket::LocalSocketStream;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use serde_json::to_writer;

fn identify(key: PKey<Public>, mut connection: LocalSocketStream) -> Result<LocalSocketStream, std::io::Error>
{
    // todo: check this has enough entropy
    let mut rng_ctx = ChaChaRng::from_rng(rand::thread_rng())?;

    let mut rand_bytes: [u8; 512] = [0u8; 512];
    rng_ctx.fill(&mut rand_bytes);

    connection.write_all(&rand_bytes)?;

    let mut verifier: Verifier = Verifier::new_without_digest(&key)?;
    verifier.update(&rand_bytes)?;
    let mut response: Vec<u8> = vec![0u8];

    // Todo: Check this isn't exploitable.
    connection.read(&mut response)?;

    let verification: bool = verifier.verify(&response)?;

    to_writer(&mut connection, &verification)?;
    
    if verification { Ok(connection) } else { Err(
        Error::from(
            AuthenticationError::ClientFailedAuthentication{
                
            }
        )
    ) }
    
}