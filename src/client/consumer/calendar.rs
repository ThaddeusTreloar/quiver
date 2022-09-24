
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
        request::{
            Action::*,
            CALENDAR,
        }
    }
};
use interprocess::local_socket::{
    LocalSocketStream
};
use bincode::{
    serialize_into,
    deserialize
};

pub fn push(item: CalendarItem) -> Result<(), &'static str>
{

    let mut connection: LocalSocketStream = match LocalSocketStream::connect(CALENDAR)
    {
        Ok(connection) => connection,
        Err(e) => 
        {
            dbg!(e);
            return Err("Failed to connection to {CALENDAR} due to: {e}");
        }
    };

    match serialize_into(&mut connection, &Put) {
        Ok(_ok) => (),
        Err(e) => 
        {
            return Err("Failed to write request::PUT to {CALENDAR} due to: {e}");
        }
    };

    let mut buffer: [u8; 4] = [b"0"[0]; 4];

    match connection.read_exact(&mut buffer)
    {
        Ok(_l) => (),
        Err(e) => return Err("Failed to read action from {CALENDAR} due to: {e}\nClosing connection..."),
    };

    match deserialize(&mut buffer)
    {
        Ok(action) =>
        {
            match action
            {
                Ready => (),
                other => return Err("Connection out-of-sync while attempting PUT. Expected READY, recieved {other}."),
            }
        }
        Err(e) => return Err("Unable to deserialise action while attempting PUT\n
                            Expected READY, failed due to {e}"),
    };

    match serialize_into(&mut connection, &item)
    {
        Ok(_ok) => return Ok(()),
        Err(e) => return Err("Error while sending caleendar_item during PUT. Failed due to: {e}"),
    }
}