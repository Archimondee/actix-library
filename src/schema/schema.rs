// @generated automatically by Diesel CLI.

diesel::table! {
    auth (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        auth_id -> Integer,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
    }
}

diesel::joinable!(users -> auth (auth_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth,
    users,
);
