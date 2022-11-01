// Internal
use crate::shared::{
    error::{
        AuthenticationError,
        AuthorizationError
    },
    lib::{
        Action,
        HandlerType,
        from_reader
    }
};
// External
use serde_json::{
    to_writer,
    to_string
};
use interprocess::local_socket::LocalSocketStream;
use failure::Error;
use openssl::{
    pkey::{
        PKey,
        Private
    },
    sign::Signer,
    hash::MessageDigest,
};
use std::io::prelude::{
    Read,
    Write
};

pub fn connect_authenticate_authorize(
    address: &'static str, 
    priv_key: &PKey<Private>, 
    name: &String,
    action: &Action,
    service: &HandlerType
) -> Result<LocalSocketStream, Error> {
    Ok(
        authorize(service, action, 
            identify(name, priv_key,
                connect(address)))?
    )
}

fn authorize(
    service: &HandlerType,
    action: &Action,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            to_writer(&mut connection, action)?;
            match from_reader(&mut connection)? {
                true => Ok(connection),
                false => Err(
                    Error::from(
                        AuthorizationError::AuthorizationFailed
                    )
                )
            }
        }
    }
}

fn identify(
    name: &String,
    priv_key: &PKey<Private>,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            to_writer(&mut connection, name)?;
            // todo: check safety
            let mut challenge: [u8; 512] = [0u8; 512];
            connection.read(&mut challenge)?;
            let mut signer = Signer::new(MessageDigest::sha256(), &priv_key)?;
            signer.update(&challenge)?;
            let sig = signer.sign_to_vec()?;
            connection.write_all(&sig)?;

            let res: bool = from_reader(&mut connection)?;
            
            if res { Ok(connection) } else {
                Err(
                    Error::from(
                        AuthenticationError::AuthenticationFailed
                    )
                )
            }
        }
    }
}

fn connect(address: &'static str) -> Result<LocalSocketStream, Error>
{
    Ok(LocalSocketStream::connect(address)?)
}


