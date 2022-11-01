pub mod server;

// Internal
use crate::shared::lib::{
    HandlerType,
    Action
};
use crate::listener;

//External
use openssl::{
    pkey::{
        PKey,
        Private  
    },
    sign::{
        Signer,
        Verifier
    }
};
use log::{
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
                    Some(func) => func(connection, action),
                    None => Ok("()".to_owned())
                }
            }
        }
    }
}

pub fn start(
    path: String, 
    priv_key: PKey<Private>,
    grp: HandlerGroup
) -> Result<(), Error>
{
    let listener: LocalSocketListener = LocalSocketListener::bind(path)?;

    for connection in listener.incoming() {
        match connection {
            Err(e) => warn!("Error accepting incoming connection: {}", e.name().unwrap()),
            Ok(mut connection) => {
                authrorize_connection()
            },
            Err(e) => ()
        }
    }
    Ok(())
}