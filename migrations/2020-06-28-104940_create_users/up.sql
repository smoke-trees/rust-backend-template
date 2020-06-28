CREATE TABLE IF NOT EXISTS users
(
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
    password     TEXT NOT NULL,
    access_level TEXT NOT NULL
);

ALTER TABLE users ADD CONSTRAINT constraints_user_text_field CHECK ( id <> '' AND
                                                                     name <> '' AND
                                                                     password <> '' AND
                                                                     access_level <> '');