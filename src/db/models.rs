use diesel::{
    prelude::*,
};
use serde::{
    Serialize,
    Deserialize
};
use crate::db::schema::services;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = services)]
pub struct ServiceAdd
{
    pub name: String,
    // Serialised Vec of Permission
    pub perm: String,
    // Serialised Vec of HandlerType
    pub exclude: String,
    // Serialised bytes from EcKey<Public>
    pub pubkey: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[diesel(table_name = services)]
pub struct ServiceQuery
{
    pub id: i32,
    pub name: String,
    // Serialised Vec of Permission
    pub perm: String,
    // Serialised Vec of HandlerType
    pub exclude: String,
    // Serialised bytes from EcKey<Public>
    pub pubkey: Vec<u8>,
}