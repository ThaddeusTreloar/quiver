
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
    #[fail(display = "'{}' not supported for '{}' service.", recieved, service)]
    UnsupportedActionError{
        recieved: String,
        service: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Fail)]
pub enum AuthenticationError
{
    #[fail(display = "'{}' failed authentication for '{}' action for '{}' service.", client, action, service)]
    ClientFailedAuthentication{
        client: String,
        action: String,
        service: String,
    },
    #[fail(display = "Failed to authenticate.")]
    AuthenticationFailed,
}

#[derive(Serialize, Deserialize, Debug, Fail)]
pub enum AuthorizationError
{
    #[fail(display = "'{}' failed authorization for '{}' action for '{}' service.", client, action, service)]
    ClientFailedAuthorization{
        client: String,
        action: String,
        service: String,
    },
    #[fail(display = "Failed to authorize.")]
    AuthorizationFailed,
}