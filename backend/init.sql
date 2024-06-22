CREATE DATABASE IF NOT EXISTS pencaucu_db;

USE pencaucu_db;

CREATE TABLE IF NOT EXISTS Careers(
    career_name VARCHAR(64) NOT NULL,
    PRIMARY KEY (career_name)
);

CREATE TABLE IF NOT EXISTS Users(
    email VARCHAR(64) NOT NULL,
    username VARCHAR(64) UNIQUE NOT NULL,
    PRIMARY KEY (email)
);

CREATE TABLE IF NOT EXISTS Logins(
    user_email VARCHAR(64) NOT NULL,
    password VARCHAR(256) NOT NULL,
    PRIMARY KEY (user_email),
    FOREIGN KEY (user_email) REFERENCES Users(email)
);

CREATE TABLE IF NOT EXISTS Players(
    player_email VARCHAR(64) NOT NULL,
    career_name VARCHAR(64) NOT NULL,
    profile_picture VARCHAR(256),
    PRIMARY KEY (player_email),
    FOREIGN KEY (player_email) REFERENCES Users(email),
    FOREIGN KEY (career_name) REFERENCES Careers(career_name)
);

CREATE TABLE IF NOT EXISTS Admins(
    admin_email VARCHAR(64) NOT NULL,
    PRIMARY KEY (admin_email),
    FOREIGN KEY (admin_email) REFERENCES Users(email)
);

CREATE TABLE IF NOT EXISTS Knockout_Stage(
    id INT,
    name VARCHAR(64) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS Teams(
    name VARCHAR(64) NOT NULL,
    group_letter CHAR NOT NULL,
    flag_image VARCHAR(256),
    PRIMARY KEY (name)
);

CREATE TABLE IF NOT EXISTS Matches(
    id INT AUTO_INCREMENT,
    date DATETIME NOT NULL,
    knockout_stage_id INT,
    stadium_id VARCHAR(20) NOT NULL,
    team_name VARCHAR(64) NOT NULL,
    faced_team_name VARCHAR(64) NOT NULL,
    admin_email VARCHAR(64) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (knockout_stage_id) REFERENCES Knockout_Stage(id),
    FOREIGN KEY (team_name) REFERENCES Teams(name),
    FOREIGN KEY (faced_team_name) REFERENCES Teams(name),
    FOREIGN KEY (admin_email) REFERENCES Admins(admin_email)
);

CREATE TABLE IF NOT EXISTS Results(
    match_id INT NOT NULL,
    team_score INT,
    faced_team_score INT,
    PRIMARY KEY (match_id),
    FOREIGN KEY (match_id) REFERENCES Matches(id)
);

CREATE TABLE IF NOT EXISTS Stadiums(
    id VARCHAR(20) NOT NULL,
    country VARCHAR(64) NOT NULL,
    state VARCHAR(64) NOT NULL,
    city VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS Bets(
    player_email VARCHAR(64) NOT NULL,
    match_id INT NOT NULL,
    team_score INT NOT NULL,
    faced_team_score INT NOT NULL,
    PRIMARY KEY (player_email, match_id),
    FOREIGN KEY (player_email) REFERENCES Players(player_email),
    FOREIGN KEY (match_id) REFERENCES Matches(id)
);

CREATE TABLE IF NOT EXISTS Player_Ranks(
    player_email VARCHAR(64) NOT NULL,
    team_name VARCHAR(64) NOT NULL,
    final_position INT NOT NULL,
    PRIMARY KEY (player_email, team_name),
    FOREIGN KEY (player_email) REFERENCES Players(player_email),
    FOREIGN KEY (team_name) REFERENCES Teams(name)
);