
diesel::table! {

    services {
        id -> Integer,
        name ->     Text,
        perm ->     Text,
        exclude ->  Text,
        pubkey ->      Binary,
    }
}