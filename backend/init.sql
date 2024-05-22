CREATE DATABASE IF NOT EXISTS pencaucu_db;

USE pencaucu_db;

CREATE TABLE IF NOT EXISTS Players(
    email VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    PRIMARY KEY (email)
);

CREATE TABLE IF NOT EXISTS Admins(
    email VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    PRIMARY KEY (email)
);

CREATE TABLE IF NOT EXISTS Matches(
    id INT NOT NULL,
    date DATETIME NOT NULL,
    admin_email VARCHAR(64) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (admin_email) REFERENCES Admins(email)
);

CREATE TABLE IF NOT EXISTS Teams(
    name VARCHAR(64) NOT NULL,
    flag_image BLOB,
    PRIMARY KEY (name)
);

CREATE TABLE IF NOT EXISTS Results(
    team_name VARCHAR(64) NOT NULL,
    match_id INT NOT NULL,
    score INT NOT NULL,
    PRIMARY KEY (team_name, match_id),
    FOREIGN KEY (team_name) REFERENCES Teams(name),
    FOREIGN KEY (match_id) REFERENCES Matches(id)
);

CREATE TABLE IF NOT EXISTS Stadiums(
    id VARCHAR(64) NOT NULL,
    country VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    city VARCHAR(64) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS Group_Stages(
    id VARCHAR(255) NOT NULL,
    round_number INT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS Bets(
    player_email VARCHAR(64) NOT NULL,
    team_name VARCHAR(64) NOT NULL,
    match_id INT NOT NULL,
    team_score INT NOT NULL,
    PRIMARY KEY (player_email, team_name, match_id),
    FOREIGN KEY (player_email) REFERENCES Players(email),
    FOREIGN KEY (team_name) REFERENCES Teams(name),
    FOREIGN KEY (match_id) REFERENCES Matches(id)
);

CREATE TABLE IF NOT EXISTS Games(
    team_name VARCHAR(64) NOT NULL,
    match_id INT NOT NULL,
    CONSTRAINT result_id FOREIGN KEY (team_name, match_id) REFERENCES Results(team_name, match_id)
);

CREATE TABLE IF NOT EXISTS Player_Ranks(
    player_email VARCHAR(64) NOT NULL,
    team_name VARCHAR(64) NOT NULL,
    final_position INT NOT NULL,
    PRIMARY KEY (player_email, team_name),
    FOREIGN KEY (player_email) REFERENCES Players(email),
    FOREIGN KEY (team_name) REFERENCES Teams(name)
);

CREATE TABLE IF NOT EXISTS Group_Stages(
    match_id INT NOT NULL,
    group_id INT NOT NULL,
    PRIMARY KEY (match_id, group_id),
    FOREIGN KEY (match_id) REFERENCES Matches(id),
    FOREIGN KEY (group_id) REFERENCES Group_Stages(id)
);