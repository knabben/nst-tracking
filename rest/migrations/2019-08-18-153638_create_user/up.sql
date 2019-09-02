CREATE TABLE IF NOT EXISTS auth (
  id BIGSERIAL,
  public_key            varchar PRIMARY KEY,
  username              varchar,
  hashed_password       varchar,
  encrypted_private_key varchar
);
