use crate::handler::lib::handler_enums::HandlerType;
use crate::handler::calendar;
use crate::core::config::CoreConfig;
use interprocess::local_socket::{
    LocalSocketListener,
    LocalSocketName,
    LocalSocketStream
};
use log::{
    error,
    warn
};
use std::thread;

fn bus_listener(cb: fn(conn: LocalSocketStream)->())
{
    let listener: LocalSocketListener = match LocalSocketListener::bind("/tmp/quiver.sock"){
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
            Ok(conn) => 
            {
                thread::spawn( move ||
                    {
                        cb(conn);
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


pub fn main<'cfg_lifetime>(config: CoreConfig) -> Result<i8, i8> {

    let active_handlers: Box<[HandlerType]>;

    // If All handlers are set then replace the array with on containing all handlers
    // todo: rewrite this so that All doesn't have to be the first item in the array.
    if config.active_handlers.len() == 1 && config.active_handlers[0] == HandlerType::All
    {
        active_handlers = Box::new([HandlerType::Calendar]);
    }
    else
    {
        active_handlers = config.active_handlers;
    }

    for handler in active_handlers.iter()
    {
        match handler
        {
            HandlerType::Calendar =>
            {
                thread::spawn(move || {
                    bus_listener(calendar::handle_connection);
                });
            }
            HandlerType::All =>
            {
                continue;
            }
        }
    }
    
    return Ok(0);
}