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
use crate::{
    handler::lib::listeners,
    shared::{
        lib::{
            request::{
                FunctionArgTypes,
                NFC,
                Action
            },
        }
    }
};
use bincode::deserialize;
use interprocess::local_socket::{
    LocalSocketStream
};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug)]
struct NfcFunction {
    name: String,
    args: Vec<(String, FunctionArgTypes)>
}

#[derive(Debug)]
struct RsaKey {
    name: String,
    key: String
}

#[derive(Debug)]
struct EccKey {
    name: String,
    key: String
}

#[derive(Debug)]
struct NtruKey {
    name: String,
    key: String
}

#[derive(Debug)]
enum NfcKey
{
    Rsa(RsaKey),
    Ecc(EccKey),
    NtruKey(NtruKey)
}


#[derive(Debug)]
enum NfcWatcher {
    Key(NfcKey),
    Function(NfcFunction),
}

pub fn start_listener()
{
    listeners::af_local_listener(NFC.to_owned(), handle_connection);
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
            let _a: Action = match deserialize(&buffer[0..len])
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