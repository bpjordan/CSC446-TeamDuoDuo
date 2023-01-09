CREATE DATABASE users;

use users;

CREATE TABLE users (
    username VARCHAR(255) NOT NULL,
    password CHAR(96) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    session  CHAR(32),
    PRIMARY KEY (username)
);

INSERT INTO users(username, password, email)
VALUES(
    "brendan",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$ZvpuZy/7QJ9aZnx1cbwitw",
    "brendan@example.com"
),
(
    "bronson",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "bronson@example.com"
),
(
    "sydney",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "sydney@example.com"
),
(
    "promyse",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$pXvskGPmY0HSLy1dpADpFw",
    "promyse@example.com"
);
