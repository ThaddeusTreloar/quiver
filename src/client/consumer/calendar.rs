
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
        CALENDAR,
    },
    error::SyncError
};
use interprocess::local_socket::{
    LocalSocketStream
};
use serde_json::{
    Value,
    from_str,
    from_reader,
    to_writer
};
use serde::de;
use failure::Error;

pub fn push(item: CalendarItem) -> Result<(), Error>
{
    let mut connection: LocalSocketStream = LocalSocketStream::connect(CALENDAR)?;

    to_writer(&mut connection, &Action::Put)?;

    let res: Action = from_reader(&mut connection)?;

    match res
    {
        Ready => (),
        other => {
            // todo: fix returning error.
            return Err(SyncError::UnexpectedActionError{
                expected: 'Ready',
                recieved: other,
                action: "Put",
                service: "Calendar"
            });
        }
    };

    to_writer(&mut connection, &item)?;
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