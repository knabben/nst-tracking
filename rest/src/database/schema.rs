table! {
    auth (public_key) {
        id -> Int8,
        public_key -> Varchar,
        username -> Varchar,
        hashed_password -> Varchar,
        encrypted_private_key -> Varchar,
    }
}

table! {
    bid (id) {
        id -> Int8,
        product_id -> Int8,
        price -> Int8,
    }
}

table! {
    product (id) {
        id -> Int8,
        record_id -> Varchar,
        auth_id -> Int8,
        title -> Varchar,
        price -> Int8,
        latitude -> Int8,
        longitude -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    auth,
    bid,
    product,
);
