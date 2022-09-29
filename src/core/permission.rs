use std::{
    io::{
        prelude::*, 
    }
};
use openssl::{
    pkey::{
        Public,
        PKey,
        PKeyRef
    },
    encrypt::Encrypter,
};
use interprocess::local_socket::LocalSocketStream;
use rand::prelude::*;
use rand_chacha::ChaChaRng;


fn identify(key: PKey<Public>, mut connection: LocalSocketStream) -> Result<bool, &'static str>
{
    // todo: check this has enough entropy
    let mut rng_ctx = match ChaChaRng::from_rng(rand::thread_rng())
    {
        Ok(ccr) => ccr,
        Err(e) => return Err("Failed to initialise RNG due to: {e}")
    };

    let mut rand_bytes: [u8; 512] = [0u8; 512];
    rng_ctx.fill(&mut rand_bytes);
    let enc = match Encrypter::new(key.as_ref())
    {
        Ok(val) => val,
        Err(e) => return Err("Failed to created encrypter from publickey due to: {e}")
    };
    let buffer_len = match enc.encrypt_len(&rand_bytes)
    {
        Ok(val) => val,
        Err(e) => return Err("Failed to get required buffer length due to: {e}")
    };
    let mut challenge = vec![0u8; buffer_len];

    match enc.encrypt(&mut rand_bytes, &mut challenge)
    {
        Ok(_v) => (),
        Err(e) => return Err("Failed to encrypt bytes for challenge due to: {e}")
    };

    match connection.write_all(&challenge)
    {
        Ok(_v)  => (),
        Err(e) => return Err("Failed to write challenge buffer due to: {e}")
    }

    let mut response: Vec<u8> = vec![0u8; 512];

    match connection.read_exact(&mut response)
    {
        Ok(_v) => {
            let resp: Vec<u8> = Vec::from(response);
            if resp == challenge {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        Err(e) => return Err("Failed to read from connection due to: {e}")
    }
}