// Internal
use super::lib::connect_authenticate_authorize;
use crate::shared::lib::{
    HandlerType,
    SERVICE_MANAGER_SOCKET_ADDR,
    Action,
    from_reader
};
//External
use failure::Error;
use serde::{
    Serialize,
    Deserialize
};
use openssl::{
    pkey::{
        Private,
        PKey
    },
};
use serde_json::{
    to_writer,
};

pub fn transaction<'a, T>(
    address: &'static str,
    service: &HandlerType,
    reference: Option<&T>,
    item: Option<&T>,
    priv_key: &PKey<Private>,
    name: &String
) -> Result<T, Error>
where
    T: Serialize,
    T: Deserialize<'a>
{
    match connect_authenticate_authorize(
        address, 
        priv_key,
        name,
        match reference {
            Some(_v) => &Action::Edit,
            None => {
                match item {
                    Some(_v) => &Action::Put,
                    None => &Action::Get
                }
            }
        },
        service
    ) {
        Err(e) => Err(e),
        Ok(mut connection) => {
            if let Some(reference) = reference {
                to_writer(&mut connection, reference)?;
            }
            if let Some(item) = item {
                to_writer(&mut connection, item)?;
            };

            from_reader(&mut connection)
        }
    }
}