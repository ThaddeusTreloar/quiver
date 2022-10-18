fn someFunc() -> ()
{
    use openssl::bn::BigNumContext;
    use openssl::ec::*;

    // get bytes from somewhere, i.e. this will not produce a valid key
    let public_key: Vec<u8> = vec![];

    // create an EcKey from the binary form of a EcPoint
    let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    let point = EcPoint::from_bytes(&group, &public_key, &mut ctx).unwrap();
    let key = EcKey::from_public_key(&group, &point);
}s