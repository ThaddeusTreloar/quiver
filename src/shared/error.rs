
use failure::Fail;
use serde::{
    Deserialize, Serialize
};

#[derive(Serialize, Deserialize, Debug, Fail)]
pub enum SyncError
{
    #[fail(display = "Expected '{}' got '{}' while waiting to '{}' item to '{}'.", expected, recieved, action, service)]
    UnexpectedActionError{
        expected: &'static str,
        recieved: &'static str,
        action: &'static str,
        service: &'static str
    },
}
