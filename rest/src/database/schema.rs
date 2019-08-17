table! {
    auth (public_key) {
        public_key -> Varchar,
        hashed_password -> Varchar,
        encrypted_private_key -> Varchar,
    }
}