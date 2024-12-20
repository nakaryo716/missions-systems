CREATE TABLE users (
    id              INTEGER AUTO_INCREMENT,
    user_id         VARCHAR(64) NOT NULL,
    user_name       VARCHAR(255) NOT NULL,
    email           VARCHAR(256) UNIQUE NOT NULL,
    password_hash   VARCHAR(256) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE INDEX (user_id)
);

CREATE TABLE daily_mission (
    id          INT AUTO_INCREMENT,
    user_id     VARCHAR(64) NOT NULL,
    mission_id  VARCHAR(64) NOT NULL,
    title       VARCHAR(255) NOT NULL,
    descriptions TEXT,
    is_complete BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE user_exp (
    id                  INTEGER AUTO_INCREMENT,
    user_id             VARCHAR(64) NOT NULL,
    experience_points   BIGINT DEFAULT 0,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);
