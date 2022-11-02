// Internal
use crate::{
    shared::{
        calendar::CalendarItem,
        lib::{
            Action,
            HandlerType,
            CALENDAR_SOCKET_ADDR,
            from_reader
        },
        error::{
            TransactionError,
            InitiationError
        }
    },
    connection::connect_to_server,
};

// External
use serde_json::{
    to_writer,
    from_slice
};
use openssl::{
    pkey::{
        PKey,
        Private,
        Public
    }
};
use failure::Error;
use std::io::Read;

pub fn put_series(
    item: &Vec<CalendarItem>,
    priv_key: &PKey<Private>,
    server_key: &PKey<Public>,
    name: &String
) -> Result<(), Error>
{
    match connect_to_server(
        CALENDAR_SOCKET_ADDR, 
        priv_key, 
        server_key,
        name,
        &HandlerType::Calendar,
        &Action::Put) {
            Err(e) => Err(e),
            Ok(mut connection) => {
                to_writer(&mut connection, item)?;
                let mut buff: Vec<u8> = Vec::new();
                connection.read(&mut buff)?;
                dbg!(&buff);
                match from_slice(&buff) {
                    Ok(val) => match val {
                        Ok::<(), TransactionError>(_) => Ok(()),
                        Err::<(), TransactionError>(e) => {
                            Err(Error::from(e))
                        }
                    },
                    Err(e) => Err(Error::from(e))
                }
            }
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
