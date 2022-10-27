
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
use crate::shared::{
    calendar::CalendarItem,
    lib::{
        Action,
        CALENDAR_SOCKET_ADDR,
    },
    error::ConnectionActionError
};
use interprocess::local_socket::{
    LocalSocketStream
};
use serde_json::{
    Value,
    from_str,
    from_reader,
    to_writer,
    Deserializer,
    Serializer
};
use serde::Deserialize;
use failure::Error;

pub fn push(item: CalendarItem) -> Result<(), Error>
{
    let mut connection: LocalSocketStream = LocalSocketStream::connect(CALENDAR_SOCKET_ADDR)?;

    to_writer(&mut connection, &Action::Put)?;

    let mut deser = Deserializer::from_reader(&mut connection);

    let res: Action = Action::deserialize(&mut deser)?;

    match res
    {
        Action::Ready => (),
        other => {
            // todo: fix returning error.
            return Err(
                Error::from(
                    ConnectionActionError::UnexpectedActionError{
                        expected: "Ready".to_owned(),
                        recieved: other.to_string(),
                        action: "Put".to_owned(),
                        service: "Calendar".to_owned()
                    }
                )
            );
        }
    };

    to_writer(&mut connection, &item)?;

    Ok(())
}


/*
todo rework
*/

fn connect() -> ()
{
    ()
} 

fn push_single(item: CalendarItem) -> Result<(), &'static str>
{
    Ok(())
}

fn _push(connection: LocalSocketStream, item: CalendarItem) -> Result<(), &'static str>
{
    Ok(())
}