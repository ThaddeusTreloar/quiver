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

    let item = shared::calendar::CalendarItem{
        title: "SomeItem".to_owned(),
        start: match DateTime::from_str("2022-09-24T12:00:00Z"){
            Ok(val) => val,
            Err(e) => panic!("{e}"),
        },
        end: match DateTime::from_str("2022-09-24T14:00:00Z"){
            Ok(val) => val,
            Err(e) => panic!("{e}"),
        },
        guests: (),
        location: shared::location::Location{
            common_name: "SomePlace".to_owned(),
            coordinate: shared::location::Coordinate {
                latitude: 5.0,
                lontitude: 6.0,
                altitude: 7.0,
            },
            address: "SomeAddress".to_owned(),
        },
        description: "An Event".to_owned(),
        // Attachments will link to either a sharing link or local file.
        // Sharing links will be visible to all with permissions to read
        // it. Maybe add something that automatically offers to add permissions to guests.
        // Local file will only be available to owner. Maybe add automatic generation
        // of a sharing link eg. local_file->upload_to_cloud->generate_link
        attachments: (),
        // Action will link to some action from another interface eg. Call, meeting link etc..
        action: (),
        // Guests will be a link to INS or contact or something
        owner: ()
    };
    let res = client::consumer::calendar::push(item);
    //dbg!(res);
    
    dbg!(res);

    thread::sleep(time::Duration::from_secs(1));

    return Ok(1);
}