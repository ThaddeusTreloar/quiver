// Internal
use crate::connection::connect_to_server;
use crate::shared::{
    calendar::CalendarItem,
    lib::{
        Action,
        HandlerType,
        CALENDAR_SOCKET_ADDR,
        from_reader
    },
    //error::*
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
