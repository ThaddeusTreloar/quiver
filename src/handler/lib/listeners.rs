// Internal
use crate::{
    shared::{
        lib::{
            Action,
            Action::*,
            Permission,
            PermissionState::*,
            HandlerType,
            from_reader,
            deserialize_pubkey
        },
        error::*
    },
    core::db::{
        db::{
            get_service
        },
        models::ServiceQuery
    }
};

// External
use openssl::{
    pkey::{
        Public,
        PKey,
    },
    sign::{
        Verifier
    },
};
use interprocess::local_socket::{
    LocalSocketListener,
    LocalSocketStream
};
use serde_json::{
    to_writer,
    from_str
};
use std::{
    thread,
    io::{
        prelude::*, 
    }
};
use log::{
    error,
    warn
};
use diesel::{
    r2d2::{
        Pool,
        ConnectionManager
    },
    sqlite::{
        SqliteConnection
    }
};
use failure::Error;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

fn authrorize(
    service: &HandlerType,
    perms: Vec<Permission>,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            let action: Action = from_reader(&mut connection)?;

            match perms.iter().find(
                |s| *s == service
            ) {
                Some(perm) => {
                    match (action, perm.state) {
                        (Get, Read | ReadWrite) => Ok(connection),
                        (Put | Pop | Edit, Write | ReadWrite) => Ok(connection),
                        _ => Err(
                            Error::from(
                                AuthorizationError::AuthorizationFailed
                            )
                        )
                    }
                },
                None => {
                    Err(
                        Error::from(
                            AuthorizationError::AuthorizationFailed
                        )
                    )
                }
            }
        }
    }
}

fn authenticate(
    key: PKey<Public>,
    connection: LocalSocketStream
) -> Result<LocalSocketStream, Error>
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
                AuthenticationError::AuthenticationFailed
            )
        ) 
    }
}

fn authenticate_authorize(
    handler: &HandlerType,
    permission_db: &Pool<ConnectionManager<SqliteConnection>>,
    connection: LocalSocketStream
) -> Result<LocalSocketStream, Error>
{
    let name: String = from_reader(&mut connection)?;
    let query: Vec<ServiceQuery> = get_service(
        name, 
        permission_db
    )?;

    match query.get(0) {
        Some(service_record) => {
            let perms: Vec<Permission> = from_str(service_record.perm.as_ref())?;
            let key = PKey::try_from(
                deserialize_pubkey(service_record.pubkey)?
            )?;

            authrorize(handler, perms, authenticate(key, connection))
        },
        None => return Err(
            Error::from(
                AuthenticationError::ServiceNotRegistered{
                    name: name
                }
            )
        )
    }
}

pub fn af_local_listener(
    listen_address: String, 
    handler: &HandlerType,
    permission_db: &Pool<ConnectionManager<SqliteConnection>>,
    connection_handler: fn(connection: Result<LocalSocketStream, Error>) -> ()
) -> Result<(), Error>
{
    let listener: LocalSocketListener = LocalSocketListener::bind(listen_address)?;
    
    for mut conn in listener.incoming() {
        match conn {
            Ok(connection) => {
                thread::spawn( move ||
                    {
                        connection_handler(
                            authenticate_authorize(
                                handler, 
                                permission_db, 
                                connection
                            )
                        )
                    }
                );
            },
            Err(e) => warn!(format!("Listener connection failed: {}", e))
        }
    }

    Ok(())
}

//transaction(authorize(handlers: String, authenticate(pubkey: Vec<u8>, identify(name: String, accept())))