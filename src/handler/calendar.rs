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
use crate::{
    handler::lib::listeners,
    shared::{
        calendar::CalendarItem,
        lib::{
            CALENDAR_SOCKET_ADDR,
            Action
        },
        error
    }
    
};
use interprocess::local_socket::{
    LocalSocketStream
};
use failure::Error;
use serde_json::{
    from_reader,
    to_writer,
    Deserializer,
};
use serde::Deserialize;

pub fn start_listener()
{
    listeners::af_local_listener(CALENDAR_SOCKET_ADDR.to_owned(), handle_connection);
}

fn handle_connection(mut connection: LocalSocketStream) -> ()
{
    let peer_pid: String = match connection.peer_pid() 
    {
        Ok(peer_id) => 
        {
            info!("Client connnected, pid<{peer_id}>.");
            peer_id.to_string()
        },
        Err(_e) => 
        {
            info!("Client connection, no pid available.");
            "Unavailable".to_owned()
        }
    };

    let mut deser = Deserializer::from_reader(&mut connection);

    match Action::deserialize(&mut deser)
    {
        Ok(connection_type) => 
        {
            match connection_type
            {
                Action::Put => 
                {
                    info!("Handling Put connection for client {peer_pid}");
                    match handle_put_connection(connection, &mut ())
                    {
                        Ok(_ok) => info!("Put action successful for pid: '{peer_pid}'."),
                        Err(e) => info!("Put action failed for pid: '{peer_pid}', due to: {e}"),
                    }
                    return;
                }
                other =>
                {
                    info!("Action {other} not supported for calendar client connection from 
                    '{peer_pid}'. Closing connection...");
                    return;
                }
            }
        }
        Err(e) => 
        {
            info!("Failed to read action type from '{peer_pid}' due to: {e}.");
            return;
        }
    };
}

fn _handler_get_connection(_conn: LocalSocketStream, _db: &())
{

}

fn handle_put_connection(mut connection:  LocalSocketStream, _db: &mut ()) -> Result<(), Error>
{
    to_writer(&mut connection, &Action::Ready)?;

    let i: CalendarItem = from_reader(&mut connection)?;

    dbg!(i);
    // Cache/forward

    Ok(())
}

fn _handle_edit_connection(_conn: LocalSocketStream, _db: &mut ())
{

}