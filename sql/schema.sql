DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE users (
	id  BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (username)
);


CREATE TABLE document (
	id  BIGSERIAL PRIMARY KEY,
	filename    VARCHAR(200) NOT NULL,
    description TEXT,
	user_id     INTEGER UNIQUE NOT NULL REFERENCES users(id)
);
