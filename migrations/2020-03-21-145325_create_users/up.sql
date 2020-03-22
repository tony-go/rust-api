-- Your SQL goes here
CREATE TABLE users
(
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  email VARCHAR UNIQUE NOT NULL,
  pseudo VARCHAR UNIQUE NOT NULL,
  password VARCHAR NOT NULL,
  activated BOOL NOT NULL
)