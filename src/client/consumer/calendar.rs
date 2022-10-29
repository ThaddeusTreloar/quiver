// Internal
use super::super::lib::connect_authenticate_authorize;
use crate::shared::{
    calendar::CalendarItem,
    lib::{
        Action,
        CALENDAR_SOCKET_ADDR,
        HandlerType,
        from_reader
    },
    error::{
        ConnectionActionError,
        TransactionError
    }
};

// External
use std::{
    io::{
        prelude::*, 
    }
};
use log::{
    info,
    error,
    warn
};
use interprocess::local_socket::{
    LocalSocketStream
};
use serde_json::{
    Value,
    from_str,
    to_writer,
    Deserializer,
    Serializer
};
use openssl::{
    pkey::{
        PKey,
        Private
    }
};
use chrono::{
    DateTime,
    offset::Utc
};
use serde::Deserialize;
use failure::Error;

pub fn get_range(
    range: &(DateTime<Utc>, DateTime<Utc>),
    priv_key: &PKey<Private>,
    name: &String
) -> Result<(), Error>
{
    match connect_authenticate_authorize(
        CALENDAR_SOCKET_ADDR, 
        priv_key, 
        name, 
        &Action::Get, 
        &HandlerType::Calendar) {
            Err(e) => Err(e),
            Ok(mut connection) => {
                to_writer(&mut connection, &Action::Get)?;
                to_writer(&mut connection, range)?;
                match from_reader(&mut connection) {
                    Ok(val) => match val {
                        Ok::<(), TransactionError>(_) => Ok(()),
                        Err::<(), TransactionError>(e) => {
                            Err(Error::from(e))
                        }
                    },
                    Err(e) => Err(e)
                }
            }
    }
}
