CREATE DATABASE gym;

use gym;

SET @example_mfa_secret = FROM_BASE64("gzBsRiEnc3Kwc/26S3gklyz5M4UUOztqbO4pbhtgDi4=");

CREATE TABLE users (
    username    VARCHAR(255) NOT NULL,
    password    CHAR(96) NOT NULL,
    email       VARCHAR(255) NOT NULL,
    session     CHAR(32),
    role        ENUM ('trainer', 'professor', 'leader'),
    mfa_secret  BINARY(32) NOT NULL,
    sprite      VARCHAR(255) NOT NULL,
    image       VARCHAR(255) NOT NULL,
    PRIMARY KEY (username)
);

CREATE TABLE pokemon (
    name    VARCHAR(225) NOT NULL,
    type    VARCHAR(225) NOT NULL,
    trainer VARCHAR(225) NOT NULL,
    sprite  VARCHAR(255) NOT NULL,
    image   VARCHAR(255) NOT NULL,
    PRIMARY KEY (name, trainer),
    FOREIGN KEY (trainer) REFERENCES users(username)
);

CREATE TABLE access_log (
    id                  BIGINT NOT NULL AUTO_INCREMENT,
    timestamp           DATETIME DEFAULT CURRENT_TIMESTAMP,
    username_provided   VARCHAR(255) NOT NULL,
    password_provided   CHAR(96) NOT NULL,
    success             BOOLEAN NOT NULL,
    mfa_success         BOOLEAN,
    user_found          VARCHAR(255),
    session_len         INT UNSIGNED,
    error               VARCHAR(255),
    PRIMARY KEY (id),
    FOREIGN KEY (user_found) REFERENCES users(username)
);

CREATE TABLE comment (
    id                  BIGINT NOT NULL AUTO_INCREMENT,
    timestamp           DATETIME DEFAULT CURRENT_TIMESTAMP,
    content             TEXT,
    PRIMARY KEY (id)
);

INSERT INTO users(username, password, email, role, mfa_secret, sprite, image)
VALUES(
    "brendan",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$ZvpuZy/7QJ9aZnx1cbwitw",
    "brendan@example.com",
    'trainer',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/c/c9/RFVF_Rosso.png",
    "https://media.pokemoncentral.it/wiki/b/be/RossoRFVF.png"
),
(
    "bronson",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$puIZlAash2vZjfDOXuVfFA",
    "bronson@example.com",
    'trainer',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/c/c9/RFVF_Rosso.png",
    "https://media.pokemoncentral.it/wiki/b/be/RossoRFVF.png"
),
(
    "sydney",
    "$argon2i$v=19$m=4096,t=3,p=1$MFhBejk5ODBSOUVZWTFlWg$/zMh5Pqht+XI4lNbhydSzQ",
    "sydney@example.com",
    'trainer',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/a/af/RFVF_Leaf.png",
    "https://media.pokemoncentral.it/wiki/3/3b/Leaf_RFVF.png"
),
(
    "promyse",
    "$argon2id$v=19$m=4096,t=3,p=1$UmRxb2ZHQmV2cDBQUG1odw$pXvskGPmY0HSLy1dpADpFw",
    "promyse@example.com",
    'trainer',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/a/af/RFVF_Leaf.png",
    "https://media.pokemoncentral.it/wiki/3/3b/Leaf_RFVF.png"
),
(
    "oak",
    "$argon2id$v=19$m=4096,t=3,p=1$SXRKeHRpOUNpU3QzMzdhWg$ruktCWzheUfFpfSDKPkyjQ", /* research */
    "oak@example.com",
    'professor',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/4/49/RFVF_Oak.png",
    "https://media.pokemoncentral.it/wiki/d/d0/OakLGPE.png"
),
(
    "brock",
    "$argon2id$v=19$m=4096,t=3,p=1$ZXhWaWM0TVRhOUhNakE4Nw$etnc2G9SKIvShFCYv6xlHg", /* rock */
    "brock@example.com",
    'leader',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/9/92/RFVF_Brock.png",
    "https://media.pokemoncentral.it/wiki/4/4b/BrockLGPE.png"
),
(
    "misty",
    "$argon2id$v=19$m=4096,t=3,p=1$eXlNdlg3eG1wWEI2U0czcg$xmmnQmuBUsPdfGQrGwkM9Q", /* water */
    "misty@example.com",
    'leader',
    @example_mfa_secret,
    "https://media.pokemoncentral.it/wiki/0/0e/RFVF_Misty.png",
    "https://media.pokemoncentral.it/wiki/3/3d/MistyLGPE.png"
);

INSERT INTO pokemon(name, type, trainer, sprite, image)
VALUES(
	"electabuzz",
    "electric",
    "promyse",
    "https://img.pokemondb.net/sprites/black-white/normal/electabuzz.png",
    "https://img.pokemondb.net/artwork/large/electabuzz.jpg"
),
(
	"chimchar",
    "fire",
    "promyse",
    "https://img.pokemondb.net/sprites/black-white/normal/chimchar.png",
    "https://img.pokemondb.net/artwork/large/chimchar.jpg"
),
(
    "magmar",
    "fire",
    "sydney",
    "https://img.pokemondb.net/sprites/black-white/normal/magmar.png",
    "https://img.pokemondb.net/artwork/large/magmar.jpg"
),
(
    "charizard",
    "fire",
    "brendan",
    "https://img.pokemondb.net/sprites/black-white/normal/charizard.png",
    "https://img.pokemondb.net/artwork/large/charizard.jpg"
),
(
    "gyarados",
    "water",
    "bronson",
    "https://img.pokemondb.net/sprites/black-white/normal/gyarados-f.png",
    "https://img.pokemondb.net/artwork/avif/gyarados.avif"
),
(
    "charmander",
    "fire",
    "oak",
    "https://img.pokemondb.net/sprites/black-white/normal/charmander.png",
    "https://img.pokemondb.net/artwork/avif/charmander.avif"
),
(
    "bulbasaur",
    "grass",
    "oak",
    "https://img.pokemondb.net/sprites/black-white/normal/bulbasaur.png",
    "https://img.pokemondb.net/artwork/avif/bulbasaur.avif"
),
(
    "squirtle",
    "water",
    "oak",
    "https://img.pokemondb.net/sprites/black-white/normal/squirtle.png",
    "https://img.pokemondb.net/artwork/avif/squirtle.avif"
),
(
    "geodude",
    "rock",
    "brock",
    "https://img.pokemondb.net/sprites/black-white/normal/geodude.png",
    "https://img.pokemondb.net/artwork/avif/geodude.avif"
),
(
    "onix",
    "rock",
    "brock",
    "https://img.pokemondb.net/sprites/black-white/normal/onix.png",
    "https://img.pokemondb.net/artwork/avif/onix.avif"
),
(
    "staryu",
    "water",
    "misty",
    "https://img.pokemondb.net/sprites/black-white/normal/staryu.png",
    "https://img.pokemondb.net/artwork/avif/staryu.avif"
),
(
    "starmie",
    "water",
    "misty",
    "https://img.pokemondb.net/sprites/black-white/normal/starmie.png",
    "https://img.pokemondb.net/artwork/avif/starmie.avif"
);