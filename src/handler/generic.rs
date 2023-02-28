

use crate::shared::lib::{HandlerType, Action};
use openssl::pkey::{Private, PKey};
use tokio::net::UnixStream;
use failure::Error;
use crate::handler::HandlerDatabaseConnectionPool;
use futures::Future;

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
    handler_fn: Box<dyn Fn(Connection, &HandlerDatabaseConnectionPool) -> dyn Future<Output = Result<(), Error>>>,
}

