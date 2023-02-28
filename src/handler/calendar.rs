// Internal
use crate::{
    handler::{
        Connection
    },
    shared::{
        structs::calendar::CalendarItem,
        lib::{
            CALENDAR_SOCKET_ADDR,
            Action,
            HandlerType,
            build_connection_pool,
            from_reader,
        },
        errors::*
    }
    
};

// External
use failure::Error;
use interprocess::local_socket::LocalSocketStream;
use serde_json::{
    to_writer,
};
use diesel::{
    r2d2::{
        Pool,
        ConnectionManager
    },
    sqlite::SqliteConnection
};
use openssl::pkey::{
    PKey,
    Private
};

use crate::handler::HandlerDatabaseConnectionPool;
use tokio::net::UnixStream;

fn handle_connection(
    connection: Connection,
    handler_db: &HandlerDatabaseConnectionPool,
) -> Result<String, Error>
{

    let (stream, action) = connection.into_parts();


    match action {
        Action::Put => {
            let mut len_buff = [0u8; 8];
            stream.try_read(&mut len_buff)?;

            let len: u64 = serde_json::de::from_slice(&len_buff)?;

            let mut buff = vec![0u8; len as usize];

            // Todo: check length validated
            stream.try_read(&mut buff)?;

            let item = serde_json::de::from_slice::<CalendarItem>(&buff);

            match item {
                Ok(_value) => {
                    stream.try_write(serde_json::ser::to_vec(&true)?.as_slice())?;
                    Ok("Successful 'put' from client".to_owned())
                }
                Err(e) => {
                    // Todo: some error message
                    stream.try_write(serde_json::ser::to_vec(&false)?.as_slice())?;
                    Err(e.into())
                },
            }
        },
        a => Err(Error::from(
            ConnectionActionError::UnsupportedActionError{
                recieved: format!("{}", a),
                service: "calendar".to_owned()
            }
        )),
    }
}