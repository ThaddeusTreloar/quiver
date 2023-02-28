// Internal
use crate::{
    shared::{
        lib::{
            Action,
            Action::*,
            Permission,
            PermissionState::*,
            HandlerType,
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
    from_str
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
use tokio::net::UnixStream;
use diesel::{
    r2d2::{
        Pool,
        ConnectionManager
    },
    sqlite::{
        SqliteConnection
    }
};

async fn connect(address: &'static str) -> Result<UnixStream, Error>
{
    Ok(UnixStream::connect(address).await?)
}

async fn challenge_client(
    key: &PKey<Public>,
    stream: Result<UnixStream, Error>
) -> Result<UnixStream, Error>
{
    match stream{
        Ok(stream) => {
            // todo: check this has enough entropy
            let mut rng_ctx = ChaChaRng::from_rng(rand::thread_rng())?;

            let mut rand_bytes: [u8; 512] = [0u8; 512];

            rng_ctx.fill(&mut rand_bytes);

            stream.writable().await?;
            stream.try_write(&rand_bytes)?;
            
            let mut verifier: Verifier = Verifier::new(MessageDigest::sha256(), key)?;
            let mut response: Vec<u8> = vec![0u8; 256];
            stream.readable().await?;
            let sig_len: usize = stream.try_read(&mut response)?;
            verifier.update(&rand_bytes)?;
            let verification: bool = verifier.verify(&response[0..sig_len])?;
            stream.writable().await?;
            stream.try_write(&serde_json::to_vec(&verification)?)?;

            if verification { Ok(stream) } else { Err(
                Error::from(
                        AuthenticationError::AuthenticationFailed
                    )
                ) 
            }
        },
        Err(e) => Err(e)
    }
   
}

async fn solve_challenge(
    key: &PKey<Private>,
    stream: Result<UnixStream, Error>
) -> Result<UnixStream, Error>
{
    match stream{
        Ok(stream) => {
            // todo: check safety
            stream.readable().await?;
            let mut challenge: [u8; 512] = [0u8; 512];
            stream.try_read(&mut challenge)?;
            let mut signer = Signer::new(MessageDigest::sha256(), key)?;
            signer.update(&challenge)?;
            let sig = signer.sign_to_vec()?;
            stream.writable().await?;
            stream.try_write(&sig)?;

            stream.readable().await?;
            let mut response: Vec<u8> = Vec::new();
            stream.try_read(&mut response)?;
            let res: bool = serde_json::de::from_slice(&response)?;
            
            if res { Ok(stream) } else {
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

async fn authorize(
    service: &HandlerType,
    action: &Action,
    stream: Result<UnixStream, Error>
) -> Result<UnixStream, Error>
{
    match stream {
        Err(e) => Err(e),
        Ok(stream) => {
            stream.writable().await?;
            let service = serde_json::ser::to_vec(&service)?;
            stream.try_write(&service)?;
            
            let act = serde_json::ser::to_vec(&action)?;
            stream.try_write(&act)?;
            
            stream.readable().await?;
            let mut buff = Vec::new();
            stream.try_read(&mut buff)?;
            let res = serde_json::de::from_slice(&buff)?;

            match res {
                true => Ok(stream),
                false => Err(
                    Error::from(
                        AuthorizationError::AuthorizationFailed
                    )
                )
            }
        }
    }
}

async fn authorize_client(
    perms: Vec<Permission>,
    stream: Result<UnixStream, Error>
) -> Result<(HandlerType, Action, UnixStream), Error>
{
    match stream {
        Err(e) => Err(e),
        Ok(stream) => {

            stream.readable().await?;
            let mut buff = Vec::new();

            stream.try_read(&mut buff)?;

            let service: HandlerType = serde_json::de::from_slice(&buff)?;

            buff = Vec::new();
            stream.try_read(&mut buff)?;
            let action: Action = serde_json::de::from_slice(&buff)?;

            match perms.iter().find(
                |s| *s == &service
            ) {
                Some(perm) => {
                    match (&action, &perm.state) {
                        (Get, Read | ReadWrite) => {
                            stream.writable().await?;
                            stream.try_write(&serde_json::to_vec(&true)?)?;
                            Ok((service, action, stream))
                        },
                        (Put | Pop | Edit, Write | ReadWrite) => {
                            stream.writable().await?;
                            stream.try_write(&serde_json::to_vec(&true)?)?;
                            Ok((service, action, stream))
                        },
                        _ => {
                            stream.writable().await?;
                            stream.try_write(&serde_json::to_vec(&false)?)?;
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

async fn interrogate_server(
    stream: Result<UnixStream, Error>
) -> Result<(HandlerType, Action, UnixStream), Error>
{
    match stream {
        Err(e) => Err(e),
        Ok(stream) => {
            stream.readable().await?;
            let mut buff = Vec::new();
            stream.try_read(&mut buff)?;
            let service: HandlerType = serde_json::de::from_slice(&buff)?;
            buff = Vec::new();
            stream.try_read(&mut buff)?;
            let action: Action = serde_json::de::from_slice(&buff)?;
            // @JMP001
            Ok((service,
                action,
                stream))
        }
    }
}

async fn authenticate(
    name: &String,
    key: &PKey<Private>,
    stream: Result<UnixStream, Error>
) -> Result<UnixStream, Error>
{
    match stream {
        Err(e) => Err(e),
        Ok(stream) => {
            stream.writable().await?;
            stream.try_write(&serde_json::ser::to_vec(name)?)?;
            solve_challenge(key, Ok(stream)).await
        }
    }
}

async fn start_client_interrogation(
    service: &HandlerType,
    action: &Action,
    stream: Result<UnixStream, Error>
) -> Result<UnixStream, Error>
{
    match stream {
        Err(e) => Err(e),
        Ok(stream) => {
            stream.writable().await?;
            let service = serde_json::ser::to_vec(&service)?;
            stream.try_write(&service)?;
            
            let act = serde_json::ser::to_vec(&action)?;
            stream.try_write(&act)?;
            
            stream.readable().await?;
            let mut buff = Vec::new();
            stream.try_read(&mut buff)?;
            let res = serde_json::de::from_slice(&buff)?;

            if res { Ok(stream) } else {Err(
                Error::from(
                    InitiationError::ServiceNotSupported
                )
            )}
        }
    }
}

pub async fn authorize_server_connection(
    key: &PKey<Private>,
    server_key: &PKey<Public>,
    connection: UnixStream
) -> Result<(HandlerType, Action, UnixStream), Error>
{ 
    interrogate_server(
        solve_challenge(key, 
            challenge_client(server_key, Ok(connection)).await).await).await
}

pub async fn authorize_client_connection(
    key: &PKey<Private>,
    permission_db: &Pool<ConnectionManager<SqliteConnection>>,
    stream: UnixStream
) -> Result<(HandlerType, Action, UnixStream), Error>
{
    stream.readable().await?;
    let mut buff: Vec<u8> = vec![0u8;64];
    stream.try_read(&mut buff)?;
    let name: String = serde_json::de::from_slice(&buff)?;
    let query: Vec<ServiceQuery> = search_service(
        &name, 
        permission_db
    )?;

    match query.get(0) {
        Some(service_record) => {
            let perms: Vec<Permission> = from_str(service_record.perm.as_ref())?;
            let client_key = PKey::public_key_from_pem(&service_record.pubkey)?;
            
            authorize_client(perms, 
                solve_challenge(key, 
                    challenge_client(&client_key, Ok(stream)).await).await).await
        },
        None => Err(
            Error::from(
                AuthenticationError::ServiceNotRegistered{
                    name
                }
            )
        )
    }
}

pub async fn connect_to_client(
    address: &'static str, 
    key: &PKey<Private>,
    client_key: &PKey<Public>,
    action: &Action,
    service: &HandlerType
) -> Result<UnixStream, Error> {
    start_client_interrogation(service, action, 
        challenge_client(client_key, 
            solve_challenge(key, 
                    connect(address).await).await).await).await
}

pub async fn connect_to_server(
    address: &'static str, 
    key: &PKey<Private>,
    server_key: &PKey<Public>,
    name: &String,
    service: &HandlerType,
    action: &Action,
) -> Result<UnixStream, Error> {
    authorize(service, action,
        challenge_client(server_key,
            authenticate(name, key,
                connect(address).await).await).await).await
}
