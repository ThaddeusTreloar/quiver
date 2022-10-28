use crate::shared::lib::{
    HandlerType,
    Action
};
use crate::handler::*;
use crate::core::{
    db,
    config::CoreConfig,
    permission,
};
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
use bincode;
use crate::client;

use crate::shared;
use chrono::{
    DateTime,
    offset
};
use std::str::FromStr;

pub fn main(config: CoreConfig) -> Result<i8, i8> {
    
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
            active_handlers = vec!(HandlerType::Calendar, HandlerType::Nfc, HandlerType::Vpn);
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
                thread_pool.push(thread::spawn(move || {
                    calendar::start_listener();
                }));
            },
            HandlerType::Nfc =>
            {
                thread_pool.push(thread::spawn(move || {
                    nfc::start_listener();
                }));
            },
            HandlerType::Vpn =>
            {
                thread_pool.push(thread::spawn(move || {
                    vpn::start_listener();
                }));
            }
            HandlerType::All =>
            {
                continue;
            }
        }
    }

    println!("Initialisation Ok. Server Started...");

    thread_pool.remove(0).join().unwrap();
    return Ok(1);
}
