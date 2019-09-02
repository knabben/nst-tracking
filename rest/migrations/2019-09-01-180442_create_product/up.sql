CREATE TABLE product (
  id BIGSERIAL PRIMARY KEY,
  record_id VARCHAR NOT NULL,
  auth_id BIGINT,
  title VARCHAR NOT NULL,
  price BIGINT,
  latitude BIGINT,
  longitude BIGINT
);
