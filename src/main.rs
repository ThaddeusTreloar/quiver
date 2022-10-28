
mod core;
mod security;
mod handler;
mod shared;
use env_logger;
use log::LevelFilter;
mod client;
// Internal
use crate::shared::lib::{
    HandlerType,
    
};
use crate::handler::*;
use crate::core::{
    init::init,
    lib::build_connection_pool,
};

// External
use std::{
    thread,
    io::prelude::*, 
};

fn main() {

    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();
    
    let config = init();

    let permission_db_pool = match build_connection_pool(config.core_db_path){
        Ok(val) => val,
        // Was going to do some pattern matching to recover from this
        // but it is not immediatly apparent what the errors might be.
        // Will todo when I have time to dig through the r2d2 source.
        Err(e) => unimplemented!(),
    };

    let active_handlers: Vec<HandlerType>;
    // Need to confirm that the vec is less than usize::MAX and resize accordingly
    // Potentially unnecessary but do it anyway.
    if config.active_handlers.len() > usize::MAX
    {
        // todo.
        // config.active_handlers.drain(usize::MAX..);
    }

    // If All handlers are set then replace the array with one containing all handlers
    // todo: rewrite this so that All doesn't have to be the first item in the array.
    if config.active_handlers.contains(&HandlerType::All) 
        { active_handlers = HandlerType::all_handlers() };

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
}

#[test]
fn send_item()
{
    use chrono::{
        DateTime,
        offset,
    };
    let item = shared::calendar::CalendarItem{
        title: "SomeItem".to_owned(),
        start: "2022-09-24T12:00:00Z".parse::<DateTime<offset::Utc>>().unwrap(),
        end: "2022-09-24T14:00:00Z".parse::<DateTime<offset::Utc>>().unwrap(),
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

    if let Ok(()) = res {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]

fn sql_request_all()
{
    use crate::core::db::db;

    let mut conn = db::establish_connection("run/core.sqlite").unwrap();

    match db::get_all_services(&mut conn)
    {
        Ok(val) => {
            dbg!(val);
            assert!(true);
            return;
        }
        Err(e) =>
        {
            dbg!(e);
            assert!(false);
            return;
        }
    };
}

#[test]

fn sql_create_filter_delete()
{
    use crate::core::db::db;
    use crate::shared::lib::*;
    use crate::client::permission;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
            PointConversionForm
        },
        bn::{
            BigNumContext
        }
    };

    let mut connection = db::establish_connection("run/core.sqlite").unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = EcKey::from_public_key(
        &group,
        key.public_key()
    ).unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &mut connection
    ) {
        dbg!(_e);
        assert!(false);
    };

    /*if let Err(_e) = db::update_service("test".to_owned(), 
        service_perm: Box<Vec<Permission>>, 
        service_exclude: Box<Vec<HandlerType>>, 
        connection)
    {
        
    };*/

    if let Err(_e) = db::get_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };

    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };
}