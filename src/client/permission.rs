use crate::shared::lib::{
    HandlerType,
    PubKey
};
use openssl;

// Is it worth using a public key, or just a fingerprint
fn register_service(
    services: Box<[HandlerType]>, 
    name: String, 
    key_type: PubKey
) -> Result<(), &'static str>
{
    Ok(())
}

fn request_permission(
    service: Box<HandlerType>, 
    key: PubKey
) -> Result<(), &'static str>
{
    Ok(())
}

fn check_permission(
    service: Box<HandlerType>, 
    key: PubKey
) -> Result<(), &'static str>
{
    Ok(())
}