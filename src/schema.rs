// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Nullable<Integer>,
        original_url -> Text,
        shorted_url -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
