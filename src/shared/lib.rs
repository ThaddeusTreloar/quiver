use serde::{
    Serialize,
    Deserialize,
};
use interprocess::local_socket::LocalSocketStream;
use std::fmt;
use failure::Error;
use openssl::{
    nid::Nid,
    pkey::{
        Public,
        PKey
    }
};
use serde_json::Deserializer;
use diesel::{
    r2d2::{
        ConnectionManager,
        Pool,
    },
    sqlite::SqliteConnection
};
use std::hash::Hash;
use std::fs::File;
use std::io::{
    BufReader,
    prelude::Read
};

// Not in use at the moment todo: delete
#[derive(Debug)]
pub struct AuthorizedConnection {
    pub connection: LocalSocketStream,
    pub client: String,
    pub action: Action,
    pub service: HandlerType
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
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

pub fn get_server_public_key() -> Result<PKey<Public>, Error>
{
    let file = File::open(SERVER_PUBLIC_KEY_PATH)?;
    let mut reader = BufReader::new(file);
    let mut buff: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buff)?;
    Ok(PKey::public_key_from_pem(&buff)?)
}

pub const SERVICE_MANAGER_SOCKET_ADDR: &'static str = "/tmp/quiver.service_manager.sock";
pub const CALENDAR_SOCKET_ADDR: &'static str = "/tmp/quiver.calendar.sock";
pub const NFC_SOCKET_ADDR: &'static str = "/tmp/quiver.nfc.sock";
pub const VPN_SOCKET_ADDR: &'static str = "/tmp/quiver.vpn.sock";

pub const AUTH_KEY_ALGORITHM: &Nid = &Nid::SECP384R1;
pub const SERVER_PUBLIC_KEY_PATH: &'static str = "./run/public.pem";