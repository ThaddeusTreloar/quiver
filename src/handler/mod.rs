pub mod calendar;

// Internal
use crate::{
    shared::{
        errors::*,
        lib::{
        HandlerType,
        Action
        },
        
    },
    connection::{
        authorize_server_connection,
        authorize_client_connection
    }
};

//External
use openssl::{
    pkey::{
        PKey,
        Private,
        Public
    },
};
use log::{
    info,
    warn
};
use interprocess::local_socket::{
    LocalSocketListener,
    LocalSocketStream
};
use failure::{
    Error,
    Fail
};
use serde_json::to_writer;
use std::{
    collections::HashMap,
    thread
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

pub struct HandlerGroup {
    handlers: HashMap<HandlerType, fn(LocalSocketStream, Action) -> Result<String, Error>>
}

impl HandlerGroup
{
    fn add_handler(
        &mut self, h: HandlerType, 
        f: fn(LocalSocketStream, Action) -> Result<String, Error>
    )
    {
        self.handlers.insert(h, f);
    }

    fn handle_connection(
        &self,
        connection: Result<(HandlerType, Action, LocalSocketStream), Error>
    ) -> Result<String, Error>
    {
        match connection {
            Err(e) => Err(e),
            Ok((service, action, mut connection)) => {
                match self.handlers.get(&service) {
                    Some(func) => {
                        // Slight coupling. Can't put @JMP001 as there is no way
                        // to check the connection has access to a handler from
                        // within the auth process. May need redesign.
                        to_writer(&mut connection, &true)?;
                        func(connection, action)
                    },
                    None => Err(
                        Error::from(
                            InitiationError::ServiceNotSupported
                        )
                    )
                }
            }
        }
    }
}

pub fn start(
    path: String, 
    key: PKey<Private>,
    server_key: PKey<Public>,
    grp: HandlerGroup
) -> Result<(), Error>
{
    for connection in LocalSocketListener::bind(path)?.incoming() {
        match connection {
            Err(e) => warn!("Error accepting incoming connection: {}", match e.name(){
                Some(n) => n,
                None => "No error name"
            }),
            Ok(connection) => {
                match grp.handle_connection(authorize_server_connection(
                    &key, &server_key, connection
                )) {
                    Ok(result) => info!("{}", result),
                    Err(e) => warn!("{}", match e.name(){
                        Some(n) => n,
                        None => "No error name"
                    })
                }
            },
        }
    }
    Ok(())
}

pub fn af_local_listener(
    listen_address: String, 
    handler: HandlerType,
    permission_db: Pool<ConnectionManager<SqliteConnection>>,
    handler_db: Pool<ConnectionManager<SqliteConnection>>,
    server_key: &PKey<Private>,
    connection_handler: fn(&HandlerType, &Pool<ConnectionManager<SqliteConnection>>, Result<(HandlerType, Action, LocalSocketStream), Error>) -> Result<String, Error>
) -> Result<(), Error>
{
    for conn in LocalSocketListener::bind(listen_address)?.incoming() {
        match conn {
            Err(e) => warn!("{}", format!("Listener connection failed: {}", e)),
            Ok(connection) => {
                let pdb = permission_db.clone();
                let hdb = handler_db.clone();
                let sk = server_key.clone();
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
                            &handler,
                            &hdb,
                            authorize_client_connection(
                                &sk,
                                &pdb, 
                                connection
                            )
                        ) {
                            Ok(log) => info!("{}", log),
                            Err(e) => warn!("{} for pid: {}", match e.name(){
                                Some(n) => n,
                                None => {
                                    dbg!(e);
                                    "No error name"
                                }
                            }, peer_pid)
                        }
                    }
                );
            },
        }
    }
    Ok(())
}