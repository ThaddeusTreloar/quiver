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
use crate::shared::lib::{
    Permission,
    HandlerType,
    AUTH_KEY_ALGORITHM,
};
use openssl::{
    pkey::Public,
    nid::Nid,
    ec::{
        EcKey,
        EcGroup
    },
    bn::{
        BigNumContext,
        BigNum
    },
};
use serde::{
    Serialize,
    Deserialize
};
use bincode::{
    serialize
};

mod schema {
    diesel::table! {

        services {
            id -> Integer,
            name ->     VarChar,
            perm ->     Binary,
            exclude ->  Binary,
            key_x ->      Binary,
            key_y ->      Binary,
        }
    }
}

use schema::services::dsl::*;

sql_function!(fn services_service_name(x: VarChar) -> VarChar);

type WithName<'a> = Eq<services_service_name::HelperType<schema::services::name>, 
                    services_service_name::HelperType<&'a str>>;

fn with_name(searched_name: &str) -> WithName
{
    services_service_name(schema::services::name).eq(services_service_name(searched_name))
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = schema::services)]
pub struct ServiceAdd<'a>
{
    name: &'a str,
    // Serialised Vec of Permission
    perm: Vec<u8>,
    // Serialised Vec of HandlerType
    exclude: Option<Vec<u8>>,
    // Serialised bytes from EcKey<Public>
    key_x: Vec<u8>,
    key_y: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[diesel(table_name = schema::services)]
pub struct ServiceQuery<'a>
{
    id: u32,
    name: &'a str,
    // Serialised Vec of Permission
    perm: Permission,
    // Serialised Vec of HandlerType
    exclude: Option<Vec<HandlerType>>,
    // Serialised bytes from EcKey<Public>
    key_x: Vec<u8>,
    key_y: Vec<u8>,
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





fn get_service<'a>(
    service_name: String,
    connection: SqliteConnection
) -> Result<&'a str, &'a str>
{


    let query: SqliteQueryBuilder = SqliteQueryBuilder::new();

    query.all().filter(with_name(service_name)).first(connection);

    Ok("")
}

fn register_service(
    service_name: String, 
    service_perm: Box<Vec<Permission>>, 
    service_exclude: Option<Box<Vec<HandlerType>>>, 
    service_key: EcKey<Public>,
    connection: SqliteConnection) -> Result<(), &'static str>
{

    let pub_key = service_key.public_key();
    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM).unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    let mut pub_x = BigNum::new().unwrap();
    let mut pub_y = BigNum::new().unwrap();
    pub_key.affine_coordinates_gfp(&group, &mut pub_x, &mut pub_y, &mut ctx);

    let value_x = pub_x.to_vec();
    let value_y = pub_y.to_vec();

    let record: ServiceAdd = ServiceAdd {
        name: service_name.as_ref(),
        perm: match serialize(&service_perm) {
            Ok(data) => data,
            Err(e) => {
                dbg!(e);
                return Err("Failed to serialise permissions due to: {e}");
            }
        },
        exclude: match service_exclude{
                Some(val) =>
                {
                    match serialize(&service_perm) {
                        Ok(data) => Some(data),
                        Err(e) => {
                            dbg!(e);
                            return Err("Failed to serialise exclusions due to: {e}");
                        }
                    }
                },
                None => None,
            },
        key_x: value_x,
        key_y: value_y,
    };
    
    match insert_into(services).values(record).execute(&mut connection)
    {

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

name        |   perm                                    |   exclude       |   key         |
someService |   Vec[Permission {                        |   Vec[Calendar] |   somePubKey  |
                    state: PermissionState::ReadWrite
                    service: HandlerType::Calendar,
                    include: Vec["All", "Other"]
                }]
*/