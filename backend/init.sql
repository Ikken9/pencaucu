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
    name VARCHAR(64) NOT NULL,
    PRIMARY KEY (name)
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
    knockout_stage VARCHAR(64),
    stadium_id VARCHAR(20) NOT NULL,
    team_name VARCHAR(64) NOT NULL,
    faced_team_name VARCHAR(64) NOT NULL,
    admin_email VARCHAR(64) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (knockout_stage) REFERENCES Knockout_Stage(name),
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

-- START DUMMY INSERTS

-- CAREER EXAMPLE DATA INSERT
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingenieria en Informatica');
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingenieria en Alimentos');
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingenieria en Electronica');
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingenieria Industrial');

-- REGISTER USER
INSERT IGNORE INTO Users(email, username)
VALUES ('martin.caraballo@correo.ucu.edu.uy', 'Martin');

INSERT IGNORE INTO Users(email, username)
VALUES ('el.peluca@correo.ucu.edu.uy', 'El Peluca');

INSERT IGNORE INTO Users(email, username)
VALUES ('flacacho.hernandez@correo.ucu.edu.uy', 'Flacacho');

INSERT IGNORE INTO Users(email, username)
VALUES ('colorado.alcahuete@correo.ucu.edu.uy', 'Colorado Alcahuete');

INSERT IGNORE INTO Users(email, username)
VALUES ('elca.tador@correo.ucu.edu.uy', 'El Polaco Catador');

INSERT IGNORE INTO Users(email, username)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Dali');

INSERT IGNORE INTO Users(email, username)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', 'Santiago');

-- REGISTER USERS PASSWORDS
INSERT IGNORE INTO Logins(user_email, password)
VALUES ('martin.caraballo@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

INSERT IGNORE INTO Logins(user_email, password)
VALUES ('el.peluca@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

INSERT IGNORE INTO Logins(user_email, password)
VALUES ('flacacho.hernandez@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

INSERT IGNORE INTO Logins(user_email, password)
VALUES ('colorado.alcahuete@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

INSERT IGNORE INTO Logins(user_email, password)
VALUES ('dali.fernandez@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

INSERT IGNORE INTO Logins(user_email, password)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

INSERT IGNORE INTO Logins(user_email, password)
VALUES ('elca.tador@correo.ucu.edu.uy', '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0');

-- REGISTER ADMIN USER
INSERT IGNORE INTO Admins VALUES ('martin.caraballo@correo.ucu.edu.uy');

-- REGISTER PLAYER USERS
INSERT IGNORE INTO Players(player_email, career_name, profile_picture)
VALUES ('colorado.alcahuete@correo.ucu.edu.uy', 'Ingenieria en Informatica', 'https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/alcahuete.png');

INSERT IGNORE INTO Players(player_email, career_name, profile_picture)
VALUES ('el.peluca@correo.ucu.edu.uy', 'Ingenieria en Electronica', 'https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/peluca.jpg');

INSERT IGNORE INTO Players(player_email, career_name, profile_picture)
VALUES ('elca.tador@correo.ucu.edu.uy', 'Ingenieria Industrial', 'https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/catador.png');

INSERT IGNORE INTO Players(player_email, career_name)
VALUES ('flacacho.hernandez@correo.ucu.edu.uy', 'Ingenieria en Electronica');

INSERT IGNORE INTO Players(player_email, career_name)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Ingenieria en Informatica');

INSERT IGNORE INTO Players(player_email, career_name)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', 'Ingenieria en Informatica');

-- TEAMS EXAMPLE DATA INSERT
INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Uruguay', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/KnSUdQWiGRoy89q4x85IgA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Panama', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/JIn8OwxL6KFFiYrKGnL2RQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Argentina', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/1xBWyjjkA6vEWopPK3lIPA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Canada', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/H23oIEP6qK-zNc3O8abnIA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Peru', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/1ZizIpPB_eo-u8zYYjnFcg_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Chile', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/cI5rCchv6SjDgZ5NuEaMMQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Ecuador', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/AKqvkBpIyr-iLOK7Ig7-yQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Venezuela', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/ohjuAvV5dzyPZSEWWNNd_Q_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Mexico', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/yJF9xqmUGenD8108FJbg9A_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Jamaica', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/4HCKfsNJNHDY-vWSEzLbeQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Estados Unidos', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/wj9uZvn_vZrelLFGH8fnPA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Bolivia', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/SGxeD7yhwPj53FmPBmMMHg_96x96.png');

-- KNOCKOUT STAGE EXAMPLE DATA INSERT
INSERT IGNORE INTO Knockout_Stage(name)
VALUES ('Fase de Grupos');

INSERT IGNORE INTO Knockout_Stage(name)
VALUES ('Cuartos de final');

INSERT IGNORE INTO Knockout_Stage(name)
VALUES ('Semifinal');

INSERT IGNORE INTO Knockout_Stage(name)
VALUES ('Final');

-- MATCHES EXAMPLE DATA INSERT
INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (1, '2024-06-20 21:00:00', 'Fase de Grupos', 'MBS-ATL-EEUU', 'Argentina', 'Canada', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (2, '2024-06-21 21:00:00', 'Fase de Grupos', 'ATS-ARL-EEUU', 'Peru', 'Chile', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (3, '2024-06-22 19:00:00', 'Fase de Grupos', 'LS-SC-EEUU', 'Ecuador', 'Venezuela', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (4, '2024-06-22 22:00:00', 'Fase de Grupos', 'NRG-HSN-EEUU', 'Mexico', 'Jamaica', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (5, '2024-06-23 19:00:00', 'Fase de Grupos', 'ATS-ARL-EEUU', 'Estados Unidos', 'Bolivia', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (6, '2024-06-23 22:00:00', 'Fase de Grupos', 'HRS-MIA-EEUU', 'Uruguay', 'Panama', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (7, '2024-05-20 21:00:00', 'Fase de Grupos', 'MBS-ATL-EEUU', 'Argentina', 'Uruguay', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, knockout_stage, stadium_id, team_name, faced_team_name, admin_email)
VALUES (8, '2024-04-18 20:00:00', 'Fase de Grupos', 'MBS-ATL-EEUU', 'Uruguay', 'Canada', 'martin.caraballo@correo.ucu.edu.uy');

-- RESULTS EXAMPLE DATA INSERT
INSERT IGNORE INTO Results(match_id, team_score, faced_team_score)
VALUES (7, 3, 1);

INSERT IGNORE INTO Results(match_id, team_score, faced_team_score)
VALUES (8, 4, 2);

-- STADIUMS EXAMPLE DATA INSERT
INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('MBS-ATL-EEUU', 'Estados Unidos', 'Georgia', 'Atlanta', 'Mercedez-Benz-Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('ATS-ARL-EEUU', 'Estados Unidos', 'Texas', 'Arlington', 'AT&T Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('LS-SC-EEUU', 'Estados Unidos', 'California', 'Santa Clara', 'Levi\'s Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('NRG-HSN-EEUU', 'Estados Unidos', 'Texas', 'Houston', 'NRG Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('HRS-MIA-EEUU', 'Estados Unidos', 'Florida', 'Miami', 'Hard Rock Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('AHS-KSC-EEUU', 'Estados Unidos', 'Missouri', 'Kansas City', 'GEHA Field at Arrowhead Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('Q2S-AST-EEUU', 'Estados Unidos', 'Texas', 'Austin', 'Q2 Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('BAS-CLE-EEUU', 'Estados Unidos', 'North Carolina', 'Charlotte', 'Bank of America Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('MLS-ETR-EEUU', 'Estados Unidos', 'New Jersey', 'East Rutherford', 'MetLife Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('SFS-GDE-EEUU', 'Estados Unidos', 'Arizona', 'Glendale', 'State Farm Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('SIS-IWD-EEUU', 'Estados Unidos', 'California', 'Inglewood', 'SoFi Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('CMP-KSC-EEUU', 'Estados Unidos', 'Kansas', 'Kansas City', 'Children’s Mercy Park');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('ATS-PDE-EEUU', 'Estados Unidos', 'Nevada', 'Paradise', 'Allegiant Stadium');

INSERT IGNORE INTO Stadiums(id, country, state, city, name)
VALUES ('ICS-OAO-EEUU', 'Estados Unidos', 'Florida', 'Orlando', 'Inter&Co Stadium');

-- PLAYER RANKS EXAMPLE DATA INSERT
INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Uruguay', 1);

INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Argentina', 2);

INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', 'Uruguay', 1);

INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', 'Argentina', 2);

-- BETS EXAMPLE DATA INSERT
INSERT IGNORE INTO Bets(player_email, match_id, team_score, faced_team_score)
VALUES ('colorado.alcahuete@correo.ucu.edu.uy', 1, 1, 2);

INSERT IGNORE INTO Bets(player_email, match_id, team_score, faced_team_score)
VALUES ('el.peluca@correo.ucu.edu.uy', 2, 0, 1);

INSERT IGNORE INTO Bets(player_email, match_id, team_score, faced_team_score)
VALUES ('flacacho.hernandez@correo.ucu.edu.uy', 3, 2, 4);

INSERT IGNORE INTO Bets(player_email, match_id, team_score, faced_team_score)
VALUES ('elca.tador@correo.ucu.edu.uy', 4, 1, 1);

INSERT IGNORE INTO Bets(player_email, match_id, team_score, faced_team_score)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 5, 0, 0);

INSERT IGNORE INTO Bets(player_email, match_id, team_score, faced_team_score)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', 6, 4, 1);