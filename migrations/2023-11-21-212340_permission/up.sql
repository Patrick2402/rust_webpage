-- Your SQL goes here
CREATE TABLE permissions (
    "group_id" SERIAL PRIMARY KEY,
    "group_name" VARCHAR NOT NULL
);

-- Inserting default groups
INSERT INTO permissions (group_name) VALUES 
    ('admin'),
    ('normal_user');