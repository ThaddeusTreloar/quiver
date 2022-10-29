use serde::{
    Serialize,
    Deserialize,
};
use interprocess::local_socket::LocalSocketStream;
use std::fmt;
use failure::Error;
use openssl::{
    pkey::Public,
    ec::{
        EcKey,
        EcGroup,
        PointConversionForm,
        EcPoint
    },
    bn::BigNumContext,
    nid::Nid
};
use serde_json::Deserializer;
use diesel::{
    r2d2::{
        ConnectionManager,
        Pool,
    },
    sqlite::SqliteConnection
};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PermissionState
{
    Read,
    Write,
    ReadWrite
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub state: PermissionState,
    pub service: HandlerType,
    pub include: Vec<String>
}

impl PartialEq<HandlerType> for Permission
{
    fn eq(&self, other: &HandlerType) -> bool {
        self.service.eq(other)
    }
}

// Will add key types to enum wrapper
#[derive(Debug, Serialize, Deserialize)]
pub enum PubKey {
    Ecc(()),
    Ntru(()), 
    Rsa(()),
}

pub fn serialize_pubkey(key: EcKey<Public>) -> Result<Vec<u8>, Error>
{
    let pub_key = key.public_key();
    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM)?;
    let mut ctx = BigNumContext::new()?;
    let bytes = pub_key.to_bytes(
        &group,
        PointConversionForm::COMPRESSED, 
        &mut ctx
    )?;
    Ok(bytes)
}

pub fn deserialize_pubkey(bytes: Vec<u8>) -> Result<EcKey<Public>, Error>
{
    let group = EcGroup::from_curve_name(*AUTH_KEY_ALGORITHM)?;
    let mut ctx = BigNumContext::new()?;
    let pubkey = EcPoint::from_bytes(
        &group,
        &bytes,
        &mut ctx
    )?;
    Ok(EcKey::from_public_key(
        &group,
        &pubkey,
    )?)
}

pub fn build_connection_pool(path: String) -> Result<Pool<ConnectionManager<SqliteConnection>>, Error>
{
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(
            ConnectionManager::<SqliteConnection>::new(path)
        )?;

    Ok(pool)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Get,
    Put,
    Pop,
    Edit,
}

impl fmt::Display for Action
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let action_str: &'static str = match *self
        {
            Action::Get => "get",
            Action::Put => "put",
            Action::Pop => "pop",
            Action::Edit => "edit",
        };
        write!(f, "{action_str}")?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum HandlerType {
    All,
    Calendar,
    Vpn,
    Nfc
}

impl HandlerType {
    pub fn all_handlers() -> Vec<HandlerType>
    {
        vec![HandlerType::Calendar, HandlerType::Nfc, HandlerType::Vpn]
    }
}

// For constructing function call interfaces into a thirdpary service
#[derive(Debug, Serialize, Deserialize)]
pub enum FunctionArgTypes
{
    UInt(u64),
    IInt(i64),
    Bool(bool),

}

pub fn from_reader<'a, T>(connection: &mut LocalSocketStream) -> Result<T, Error>
where
    T: Deserialize<'a>
{
    let mut deser = Deserializer::from_reader(connection);
    let res = T::deserialize(&mut deser)?;
    Ok(res)
}

pub const SERVICE_MANAGER_SOCKET_ADDR: &'static str = "/tmp/quiver.service_manager.sock";
pub const CALENDAR_SOCKET_ADDR: &'static str = "/tmp/quiver.calendar.sock";
pub const NFC_SOCKET_ADDR: &'static str = "/tmp/quiver.nfc.sock";
pub const VPN_SOCKET_ADDR: &'static str = "/tmp/quiver.vpn.sock";

pub const AUTH_KEY_ALGORITHM: &Nid = &Nid::SECP384R1;