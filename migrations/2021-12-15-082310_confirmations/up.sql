-- Your SQL goes here
CREATE TABLE invitations (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(50) NOT NULL UNIQUE,
  expires_at TIMESTAMP NOT NULL
);