use crate::handler::lib::handler_enums::HandlerType;
use crate::handler::calendar;
use crate::core::config::CoreConfig;
use interprocess::local_socket::{
    LocalSocketStream
};
use log::{
    error,
    warn
};
use std::{
    thread,
    time,
    io::{
    prelude::*, 
    BufWriter
}};
use crate::shared::lib::request;
use bincode;

pub fn main<'cfg_lifetime>(config: CoreConfig) -> Result<i8, i8> {
    
    let active_handlers: Vec<HandlerType>;
    // Need to confirm that the vec is less than usize::MAX and resize accordingly
    // Potentially unnecessary but do it anyway.
    if config.active_handlers.len() > usize::MAX
    {
        // todo.
        // config.active_handlers.drain(usize::MAX..);
    }
    // If All handlers are set then replace the array with on containing all handlers
    // todo: rewrite this so that All doesn't have to be the first item in the array.
    match config.active_handlers.iter().position(|&term| term == HandlerType::All)
    {
        Some(_i) => 
        {
            active_handlers = vec!(HandlerType::Calendar);
        }
        None => 
        {
            active_handlers = config.active_handlers;
        }
    };
    
    let mut thread_pool: Vec<thread::JoinHandle<()>> = Default::default();

    for handler in active_handlers.iter()
    {
        match handler
        {
            HandlerType::Calendar =>
            {
                let path = config.sockets_path.clone();
                thread_pool.push(thread::spawn(move || {
                    calendar::start_listener(path);
                }));
            }
            HandlerType::All =>
            {
                continue;
            }
        }
    }

    thread::sleep(time::Duration::from_secs(1));

    let c: LocalSocketStream = match LocalSocketStream::connect("/tmp/quiver.calendar.sock")
    {
        Ok(connection) => {
            connection
        }
        Err(e) => 
        {
            panic!("{e}");
        }
    };
    
    let mut connection_writer = BufWriter::new(c);

    match bincode::serialize_into(&mut connection_writer, &request::Action::Put)
    {
        Ok(_b) => 
        {
            connection_writer.flush().unwrap();
            println!("Sent...");
        }
        Err(e) =>
        {
            panic!("{e}");
        }
    };

    thread::sleep(time::Duration::from_secs(1));

    return Ok(1);
}