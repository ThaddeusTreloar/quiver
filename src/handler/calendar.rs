// Internal
use crate::{
    handler::af_local_listener,
    shared::{
        structs::calendar::CalendarItem,
        lib::{
            CALENDAR_SOCKET_ADDR,
            Action,
            HandlerType,
            build_connection_pool,
            from_reader
        },
        errors::*
    }
    
};

// External
use failure::Error;
use interprocess::local_socket::LocalSocketStream;
use serde_json::{
    to_writer,
    to_string
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
use std::io::Write;

pub fn start_listener(server_key: PKey<Private>, permission_db: Pool<ConnectionManager<SqliteConnection>>)
{
    af_local_listener(
        CALENDAR_SOCKET_ADDR.to_owned(), 
        HandlerType::Calendar,
        permission_db,
        match build_connection_pool(
            "run/calendar.sqlite".to_owned()
        ){
            Ok(val) => val,
            // Some recovery: todo
            Err(_e) => {
                unimplemented!();
            }
        },
        &server_key,
        handle_connection,
    );
}

fn handle_connection(
    service: &HandlerType,
    _handler_db: &Pool<ConnectionManager<SqliteConnection>>,
    connection: Result<(HandlerType, Action, LocalSocketStream), Error>, 
) -> Result<String, Error>
{
    match connection {
        Err(e) => Err(e),
        Ok((handler, action, mut connection)) => {
            if *service != handler { 
                to_writer(&mut connection, &false)?; 
                return Err(
                    Error::from(
                        InitiationError::ServiceNotSupported
                    )
                ) 
            } else { to_writer(&mut connection, &true)? };

            match action {
                Action::Put => match from_reader::<CalendarItem>(&mut connection) {
                    Ok(_value) => {
                        to_writer(&mut connection, &true)?;
                        Ok("Successful 'put' from client".to_owned())
                    }
                    Err(e) => {
                        to_writer(&mut connection, &false)?;
                        Err(e)
                    },
                },
                a => Err(Error::from(
                    ConnectionActionError::UnsupportedActionError{
                        recieved: format!("{}", a),
                        service: "calendar".to_owned()
                    }
                )),
            }
        }
    }
}