-- Your SQL goes here
CREATE TABLE users (
    id VARCHAR(255) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    is_disabled BOOLEAN NOT NULL DEFAULT FALSE
);
