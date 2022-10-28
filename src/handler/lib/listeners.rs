// Internal
use crate::{
    shared::{
        lib::{
            from_reader
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
use interprocess::local_socket::{
    LocalSocketListener,
    LocalSocketStream
};
use serde_json::{
    to_writer
};
use std::{
    thread
};
use log::{
    error,
    warn
};
use failure::Error;

fn identify(
    connection: Result<LocalSocketStream, Error>
) -> Result<LocalSocketStream, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok(mut connection) => {
            let name: String = from_reader(&mut connection);

            let query: Vec<ServiceQuery> = get_service(name, )?;
        }
    }
}

pub fn af_local_listener(
    listen_address: String, 
    handler: &HandlerType,
    connection_handler: fn(connection: LocalSocketStream) -> ()
) -> Result<(), Error>
{
    let listener: LocalSocketListener = LocalSocketListener::bind(listen_address)?;
    
    for mut conn in listener.incoming() {
        match conn {
            Ok(connection) => {
                thread::spawn( move ||
                    {
                        handle(authrorize(handler, identify(connection)))
                    }
                );
            },
            Err(e) => warn!(format!("Listener connection failed: {}", e))
        }
    }
}


pub fn 

transaction(authorize(handler: String, identify(accept()))