-- up.sql
CREATE TABLE todos (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
-- Your SQL goes here
