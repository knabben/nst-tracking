CREATE TABLE IF NOT EXISTS auth (
    public_key            varchar PRIMARY KEY,
    username              varchar,
    hashed_password       varchar,
    encrypted_private_key varchar
)
