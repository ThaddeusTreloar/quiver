use interprocess::local_socket::{
    LocalSocketStream,
};
use std::io::{
    self, 
    prelude::*, 
    BufReader
};

pub fn someFunc() -> ()
{

}

pub fn handle_connection(conn: LocalSocketStream) -> ()
{
    let mut connection = BufReader::new(conn);
    let mut buffer = String::new();

    match connection.read_line(&mut buffer)
    {
        _ => unimplemented!()
    }
}

pub fn handler_read_connection(conn: LocalSocketStream, _db: &())
{

}

pub fn handle_write_connection(conn: LocalSocketStream, _db: &mut ())
{

}