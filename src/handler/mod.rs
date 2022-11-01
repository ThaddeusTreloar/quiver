pub mod lib;
pub mod calendar;

// Internal
use crate::{
    shared::{
        error::*,
        lib::{
        HandlerType,
        Action
        },
        
    },
    connection::{
        authorize_server_connection
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
use std::collections::HashMap;

struct HandlerGroup {
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
    let listener: LocalSocketListener = LocalSocketListener::bind(path)?;

    for connection in listener.incoming() {
        match connection {
            Err(e) => warn!("Error accepting incoming connection: {}", e.name().unwrap()),
            Ok(mut connection) => {
                match grp.handle_connection(authorize_server_connection(
                    &key, &server_key, connection
                )) {
                    Ok(result) => info!("{}", result),
                    Err(e) => warn!("{}", e.name().unwrap())
                }
            },
            Err(e) => ()
        }
    }
    Ok(())
}