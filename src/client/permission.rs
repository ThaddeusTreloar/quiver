// Internal
use crate::{
    connection::connect_to_server,
    shared::lib::{
        HandlerType,
        PermissionState,
        SERVICE_MANAGER_SOCKET_ADDR,
        Action,
        from_reader
    }
};
//External
use failure::Error;
use openssl::{
    pkey::{
        Private,
        Public,
        PKey
    },
};
use serde_json::{
    to_writer,
};

pub fn permission_transaction(
    reference: Option<&(String, HandlerType)>,
    item: Option<&PermissionState>,
    priv_key: &PKey<Private>,
    server_key: &PKey<Public>,
    name: &String
) -> Result<PermissionState, Error>
{
    match connect_to_server(
        SERVICE_MANAGER_SOCKET_ADDR, 
        priv_key,
        server_key,
        name,
        &HandlerType::Calendar,
        match reference {
            Some(_v) => &Action::Edit,
            None => {
                match item {
                    Some(_v) => &Action::Put,
                    None => &Action::Get
                }
            }
        }
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