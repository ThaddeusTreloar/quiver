pub mod models;
pub mod schema;

use crate::shared::lib::{
    Permission,
    HandlerType,
};
use crate::db::{
    models::*,
};
use crate::db::schema::services::dsl::*;

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
    insert_into,
    delete,
    update,
    r2d2::{
        Pool,
        ConnectionManager
    }
};
use serde_json::{
    to_string,
};
use failure::Error;





pub fn get_all_services(
    connection: &Pool<ConnectionManager<SqliteConnection>>
) -> Result<Vec<models::ServiceQuery>, Error>
{
    let results = services
        .load::<ServiceQuery>(&mut connection.get()?)?;
    
    Ok(results)
}

pub fn search_service(
    service_name: &String,
    connection: &Pool<ConnectionManager<SqliteConnection>>
) -> Result<Vec<models::ServiceQuery>, Error>
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
        .load::<ServiceQuery>(&mut connection.get()?)?;
    
    Ok(results)
}

pub fn get_service(
    service_name: &String,
    connection: &Pool<ConnectionManager<SqliteConnection>>
) -> Result<Vec<models::ServiceQuery>, Error>
{
    // Double check this only returns exact, whole matches. todo
    let results = services
        .filter(
            name.eq(
                format!(
                    "%{}%",
                    service_name
                )
            )
        )
        .load::<ServiceQuery>(&mut connection.get()?)?;
    
    Ok(results)
}

pub fn register_service(
    service_name: String, 
    service_address: String,
    service_perm: Box<Vec<Permission>>, 
    service_exclude: Box<Vec<HandlerType>>, 
    service_key: Vec<u8>,
    connection: &Pool<ConnectionManager<SqliteConnection>>
) -> Result<(), Error>
{
    let record: ServiceAdd = ServiceAdd {
        name: service_name,
        address: service_address,
        perm: to_string(&service_perm)?,
        exclude: to_string(&service_exclude)?,
        pubkey: service_key,
    };
    
    insert_into(services).values(record).execute(&mut connection.get()?)?;
    
    Ok(())
}

pub fn update_service_permissions(
    service_name: String, 
    service_perm: &Vec<Permission>, 
    connection: &Pool<ConnectionManager<SqliteConnection>>
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
    ).execute(&mut connection.get()?){
        Ok(val) => {
            if val != 1 {
                Err(failure::format_err!("{}", 
                    diesel::result::Error::NotFound.to_string()
                ))
            } else {
                Ok(())
            }
        },
        Err(e) => {
            Err(failure::format_err!("{}", 
                e.to_string()
            ))
        },
    }
}

pub fn update_service_exclusions(
    service_name: String, 
    service_exclude: &Vec<HandlerType>,
    connection: &Pool<ConnectionManager<SqliteConnection>>
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
    ).execute(&mut connection.get()?){
        Ok(val) => {
            if val != 1 {
                Err(failure::format_err!("{}", 
                    diesel::result::Error::NotFound.to_string()
                ))
            } else {
                Ok(())
            }
        },
        Err(e) => {
            Err(failure::format_err!("{}", 
                e.to_string()
            ))
        },
    }
}

pub fn remove_service(
    service_name: String,
    connection: &Pool<ConnectionManager<SqliteConnection>>,
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
    ).execute(&mut connection.get()?){
        Ok(val) => {
            if val != 1 {
                Err(failure::format_err!("{}", 
                    diesel::result::Error::NotFound.to_string()
                ))
            } else {
                Ok(())
            }
        },
        Err(e) => {
            Err(failure::format_err!("{}", 
                e.to_string()
            ))
        },
    }
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

name        | Address         | perm                                    |   exclude       |   key         |
someService | Address(String) | Vec[Permission {                        |   Vec[Calendar] |   somePubKey  |
                                    state: PermissionState::ReadWrite
                                    service: HandlerType::Calendar,
                                    include: Vec["All", "Other"]
                                }]
*/