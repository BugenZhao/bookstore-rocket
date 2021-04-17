table! {
    books (id) {
        id -> Integer,
        isbn -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        author -> Text,
        price -> Double,
        description -> Text,
        inventory -> Integer,
        image -> Text,
    }
}

table! {
    user_auths (user_id) {
        user_id -> Integer,
        username -> Text,
        password -> Text,
        user_type -> Integer,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        nickname -> Text,
        name -> Nullable<Text>,
        tel -> Nullable<Text>,
        address -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    user_auths,
    users,
);
