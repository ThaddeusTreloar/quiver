use std::{
    thread,
    io::{
        prelude::*, 
        BufReader
    }
};
use log::{
    error,
    warn
};
use crate::handler::lib::listeners;
use crate::shared::lib::request;
use bincode::deserialize;
use interprocess::local_socket::{
    LocalSocketStream
};

pub fn start_listener(sockets_path: String)
{
    listeners::af_local_listener(sockets_path+"quiver.calendar.sock", handle_connection);
}

fn handle_connection(conn: LocalSocketStream) -> ()
{
    let mut connection = BufReader::new(conn);
    let mut buffer: [u8; 32] = [b"0"[0]; 32];

    println!("Client connected");

    match connection.read(&mut buffer)
    {
        Ok(len) => 
        {
            println!("Read: {len} bytes...");
            dbg!(&buffer[0..len]);
            let _a: request::Action = match deserialize(&buffer[0..len])
            {
                Ok(val) => {
                    dbg!(val)
                },
                Err(e) => 
                {
                    panic!("{e}");
                }
            };
        }
        Err(e) => 
        {
            println!("{e}");
        }
    }
}

fn _handler_get_connection(_conn: LocalSocketStream, _db: &())
{

}

fn _handle_put_connection(_conn: LocalSocketStream, _db: &mut ())
{

}

fn _handle_edit_connection(_conn: LocalSocketStream, _db: &mut ())
{

}