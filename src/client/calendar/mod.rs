// Internal
use crate::{
    shared::{
        structs::calendar::CalendarItem,
        lib::{
            Action,
            HandlerType,
            CALENDAR_SOCKET_ADDR,
            from_reader
        },
        errors::*,
    },
    connection::connect_to_server
};

// External
use serde_json::to_writer;
use openssl::{
    pkey::{
        PKey,
        Private,
        Public
    }
};
use chrono::{
    DateTime,
    offset::Utc
};
use failure::Error;

trait CalendarHandler {
    
}

pub fn put_series(
    item: &Vec<CalendarItem>,
    priv_key: &PKey<Private>,
    server_key: &PKey<Public>,
    name: &String
) -> Result<(), Error>
{
    let mut connection = connect_to_server(
        CALENDAR_SOCKET_ADDR, 
        priv_key, 
        server_key,
        name, 
        &HandlerType::Calendar,
        &Action::Put)?;

    match from_reader(&mut connection)? {
        true => {
            to_writer(&mut connection, item)?;

            match from_reader(&mut connection)? { 
                true => {
                    Ok(())
                },
                false => Err(
                    Error::from(
                        TransactionError::SerializeToWriterFailed
                    )
                )
            }
        },
        false => Err(
            Error::from(
                InitiationError::ServiceNotSupported
            )
        )
    }
}

pub fn put(
    item: &CalendarItem,
    priv_key: &PKey<Private>,
    server_key: &PKey<Public>,
    name: &String
) -> Result<(), Error>
{
    let mut connection = connect_to_server(
        CALENDAR_SOCKET_ADDR, 
        priv_key, 
        server_key,
        name, 
        &HandlerType::Calendar,
        &Action::Put)?;

    match from_reader(&mut connection)? {
        true => {
            to_writer(&mut connection, item)?;

            match from_reader(&mut connection)? { 
                true => {
                    Ok(())
                },
                false => Err(
                    Error::from(
                        TransactionError::SerializeToWriterFailed
                    )
                )
            }
        },
        false => Err(
            Error::from(
                InitiationError::ServiceNotSupported
            )
        )
    }
}

pub fn get_range(
    range: &(DateTime<Utc>, DateTime<Utc>),
    priv_key: &PKey<Private>,
    name: &String,
    server_key: &PKey<Public>
) -> Result<Vec<CalendarItem>, Error>
{
    match connect_to_server(
        CALENDAR_SOCKET_ADDR, 
        priv_key, 
        server_key,
        name,
        &HandlerType::Calendar,
        &Action::Get) {
        Err(e) => Err(e),
        Ok(mut connection) => {
            to_writer(&mut connection, range)?;
            match from_reader(&mut connection) {
                Err(e) => Err(e),
                Ok(val) => Ok(val)
            }
        }
    }
}

