use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use interprocess::local_socket::LocalSocketStream;
use failure::Error;
use crate::shared::lib::{HandlerType, Action};

trait Service {
    fn handle_connection(
        handler_db: &Pool<ConnectionManager<SqliteConnection>>,
        connection: Result<(HandlerType, Action, LocalSocketStream), Error>, 
    ) -> Result<String, Error>;
}