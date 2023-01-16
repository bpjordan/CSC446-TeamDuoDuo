CREATE DATABASE gym;

use gym;

CREATE TABLE users (
    username VARCHAR(255) NOT NULL,
    password CHAR(96) NOT NULL,
    email    VARCHAR(255) NOT NULL,
    session  CHAR(32),
    role     ENUM ('trainer', 'professor', 'leader'),
    PRIMARY KEY (username)
);

CREATE TABLE pokemon (
    name    VARCHAR(225) NOT NULL,
    type    VARCHAR(225) NOT NULL,
    trainer VARCHAR(225) NOT NULL,
    PRIMARY KEY (name, trainer),
    FOREIGN KEY (trainer) REFERENCES users(username)
);

CREATE TABLE access_log (
    timestamp           TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    username_provided   VARCHAR(255) NOT NULL,
    password_provided   CHAR(96) NOT NULL,
    success             BOOLEAN NOT NULL,
    user_found          VARCHAR(255),
    session_len         INT,
    PRIMARY KEY (timestamp, username_provided),
    FOREIGN KEY (user_found) REFERENCES users(username)
);

INSERT INTO users(username, password, email, role)
VALUES(
    "brendan",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$ZvpuZy/7QJ9aZnx1cbwitw",
    "brendan@example.com",
    "trainer"
),
(
    "bronson",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "bronson@example.com",
    "trainer"
),
(
    "sydney",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "sydney@example.com",
    "trainer"
),
(
    "promyse",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$pXvskGPmY0HSLy1dpADpFw",
    "promyse@example.com",
    "trainer"
),
(
    "oak",
    "$argon2id$v=19$m=4096,t=3,p=1$SXRKeHRpOUNpU3QzMzdhWg$ruktCWzheUfFpfSDKPkyjQ", /* research */
    "oak@example.com",
    "professor"
),
(
    "brock",
    "$argon2id$v=19$m=4096,t=3,p=1$ZXhWaWM0TVRhOUhNakE4Nw$etnc2G9SKIvShFCYv6xlHg", /* rock */
    "brock@example.com",
    "leader"
),
(
    "misty",
    "$argon2id$v=19$m=4096,t=3,p=1$eXlNdlg3eG1wWEI2U0czcg$xmmnQmuBUsPdfGQrGwkM9Q", /* water */
    "misty@example.com",
    "leader"
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