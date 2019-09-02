CREATE TABLE product (
  id SERIAL PRIMARY KEY,
  record_id VARCHAR NOT NULL,
  auth_id INTEGER,
  title VARCHAR NOT NULL,
  price INTEGER,
  latitude NUMERIC(7, 4),
  longitude NUMERIC(7, 4)
);
