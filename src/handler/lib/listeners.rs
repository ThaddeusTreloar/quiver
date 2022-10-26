use interprocess::local_socket::{
    LocalSocketListener,
    LocalSocketStream
};
use std::{
    thread
};
use log::{
    error,
    warn
};
use failure::Error;


pub fn af_local_listener(listen_address: String, 
    connection_handler: fn(connection: LocalSocketStream) -> Result<(), Error>)
{
    let listener: LocalSocketListener = match LocalSocketListener::bind(listen_address){
        Ok(val) => val,
        Err(e) => {
            error!("Listener failed to initialise: {e}");
            return;
        }
    };
    
    for mut conn in listener.incoming()
    {
        match conn
        {
            Ok(connection) => 
            {
                thread::spawn( move ||
                    {
                        connection_handler(connection);
                    }
                );
            }
            Err(e) => 
            {
                warn!("Listener connection failed: {e}");
            }
        }
    }
}
