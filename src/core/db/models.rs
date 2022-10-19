use crate::shared::lib::{
    Permission,
    HandlerType,
};

use diesel::{
    prelude::*,
};
use serde::{
    Serialize,
    Deserialize
};
use crate::core::db::schema::services;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = services)]
pub struct ServiceAdd
{
    pub name: String,
    // Serialised Vec of Permission
    pub perm: Vec<u8>,
    // Serialised Vec of HandlerType
    pub exclude: Vec<u8>,
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
    pub perm: Vec<u8>,
    // Serialised Vec of HandlerType
    pub exclude: Vec<u8>,
    // Serialised bytes from EcKey<Public>
    pub pubkey: Vec<u8>,
}