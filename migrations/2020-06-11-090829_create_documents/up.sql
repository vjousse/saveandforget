-- Your SQL goes here

CREATE TABLE documents (
  id  BIGSERIAL PRIMARY KEY,
  filename    VARCHAR(200) NOT NULL,
  description TEXT,
  created_at TIMESTAMP NOT NULL,
  user_id     BIGINT UNIQUE NOT NULL REFERENCES users(id)
);
