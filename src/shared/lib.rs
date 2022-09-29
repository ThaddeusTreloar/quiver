use serde::{
    Serialize,
    Deserialize,
};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum PermissionState
{
    Read,
    Write,
    ReadWrite
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    state: PermissionState,
    service: HandlerType,
    include: Vec<String>
}

// Will add key types to enum wrapper
#[derive(Debug, Serialize, Deserialize)]
pub enum PubKey {
    Ecc(()), 
    Ntru(()), 
    Rsa(()),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Null,
    Get,
    Put,
    Pop,
    Edit,
    Ready
}

impl fmt::Display for Action
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let action_str: &'static str = match *self
        {
            Action::Null => "null",
            Action::Get => "get",
            Action::Put => "put",
            Action::Pop => "pop",
            Action::Edit => "edit",
            Action::Ready => "ready",
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

// For constructing function call interfaces into a thirdpary service
#[derive(Debug, Serialize, Deserialize)]
pub enum FunctionArgTypes
{
    UInt(u64),
    IInt(i64),
    Bool(bool),

}

pub const CALENDAR: &'static str = "/tmp/quiver.calendar.sock";
pub const NFC: &'static str = "/tmp/quiver.nfc.sock";
pub const VPN: &'static str = "/tmp/quiver.vpn.sock";