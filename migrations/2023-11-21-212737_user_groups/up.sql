CREATE TABLE user_groups (
    "user_id" INT REFERENCES users(id),
    "group_id" INT REFERENCES permissions(group_id), -- Corrected reference to 'permissions' table
    PRIMARY KEY (user_id, group_id)
);