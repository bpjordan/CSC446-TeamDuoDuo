CREATE DATABASE users;

use users;

CREATE TABLE users (
    username VARCHAR(255) NOT NULL,
    password CHAR(96) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    PRIMARY KEY (username)
);

INSERT INTO users
VALUES(
    "user",
    "$argon2id$v=19$m=4096,t=3,p=1$JdzJMju1ONSYs/MEyLw7Pg$ZazGKYrgaFX5OXwmnMrQsrXnA38lQ7CFzCuv05rpGhM",
    "user@example.com"
);
