// @generated automatically by Diesel CLI.

diesel::table! {
    services (id) {
        id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
        perm -> Text,
        exclude -> Text,
        pubkey -> Binary,
    }
}
