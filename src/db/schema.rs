
diesel::table! {
    services {
        id -> Integer,
        name ->     Text,
        address -> Text,
        perm ->     Text,
        exclude ->  Text,
        pubkey ->      Binary,
    }
}