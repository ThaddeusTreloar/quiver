use crate::shared::lib::{
    HandlerType,
    PermissionState,
    SERVICE_MANAGER_SOCKET_ADDR,
    Action,
    from_reader
};
use std::io::prelude::*;
use crate::shared::error;
use openssl::{
    ec::EcKey,
    pkey::{
        Public,
        Private,
    },
    sign::Signer
};
use failure::Error;
use interprocess::local_socket::LocalSocketStream;
use serde::Deserialize;
use serde_json::{
    to_writer,
    Deserializer
};


fn permission_transaction(
    service: HandlerType, 
    connection: Result<LocalSocketStream, Error>
) -> Result<PermissionState, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(conn) => {
            to_writer(&mut conn, &service)?;

            let state = from_reader<>
        }
    }
}

fn verify_identity(
    priv_key: &EcKey<Private>,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection {
        Err(e) => return Err(e),
        Ok(conn) => {
            let mut challenge: [u8; 512] = [0u8; 512];
            conn.read_buf_exact(&mut challenge)?;

            let mut signer = Signer::new_without_digest(priv_key)?;

            signer.sign(challenge)?;

            conn.write_all(&challenge)?;

            let mut deser = Deserializer::from_reader(&mut conn);
            let res = bool::deserialize(&mut deser)?;
            
            if res {
                return Ok(conn);
            } else {
                return Err(error::AuthenticationError::AuthenticationFailed);
            }
        }
    };   
}

fn get_connection(address: &'static str, action: Action) -> Result<LocalSocketStream, Error>
{
    let mut connection = LocalSocketStream::connect(SERVICE_MANAGER_SOCKET_ADDR)?;

    to_writer(&mut connection, &Action::Get)?;

    let res: Action = from_reader(&mut connection)?;

    match res 
    {
        Action::Ready => {
            return Ok(connection);
        }
        other => {
            return Err(
                Error::from(
                    error::ConnectionActionError::UnexpectedActionError{
                        expected: "Ready".to_owned(),
                        recieved: other.to_string(),
                        action: action.to_string(),
                        service: "Service Manager".to_owned()
                    }
                )
            );
        }
    }
}

fn permission(service: HandlerType, address: &'static str, action: Action, priv_key: EcKey<Private>)
{
    //check_permission(service, verify_identity(get_connection(address, action), eckey))

    //permission(service, address, action, eckey)
}

