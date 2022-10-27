// @generated automatically by Diesel CLI.

diesel::table! {
    services (id) {
        id -> Nullable<Integer>,
        name -> Text,
        perm -> Text,
        exclude -> Text,
        pubkey -> Binary,
    }
}
