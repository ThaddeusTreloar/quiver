use crate::shared::lib::{
    Permission,
    HandlerType,
    serialize_pubkey
};
use crate::core::db::{
    schema,
    models::*,
};
use crate::core::db::schema::services::dsl::*;

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
    insert_into,
    delete,
    update,
};
use openssl::{
    pkey::Public,
    ec::{
        EcKey,
        EcGroup,
        PointConversionForm
    },
    bn::{
        BigNumContext,
        BigNum
    },
};
use serde_json::{
    to_string,
};
use failure::Error;

pub fn establish_connection(path: &str) -> Result<SqliteConnection, &str>
{
    match SqliteConnection::establish(path)
    {
        Ok(c) => Ok(c),
        Err(e) =>
        {
            Err("Failed to establish db connection to {path} due to {e}")
        }
    }
}

pub fn get_all_services(
    connection: &mut SqliteConnection
) -> Result<Vec<super::models::ServiceQuery>, Error>
{
    let results = services
        .load::<ServiceQuery>(connection)?;
    
    Ok(results)
}

pub fn get_service(
    service_name: String,
    connection: &mut SqliteConnection
) -> Result<Vec<super::models::ServiceQuery>, Error>
{
    let results = services
        .filter(
            name.like(
                format!(
                    "%{}%",
                    service_name
                )
            )
        )
        .load::<ServiceQuery>(connection)?;
    
    Ok(results)
}

pub fn register_service(
    service_name: String, 
    service_perm: Box<Vec<Permission>>, 
    service_exclude: Box<Vec<HandlerType>>, 
    service_key: EcKey<Public>,
    connection: &mut SqliteConnection
) -> Result<(), Error>
{
    let record: ServiceAdd = ServiceAdd {
        name: service_name,
        perm: to_string(&service_perm)?,
        exclude: to_string(&service_exclude)?,
        pubkey: serialize_pubkey(service_key)?,
    };
    
    insert_into(services).values(record).execute(connection)?;
    
    Ok(())
}

pub fn update_service_permissions(
    service_name: String, 
    service_perm: &Vec<Permission>, 
    connection: &mut SqliteConnection
) -> Result<(), Error>
{
    match update(services
        .filter(
            name.like(
                format!(
                    "%{}%",
                    service_name
                )
            )
        )
    ).set(
        perm.eq(
            to_string(service_perm)?
        )
    ).execute(connection){
        Ok(val) => {
            if val != 1 {
                return Err(failure::format_err!("{}", 
                    diesel::result::Error::NotFound.to_string()
                ));
            } else {
                return Ok(());
            }
        },
        Err(e) => {
            return Err(failure::format_err!("{}", 
                e.to_string()
            ));
        },
    };
}

pub fn update_service_exclusions(
    service_name: String, 
    service_exclude: &Vec<HandlerType>,
    connection: &mut SqliteConnection
) -> Result<(), Error>
{
    match update(services
        .filter(
            name.like(
                format!(
                    "%{}%",
                    service_name
                )
            )
        )
    ).set(
        exclude.eq(
            to_string(service_exclude)?
        )
    ).execute(connection){
        Ok(val) => {
            if val != 1 {
                return Err(failure::format_err!("{}", 
                    diesel::result::Error::NotFound.to_string()
                ));
            } else {
                return Ok(());
            }
        },
        Err(e) => {
            return Err(failure::format_err!("{}", 
                e.to_string()
            ));
        },
    };
}

pub fn remove_service(
    service_name: String,
    connection: &mut SqliteConnection,
) -> Result<(), Error>
{
    match delete(
        services
        .filter(
            name.like(
                format!(
                    "%{}%",
                    service_name
                )
            )
        )
    ).execute(connection){
        Ok(val) => {
            if val != 1 {
                return Err(failure::format_err!("{}", 
                    diesel::result::Error::NotFound.to_string()
                ));
            } else {
                return Ok(());
            }
        },
        Err(e) => {
            return Err(failure::format_err!("{}", 
                e.to_string()
            ));
        },
    };
}

/*fn validate_service_permission(
    name: String, 
    perm: Permission) -> bool
{
    false
}*/


/*
The below record describes as service named 'someService' who had read/write permissions
to the calendar interface with access to all calendar services including the service
excluded service 'Other'. It excludes itself from having it's calendar data read by
other services without a service being explicitly granted permissions to it.
It is identified via a challenge using the key 'somePubKey' before it's actions are
authorized according to permissions listed in perm.

name        |   perm                                    |   exclude       |   key         |
someService |   Vec[Permission {                        |   Vec[Calendar] |   somePubKey  |
                    state: PermissionState::ReadWrite
                    service: HandlerType::Calendar,
                    include: Vec["All", "Other"]
                }]
*/