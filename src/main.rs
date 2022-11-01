
mod listener;
mod core;
mod handler;
mod shared;
mod client;
mod connection;

use env_logger;
use log::LevelFilter;

// Internal
use crate::shared::lib::{
    HandlerType,
    build_connection_pool
};
use client::server::server;
use crate::handler::*;
use crate::core::{
    init::init,
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
        { active_handlers = HandlerType::all_handlers() } else {
            active_handlers = config.active_handlers;
        };

    let mut thread_pool: Vec<thread::JoinHandle<()>> = Default::default();

    for handler in active_handlers.iter()
    {
        match handler
        {
            HandlerType::Calendar =>
            {
                let p = permission_db_pool.clone();
                thread_pool.push(thread::spawn(move || {
                    calendar::start_listener(p);
                }));
            },
            HandlerType::Nfc =>
            {
                thread_pool.push(thread::spawn(move || {
                    //nfc::start_listener();
                }));
            },
            HandlerType::Vpn =>
            {
                thread_pool.push(thread::spawn(move || {
                    //vpn::start_listener();
                }));
            }
            HandlerType::All =>
            {
                continue;
            }
        }
    }

    println!("Initialisation Ok. Server Started...");

    for _i in 0..thread_pool.len() {
        thread_pool.remove(0).join().unwrap();
    }
}

#[test]
fn send_item()
{
    use chrono::{
        DateTime,
        offset,
    };
    use crate::core::db::db;
    use crate::shared::lib::*;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
        },
        bn::{
            BigNumContext
        },
        pkey::PKey
    };

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
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

    let privkey = key.private_key_to_der().unwrap();
    let priv_key = PKey::private_key_from_der(&privkey).unwrap();

    let res = client::producer::calendar::put(
        &item,
        &priv_key,
        &"test".to_owned()
    );

    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };

    if let Ok(()) = res {
        assert!(true);
    } else {
        dbg!(res);
        assert!(false);
    }
  
}

#[test]
fn threading_test()
{
    use chrono::{
        DateTime,
        offset,
    };
    use crate::core::db::db;
    use crate::shared::lib::*;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
        },
        bn::{
            BigNumContext
        },
        pkey::PKey
    };

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
    };

    use std::thread;
    let mut thread_pool: Vec<thread::JoinHandle<bool>> = vec![];
    let privkey = key.private_key_to_der().unwrap();
    let priv_key = PKey::private_key_from_der(&privkey).unwrap();

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

    for x in 0..30 {
        let i = item.clone();
        let k = priv_key.clone();
        thread_pool.push(thread::spawn( move || {
            

            let res = client::producer::calendar::put(
                &i,
                &k,
                &"test".to_owned()
            );

            match res {
                Ok(_) => true,
                Err(_) => {
                    dbg!(res);
                    false
                }
            }
        }));
    }

    for _i in 0..thread_pool.len() {
        if !thread_pool.remove(0).join().unwrap(){
            assert!(false);
        };
    }

    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };

    assert!(true);    
}

#[test]
fn send_item_different_user()
{
    use chrono::{
        DateTime,
        offset,
    };
    use crate::core::db::db;
    use crate::shared::lib::*;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
        },
        bn::{
            BigNumContext
        },
        pkey::PKey
    };

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
    };

    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test2".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
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

    let privkey = key.private_key_to_der().unwrap();
    let priv_key = PKey::private_key_from_der(&privkey).unwrap();

    let res = client::producer::calendar::put(
        &item,
        &priv_key,
        &"test".to_owned()
    );

    if let Err(_e) = db::remove_service("test2".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };
    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };


    use crate::shared::error;

    match res {
        Ok(_) => assert!(false),
        Err(e) => {
            dbg!(&e);
            dbg!(e.name());
            match e.name() {
                Some("quiver::shared::error::AuthenticationError") => assert!(true),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn send_item_no_permissions()
{
    use chrono::{
        DateTime,
        offset,
    };
    use crate::core::db::db;
    use crate::shared::lib::*;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
        },
        bn::{
            BigNumContext
        },
        pkey::PKey
    };

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::Read,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
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

    let privkey = key.private_key_to_der().unwrap();
    let priv_key = PKey::private_key_from_der(&privkey).unwrap();

    let res = client::producer::calendar::put(
        &item,
        &priv_key,
        &"test".to_owned()
    );

    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };

    match res {
        Ok(_) => assert!(false),
        Err(e) => {
            dbg!(&e);
            dbg!(&e.name());
            match e.name() {
                Some("quiver::shared::error::AuthorizationError") => assert!(true),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn send_item_wrong_permissions()
{
    use chrono::{
        DateTime,
        offset,
    };
    use crate::core::db::db;
    use crate::shared::lib::*;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
        },
        bn::{
            BigNumContext
        },
        pkey::PKey
    };

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::Read,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
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

    let privkey = key.private_key_to_der().unwrap();
    let priv_key = PKey::private_key_from_der(&privkey).unwrap();

    let res = client::producer::calendar::put(
        &item,
        &priv_key,
        &"test".to_owned()
    );

    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };

    match res {
        Ok(_) => assert!(false),
        Err(e) => {
            dbg!(&e);
            dbg!(e.name());
            match e.name() {
                Some("quiver::shared::error::AuthorizationError") => assert!(true),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn send_item_wrong_key()
{
    use chrono::{
        DateTime,
        offset,
    };
    use crate::core::db::db;
    use crate::shared::lib::*;
    use openssl::{
        ec::{
            EcKey,
            EcGroup,
        },
        bn::{
            BigNumContext
        },
        pkey::PKey
    };

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
    ) {
        dbg!(_e);
        assert!(false);
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

    let key = EcKey::generate(&group).unwrap();

    let privkey = key.private_key_to_der().unwrap();
    let priv_key = PKey::private_key_from_der(&privkey).unwrap();

    let res = client::producer::calendar::put(
        &item,
        &priv_key,
        &"test".to_owned()
    );

    if let Err(_e) = db::remove_service("test".to_owned(), &mut connection)
    {
        dbg!(_e);
        assert!(false);
    };

    match res {
        Ok(_) => assert!(false),
        Err(e) => {
            dbg!(&e);
            dbg!(e.name());
            match e.name() {
                Some("quiver::shared::error::AuthenticationError") => assert!(true),
                _ => assert!(false),
            }
        }
    }
}

#[test]

fn sql_request_all()
{
    use crate::core::db::db;

    let mut conn = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    match db::get_all_services(&conn)
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

//#[test]

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

    let mut connection = crate::shared::lib::build_connection_pool("run/core.sqlite".to_owned()).unwrap();

    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let pubkey = key.public_key_to_der().unwrap();

    if let Err(_e) = db::register_service(
        "test".to_owned(),
        Box::new(vec![Permission{
            state: PermissionState::ReadWrite,
            service: HandlerType::Calendar,
            include: vec!["All".to_owned()]
        }]),
        Box::new(vec![]),
        pubkey,
        &connection
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

    if let Err(_e) = db::get_service(&"test".to_owned(), &mut connection)
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