CREATE TABLE user_groups (
    "user_id" INT REFERENCES users(id),
    "group_id" INT REFERENCES permissions(group_id), -- Corrected reference to 'permissions' table
    PRIMARY KEY (user_id, group_id)
);

ALTER TABLE user_groups
DROP CONSTRAINT user_groups_user_id_fkey,
ADD CONSTRAINT user_groups_user_id_fkey
FOREIGN KEY (user_id)
REFERENCES users (id)
ON UPDATE CASCADE
ON DELETE CASCADE;
