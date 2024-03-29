// @generated automatically by Diesel CLI.

diesel::table! {
    shopping_items (id) {
        id -> Int4,
        name -> Varchar,
        quantity -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    shopping_items,
    users,
);
