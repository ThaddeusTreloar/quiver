use crate::shared::lib::{
    Permission,
    HandlerType,
    AUTH_KEY_ALGORITHM,
};
use crate::core::db::{
    schema,
    models::*,
};
use crate::core::db::schema::services::dsl::*;


use diesel::{
    prelude::*,
    sqlite::{
        SqliteConnection,
        SqliteQueryBuilder
    },
    sql_types::VarChar,
    insert_into,
    dsl::Eq,
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
use bincode::{
    serialize
};
use failure::Error;

sql_function!(fn services_service_name(x: VarChar) -> VarChar);

type WithName<'a> = Eq<services_service_name::HelperType<schema::services::name>, 
                    services_service_name::HelperType<&'a str>>;

fn with_name(searched_name: &str) -> WithName
{
    services_service_name(schema::services::name).eq(services_service_name(searched_name))
}



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

// Huh?
pub fn init_database(path: &str) -> Result<&str, &str>
{
    let db_connection = match SqliteConnection::establish(path)
    {
        Ok(db) => db,
        Err(e) => 
        {
            dbg!(e);
            return Err("{e}");
        },
    };

    let table_chk: SqliteQueryBuilder = SqliteQueryBuilder::new();
    Ok("")

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
        .filter(with_name(service_name.as_ref()))
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

    let pub_key = service_key.public_key();
    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let mut ctx = BigNumContext::new().unwrap();

    let record: ServiceAdd = ServiceAdd {
        name: service_name,
        perm: serialize(&service_perm)?,
        exclude: serialize(&service_exclude)?,
        pubkey: pub_key.to_bytes(
            &group,
            PointConversionForm::COMPRESSED, 
            &mut ctx
        )?,
    };
    
    insert_into(services).values(record).execute(connection)?;
    
    Ok(())
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