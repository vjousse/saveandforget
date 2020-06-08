-- Your SQL goes here

CREATE TABLE documents (
  id  BIGSERIAL PRIMARY KEY,
  filename    VARCHAR(200) NOT NULL,
  description TEXT
--  user_id     INTEGER UNIQUE NOT NULL REFERENCES users(id)
);
