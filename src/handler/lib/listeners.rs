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
            AuthorizedConnection
        },
        error::*
    },
    core::db::{
        db::{
            get_service,
            search_service
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
    hash::MessageDigest,
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
    info,
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
                    match (action, &perm.state) {
                        (Get, Read | ReadWrite) => {
                            to_writer(&mut connection, &true)?;
                            Ok(connection)
                        },
                        (Put | Pop | Edit, Write | ReadWrite) => {
                            to_writer(&mut connection, &true)?;
                            Ok(connection)
                        },
                        _ => {
                            to_writer(&mut connection, &false)?;
                            Err(
                                Error::from(
                                    AuthorizationError::AuthorizationFailed
                                )
                            )
                        }
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
    mut connection: LocalSocketStream
) -> Result<LocalSocketStream, Error>
{
    // todo: check this has enough entropy
    let mut rng_ctx = ChaChaRng::from_rng(rand::thread_rng())?;

    let mut rand_bytes: [u8; 512] = [0u8; 512];
    rng_ctx.fill(&mut rand_bytes);
    connection.write_all(&rand_bytes)?;
    
    let mut verifier: Verifier = Verifier::new(MessageDigest::sha256(), &key)?;
    let mut response: Vec<u8> = vec![0u8; 256];
    let sig_len: usize = connection.read(&mut response)?;
    verifier.update(&rand_bytes)?;
    let verification: bool = verifier.verify(&response[0..sig_len])?;
    to_writer(&mut connection, &verification)?;
    if verification { Ok(connection) } else { Err(
        Error::from(
                AuthenticationError::AuthenticationFailed
            )
        ) 
    }
}

pub fn authenticate_authorize(
    handler: &HandlerType,
    permission_db: &Pool<ConnectionManager<SqliteConnection>>,
    mut connection: LocalSocketStream
) -> Result<LocalSocketStream, Error>
{
    let name: String = from_reader(&mut connection)?;
    let query: Vec<ServiceQuery> = search_service(
        &name, 
        permission_db
    )?;

    //dbg!(&query);

    match query.get(0) {
        Some(service_record) => {
            let perms: Vec<Permission> = from_str(service_record.perm.as_ref())?;
            let key = PKey::public_key_from_der(&service_record.pubkey)?;
            
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
    handler: HandlerType,
    permission_db: Pool<ConnectionManager<SqliteConnection>>,
    handler_db: Pool<ConnectionManager<SqliteConnection>>,
    connection_handler: fn(Result<LocalSocketStream, Error>, &Pool<ConnectionManager<SqliteConnection>>) -> Result<String, Error>
) -> Result<(), Error>
{
    let listener: LocalSocketListener = LocalSocketListener::bind(listen_address)?;
    
    for conn in listener.incoming() {
        match conn {
            Err(e) => warn!("{}", format!("Listener connection failed: {}", e)),
            Ok(connection) => {
                let pdb = permission_db.clone();
                let hdb = handler_db.clone();
                thread::spawn( move ||
                    {
                        let peer_pid: String = match connection.peer_pid() 
                        {
                            Ok(peer_id) => 
                            {
                                info!("Client connnected, pid<{peer_id}>.");
                                peer_id.to_string()
                            },
                            Err(_e) => 
                            {
                                info!("Client connection, no pid available.");
                                "Unavailable".to_owned()
                            }
                        };
                        match connection_handler(
                            authenticate_authorize(
                                &handler, 
                                &pdb, 
                                connection
                            ),
                            &hdb
                        ) {
                            Ok(log) => info!("{}", log),
                            Err(e) => warn!("{} for pid: {}", e.name().unwrap(), peer_pid)
                        }
                    }
                );
            },
        }
    }

    Ok(())
}

//transaction(authorize(handlers: String, authenticate(pubkey: Vec<u8>, identify(name: String, accept())))