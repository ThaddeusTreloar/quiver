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
        },
        errors::*
    },
    db::{
        search_service,
        models::ServiceQuery
    },
};

// External
use failure::Error;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use serde_json::{
    to_writer,
    from_str
};
use std::io::prelude::{
    Read,
    Write
};
use openssl::{
    pkey::{
        Private,
        Public,
        PKey,
    },
    sign::{
        Verifier,
        Signer
    },
    hash::MessageDigest,
};
use interprocess::local_socket::{
    LocalSocketStream
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

fn connect(address: &'static str) -> Result<LocalSocketStream, Error>
{
    Ok(LocalSocketStream::connect(address)?)
}

fn challenge_client(
    key: &PKey<Public>,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection{
        Ok(mut connection) => {
            // todo: check this has enough entropy
            let mut rng_ctx = ChaChaRng::from_rng(rand::thread_rng())?;

            let mut rand_bytes: [u8; 512] = [0u8; 512];

            rng_ctx.fill(&mut rand_bytes);

            connection.write_all(&rand_bytes)?;
            
            let mut verifier: Verifier = Verifier::new(MessageDigest::sha256(), key)?;
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
        },
        Err(e) => Err(e)
    }
   
}

fn solve_challenge(
    key: &PKey<Private>,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection{
        Ok(mut connection) => {
        // todo: check safety
        let mut challenge: [u8; 512] = [0u8; 512];
        connection.read(&mut challenge)?;
        let mut signer = Signer::new(MessageDigest::sha256(), key)?;
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
        },
        Err(e) => Err(e)
    }
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

fn authorize_client(
    perms: Vec<Permission>,
    connection: Result<LocalSocketStream, Error>
) -> Result<(HandlerType, Action, LocalSocketStream), Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            let service: HandlerType = from_reader(&mut connection)?;
            let action: Action = from_reader(&mut connection)?;

            match perms.iter().find(
                |s| *s == &service
            ) {
                Some(perm) => {
                    match (&action, &perm.state) {
                        (Get, Read | ReadWrite) => {
                            to_writer(&mut connection, &true)?;
                            Ok((service, action, connection))
                        },
                        (Put | Pop | Edit, Write | ReadWrite) => {
                            to_writer(&mut connection, &true)?;
                            Ok((service, action, connection))
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

fn interrogate_server(
    connection: Result<LocalSocketStream, Error>
) -> Result<(HandlerType, Action, LocalSocketStream), Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            let service: HandlerType = from_reader(&mut connection)?;
            let action: Action = from_reader(&mut connection)?;
            // @JMP001
            Ok((service,
                action,
                connection))
        }
    }
}

fn authenticate(
    name: &String,
    key: &PKey<Private>,
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            to_writer(&mut connection, name)?;
            solve_challenge(key, Ok(connection))
        }
    }
}

fn start_client_interrogation(
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

            let res = from_reader(&mut connection)?;

            if res { Ok(connection) } else {Err(
                Error::from(
                    InitiationError::ServiceNotSupported
                )
            )}
        }
    }
}

pub fn authorize_server_connection(
    key: &PKey<Private>,
    server_key: &PKey<Public>,
    connection: LocalSocketStream
) -> Result<(HandlerType, Action, LocalSocketStream), Error>
{ 
    interrogate_server(
        solve_challenge(key, 
            challenge_client(server_key, Ok(connection))))
}

pub fn authorize_client_connection(
    key: &PKey<Private>,
    permission_db: &Pool<ConnectionManager<SqliteConnection>>,
    mut connection: LocalSocketStream
) -> Result<(HandlerType, Action, LocalSocketStream), Error>
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
            let client_key = PKey::public_key_from_pem(&service_record.pubkey)?;
            
            authorize_client(perms, 
                solve_challenge(key, 
                    challenge_client(&client_key, Ok(connection))))
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

pub fn connect_to_client(
    address: &'static str, 
    key: &PKey<Private>,
    client_key: &PKey<Public>,
    action: &Action,
    service: &HandlerType
) -> Result<LocalSocketStream, Error> {
    Ok(
        start_client_interrogation(service, action, 
            challenge_client(client_key, 
                solve_challenge(&key, 
                    connect(address))))?
    )
}

pub fn connect_to_server(
    address: &'static str, 
    key: &PKey<Private>,
    server_key: &PKey<Public>,
    name: &String,
    service: &HandlerType,
    action: &Action,
) -> Result<LocalSocketStream, Error> {
    Ok(
        authorize(service, action,
            challenge_client(server_key,
                authenticate(name, key,
                    connect(address))))?
    )
}
