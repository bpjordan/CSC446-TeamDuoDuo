CREATE DATABASE users;

use users;

CREATE TABLE users (
    username VARCHAR(255) NOT NULL,
    password CHAR(96) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    role VARCHAR(225) NOT NULL,
    session  CHAR(32),
    PRIMARY KEY (username)
);

CREATE TABLE pokemon (
    name VARCHAR(225) NOT NULL,
    type VARCHAR(225) NOT NULL,
    trainer VARCHAR(225) NOT NULL,
    PRIMARY KEY (name, trainer),
    FOREIGN KEY (trainer) REFERENCES users(username)
);

INSERT INTO users(username, password, email, role)
VALUES(
    "brendan",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$ZvpuZy/7QJ9aZnx1cbwitw",
    "brendan@example.com",
    "Trainer"
),
(
    "bronson",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "bronson@example.com",
    "Trainer"
),
(
    "sydney",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "sydney@example.com",
    "Trainer"
),
(
    "promyse",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$pXvskGPmY0HSLy1dpADpFw",
    "promyse@example.com",
    "Trainer"
),
(
    "oak",
    "research",
    "oak@example.com",
    "Professor"
),
(
    "brock",
    "rock",
    "brock@example.com",
    "Gym Leader"
),
(
    "misty",
    "water",
    "misty@example.com",
    "Gym Leader"
);

INSERT INTO pokemon(name, type, trainer)
VALUES(
	"Electabuzz",
    "Electric",
    "promyse"
),
(
	"Litten",
    "Fire",
    "promyse"
),
(
    "Magmar",
    "Fire",
    "sydney"
),
(
    "Charizard",
    "Fire",
    "brendan"
),
(
    "Gyarados",
    "Water",
    "bronson"
);