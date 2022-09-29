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
            CALENDAR,
            Action
        }
    }
};
use bincode::{
    deserialize,
    serialize_into
};
use interprocess::local_socket::{
    LocalSocketStream
};

pub fn start_listener()
{
    listeners::af_local_listener(CALENDAR.to_owned(), handle_connection);
}

fn handle_connection(mut connection: LocalSocketStream) -> ()
{
    let mut buffer: [u8; 4] = [b"0"[0]; 4];

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

    match connection.read_exact(&mut buffer)
    {
        Ok(_l) => (),
        Err(e) =>
        {
            info!("Failed to read action due to: {e}\nClosing connection...");
            return
        }
    };

    match deserialize(&buffer)
    {
        Ok(connection_type) => 
        {
            match connection_type
            {
                Action::Put => 
                {
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

fn handle_put_connection(mut connection:  LocalSocketStream, _db: &mut ()) -> Result<(), &'static str>
{
    match serialize_into(&mut connection, &Action::Ready) {
        Ok(_ok) => (),
        Err(e) => return Err("{e}"),
    }

    let mut buffer: Vec<u8> = Vec::new();

    // todo: Potentially unsafe.
    match connection.read_to_end(&mut buffer)
    {
        Ok(_l) => (),
        Err(e) =>
        {
            return Err("{e}");
        }
    };

    let item: CalendarItem = match deserialize(&buffer[..buffer.len()])
    {
        Ok(item) => item,
        Err(e) => return Err("{e}"),
    };

    //dbg!(item);

    Ok(())
}

fn _handle_edit_connection(_conn: LocalSocketStream, _db: &mut ())
{

}