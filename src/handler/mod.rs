pub mod calendar;

// Internal
use crate::{
    shared::{
        errors::*,
        lib::{
        HandlerType,
        Action,
        ConnectionHandler
        },
        
    },
    connection_async::{
        authorize_server_connection,
        authorize_client_connection
    }, listener
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
mod generic;

use tokio::net::UnixStream;
use futures::Future;
use std::pin::Pin;

pub struct Connection {
    pub stream: UnixStream,
    pub action: Action,
}

impl Connection {
    pub async fn new(mut stream: UnixStream) -> Result<Self, Error> {
        let action = Action::from_reader(&mut stream).await?;
        Ok(Self { stream, action })
    }

    pub fn into_parts(self) -> (UnixStream, Action) {
        (self.stream, self.action)
    }
}


pub struct Handler {
    handler_db: HandlerDatabaseConnectionPool,
    handler_fn: Box<dyn Fn(Connection, &HandlerDatabaseConnectionPool) -> Pin<dyn Future<Output = Result<(), Error>>>>,
}

/*
When writing a new handler, one should never write to the permissions db.
All write requests to the permission db should be sent to the permission handler.
*/

pub struct HandlerGroup {
    handlers: HashMap<HandlerType, Handler>
}

impl HandlerGroup
{
    fn _add_handler(
        &mut self, type_: HandlerType, 
        handler: Handler
    ) {
        self.handlers.insert(type_, handler);
    }

    async fn handle_connection(
        &self,
        connection: Result<(HandlerType, Action, UnixStream), Error>
    ) -> Result<String, Error> {
        match connection {
            Err(e) => Err(e),
            Ok((service, action, mut stream)) => {
                match self.handlers.get(&service) {
                    Some(handler) => {
                        // Slight coupling. Can't put @JMP001 as there is no way
                        // to check the connection has access to a handler from
                        // within the auth process. May need redesign.
                        stream.writable().await?;
                        stream.try_write(&serde_json::ser::to_vec(&true)?)?;
                        (handler.handler_fn)(stream, &handler.handler_db)
                    },
                    None => {
                        stream.writable().await?;
                        stream.try_write(&serde_json::ser::to_vec(&false)?)?;
                        Err(
                            Error::from(
                                InitiationError::ServiceNotSupported
                            )
                        )
                    }
                }
            }
        }
    }
}

use tokio::net::UnixListener;

pub async fn start(
    path: String, 
    key: PKey<Private>,
    server_key: PKey<Public>,
    grp: HandlerGroup
) -> Result<(), Error>
{
    let connection = UnixListener::bind(path)?;

    loop {
        match connection.accept().await {
            Err(e) => warn!("Error accepting incoming connection: {}", e.name().unwrap_or("No error name")),
            Ok((connection, _addr)) => {
                match grp.handle_connection(
                    authorize_server_connection(
                        &key, &server_key, connection
                    ).await
                ).await {
                    Ok(result) => info!("{}", result),
                    Err(e) => warn!("{}", e.name().unwrap_or("No error name"))
                }
            },
        }
    }
}
/*
pub async fn start_async(
    path: String, 
    key: PKey<Private>,
    server_key: PKey<Public>,
    grp: HandlerGroup
) -> Result<(), Error>
{
    let listener = UnixListener::bind(path)?;

    loop {
        match listener.accept().await {
            Err(e) => warn!("Error accepting incoming connection: {}", e.name().unwrap_or("No error name")),
            Ok((stream, _addr)) => {
                match grp.handle_connection(
                    authorize_client_connection(
                        &key, &server_key, stream
                    )
                ) {
                    Ok(result) => info!("{}", result),
                    Err(e) => warn!("{}", e.name().unwrap_or("No error name"))
                }
            },
        }
    }
} */



/*pub fn af_local_listener(
    listen_address: String, 
    handler: HandlerType,
    permission_db: Pool<ConnectionManager<SqliteConnection>>,
    handler_db: Pool<ConnectionManager<SqliteConnection>>,
    server_key: &PKey<Private>,
    connection_handler: ConnectionHandler
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
}*/

pub fn get_service_providers(service: HandlerType) -> Result<Vec<String>, Error> { Err(Error::from(crate::shared::errors::InitiationError::ServiceNotSupported)) }

pub fn get_service_consumers(service: HandlerType) -> Result<Vec<String>, Error> { Err(Error::from(crate::shared::errors::InitiationError::ServiceNotSupported)) }

struct HandlerCache {
    // schema
    // connectionPool
    // functions that check providers
}


//struct ServiceSchema {
    /* 
    Field {
        Index,
        Range
    }

    fn(Vec<Field<Type>>) -> Result<ServiceSchema, Error>
    */
//}

trait ServiceSchema {
    
}

struct ServiceSet {
    /*
    

    fn(Result<Action<ServiceSchema>, Error>) -> Result<String, Error>
    */
}

enum FieldValue {
    Exact(fn(String, String) -> String),
    Series(fn(String, Vec<FieldValue>) -> Vec<String>),
    Contains(fn(String, String) -> String),
    NotContains(fn(String, String) -> String),
    Range(fn(String, String, String) -> (String, String))
}

struct Field {
    name: String,
    state: FieldValue,
}
type ActionFunction = fn(Result<Vec<Field>, Error>, SqliteConnection) -> Result<String, Error>;
struct ServiceActions {
    get: ActionFunction,
    put: ActionFunction,
    edit: ActionFunction,
    delete: ActionFunction,
}


/*

Schema {
    Fields: State
}

*/



mod cache {

    struct cache {

    }

    trait Cacher {

    }

    impl Cacher for cache {

    }

    /*
    
    The below record describes as service named 'someService' who had read/write permissions
    to the calendar interface with access to all calendar services including the service
    excluded service 'Other'. It excludes itself from having it's calendar data read by
    other services without a service being explicitly granted permissions to it.
    It is identified via a challenge using the key 'somePubKey' before it's actions are
    authorized according to permissions listed in perm.

    path             | index          | metadata
    google::calendar | unique integer |

    */
}

type HandlerDatabaseConnectionPool = Pool<ConnectionManager<SqliteConnection>>;