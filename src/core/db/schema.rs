
diesel::table! {

    services {
        id -> Integer,
        name ->     VarChar,
        perm ->     Binary,
        exclude ->  Binary,
        pubkey ->      Binary,
    }
}