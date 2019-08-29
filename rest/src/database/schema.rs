table! {
    auth (public_key) {
        public_key -> Varchar,
        username -> Varchar,
        hashed_password -> Varchar,
        encrypted_private_key -> Varchar,
    }
}
