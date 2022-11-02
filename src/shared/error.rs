
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
pub enum InitiationError
{
    #[fail(display = "Service not supported")]
    ServiceNotSupported
}


#[derive(Serialize, Deserialize, Debug, Fail)]
pub enum AuthenticationError
{
    #[fail(display = "'{}' failed to authenticate.", client)]
    ClientFailedAuthentication{
        client: String,
    },
    #[fail(display = "Failed to authenticate.")]
    AuthenticationFailed,

    #[fail(display = "Service '{}' not registered.", name)]
    ServiceNotRegistered{
        name: String
    },
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

#[derive(Serialize, Deserialize, Debug, Fail)]
pub enum TransactionError
{
    #[fail(display = "'{}' failed authorization for '{}' action for '{}' service.", client, action, service)]
    ClientFailedAuthorization{
        client: String,
        action: String,
        service: String,
    },
    #[fail(display = "Failed to authorize.")]
    AuthorizationFailed,
    #[fail(display = "Serialize to writer failed.")]
    SerializeToWriterFailed,
}