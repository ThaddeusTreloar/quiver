
use failure::Fail;
use serde::{
    Deserialize, Serialize
};

#[derive(Serialize, Deserialize, Debug, Fail)]
pub enum ConnectionActionError
{
    #[fail(display = "Expected '{}' got '{}' while waiting to '{}' item to '{}'.", expected, recieved, action, service)]
    UnexpectedActionError{
        expected: String,
        recieved: String,
        action: String,
        service: String,
    },
    #[fail(display = "'{}' not supported for '{}' service.", expected, service)]
    UnsupportedActionError{
        recieved: String,
        service: String,
    },
}