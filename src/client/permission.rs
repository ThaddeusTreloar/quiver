// Internal
use super::lib::connect_authenticate_authorize;
use crate::shared::lib::{
    HandlerType,
    PermissionState,
    SERVICE_MANAGER_SOCKET_ADDR,
    Action,
    from_reader
};
//External
use failure::Error;
use openssl::{
    pkey::{
        Private,
        PKey
    },
};
use serde_json::{
    to_writer,
};

pub fn transaction(
    service: &HandlerType,
    reference: Option<&PermissionState>,
    item: Option<&PermissionState>,
    priv_key: &PKey<Private>,
    name: &String
) -> Result<PermissionState, Error>
{
    match connect_authenticate_authorize(
        SERVICE_MANAGER_SOCKET_ADDR, 
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