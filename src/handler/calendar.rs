// Internal
use crate::{
    handler::lib::listeners,
    shared::{
        calendar::CalendarItem,
        lib::{
            CALENDAR_SOCKET_ADDR,
            Action,
            HandlerType,
            build_connection_pool,
            from_reader
        },
        error::*
    }
    
};

// External
use std::{
    io::{
        prelude::*, 
    }
};
use log::{
    info,
    error,
    warn
};

use interprocess::local_socket::{
    LocalSocketStream
};
use failure::Error;
use serde_json::{
    to_writer,
    Deserializer,
};
use serde::Deserialize;
use diesel::{
    r2d2::{
        Pool,
        ConnectionManager
    },
    sqlite::SqliteConnection
};

pub fn start_listener(permission_db: Pool<ConnectionManager<SqliteConnection>>)
{
    listeners::af_local_listener(
        CALENDAR_SOCKET_ADDR.to_owned(), 
        HandlerType::Calendar,
        permission_db,
        match build_connection_pool(
            "run/calendar.sqlite".to_owned()
        ){
            Ok(val) => val,
            // Some recovery: todo
            Err(e) => {
                unimplemented!();
            }
        },
        handle_connection,
    );
}

fn handle_connection(
    mut connection: Result<LocalSocketStream, Error>, 
    handler_db: &Pool<ConnectionManager<SqliteConnection>>
)
{
    dbg!(&connection);
    match connection {
        Err(e) => return,
        Ok(mut connection) => match from_reader(&mut connection) {
            Ok(action) => match action {
                Action::Put => if let Ok(value) = from_reader::<CalendarItem>(&mut connection) {
                    dbg!(value);
                    to_writer(&mut connection, &Ok::<(), TransactionError>(()));
                },
                _ => return,
            },
            Err(e) => return,
        }
    };
}