// @generated automatically by Diesel CLI.

diesel::table! {
    vocabulary (id) {
        id -> Integer,
        en -> Varchar,
        tr -> Text,
    }
}
