table! {
    links (id) {
        id -> Integer,
        name -> Varchar,
        link -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        salt -> Varchar,
        password -> Varchar,
        admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    users,
);
