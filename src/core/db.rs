use diesel::{
    sqlite
};
use crate::shared::lib::{
    Permission,
    HandlerType
};
use openssl::{
    pkey::Public,
    ec::{
        EcKey,
    }
};

fn someFunc() -> ()
{
    use openssl::bn::BigNumContext;
    use openssl::ec::*;
    use openssl::nid::Nid;
    use openssl::pkey::PKey;

    // get bytes from somewhere, i.e. this will not produce a valid key
    let public_key: Vec<u8> = vec![];

    // create an EcKey from the binary form of a EcPoint
    let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    let point = EcPoint::from_bytes(&group, &public_key, &mut ctx).unwrap();
    let key = EcKey::from_public_key(&group, &point);
}

/*fn create_schema()
{
    diesel::table! {
        services {
            name ->     VarChar,
            perm ->     Binary,
            exclude ->  Binary,
            key ->      Binary
        }
    }
}*/

fn register_service(
    name: String, 
    perm: Box<Vec<Permission>>, 
    exclude: Box<Vec<HandlerType>>, 
    key: EcKey<Public>) -> Result<(), &'static str>
{
    Ok(())
}


fn validate_service_permission(
    name: String, 
    perm: Permission) -> bool
{
    false
}


/*

The below record describes as service named 'someService' who had read/write permissions
to the calendar interface with access to all calendar services including the service
excluded service 'Other'. It excludes itself from having it's calendar data read by
other services without a service being explicitly granted permissions to it.
It is identified via a challenge using the key 'somePubKey' before it's actions are
authorized according to permissions listed in perm.

name        |   perm                                    |   exclude       |   key         |
someService |   Vec[Permission {                        |   Vec[Calendar] |   somePubKey  |
                    state: PermissionState::ReadWrite
                    service: HandlerType::Calendar,
                    include: Vec["All", "Other"]
                }]
*/