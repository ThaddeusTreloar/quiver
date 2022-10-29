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
use serde_json::to_writer;
use interprocess::local_socket::LocalSocketStream;
use failure::Error;
use openssl::{
    pkey::{
        PKey,
        Private
    },
    sign::Signer,
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
) -> Result<LocalSocketStream, Error>
{
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
            to_writer(&mut connection, service)?;
            to_writer(&mut connection, action)?;
            match from_reader(&mut connection)?
            {
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

            let mut challenge: [u8; 512] = [0u8; 512];
            connection.read(&mut challenge)?;

            let signer = Signer::new_without_digest(priv_key)?;

            let sig_len = signer.sign(&mut challenge)?;

            dbg!(sig_len);
            to_writer(&mut connection, &sig_len)?;
            connection.write_all(&challenge[0..sig_len])?;

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


