// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
        id -> Int4,
        name -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
