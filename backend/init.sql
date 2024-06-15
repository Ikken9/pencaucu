CREATE DATABASE IF NOT EXISTS pencaucu_db;

USE pencaucu_db;

CREATE TABLE IF NOT EXISTS Careers(
    career_name VARCHAR(64) NOT NULL,
    PRIMARY KEY (career_name)
);

CREATE TABLE IF NOT EXISTS Users(
    email VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
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
    flag_image VARCHAR(128),
    PRIMARY KEY (name)
);

CREATE TABLE IF NOT EXISTS Matches(
    id INT NOT NULL,
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

#LOAD DUMMY DATA TO DATABASE
# REGISTER ADMIN USER
INSERT INTO Users(email, name)
    VALUES (
           'martin.caraballo@correo.ucu.edu.uy',
           'Martin'
          );

INSERT INTO Logins(user_email, password)
    VALUES (
            'martin.caraballo@correo.ucu.edu.uy',
            '$argon2id$v=19$m=16384,t=2,p=1$d3wl2tx3lAa1WcCqtEyO8Q$Oy9THcFIYwk2sb2WODcW1xsZVXFcQwqWJmVF+ZZeOu0'
           );

INSERT INTO Admins VALUES ('martin.caraballo@correo.ucu.edu.uy');

# CAREER EXAMPLE DATA INSERT
INSERT INTO Careers(career_name) VALUES ('Ingeniería en Informática');
INSERT INTO Careers(career_name) VALUES ('Ingeniería en Alimentos');
INSERT INTO Careers(career_name) VALUES ('Ingeniería en Electrónica');
INSERT INTO Careers(career_name) VALUES ('Ingeniería Industrial');

# TEAMS EXAMPLE DATA INSERT
INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Uruguay', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/KnSUdQWiGRoy89q4x85IgA_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Panamá', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/JIn8OwxL6KFFiYrKGnL2RQ_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Argentina', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/1xBWyjjkA6vEWopPK3lIPA_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Canadá', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/H23oIEP6qK-zNc3O8abnIA_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Perú', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/1ZizIpPB_eo-u8zYYjnFcg_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Chile', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/cI5rCchv6SjDgZ5NuEaMMQ_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Ecuador', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/AKqvkBpIyr-iLOK7Ig7-yQ_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Venezuela', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/ohjuAvV5dzyPZSEWWNNd_Q_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('México', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/yJF9xqmUGenD8108FJbg9A_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Jamaica', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/4HCKfsNJNHDY-vWSEzLbeQ_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Estados Unidos', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/wj9uZvn_vZrelLFGH8fnPA_96x96.png');

INSERT INTO Teams(name, group_letter, flag_image)
VALUES ('Bolivia', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/SGxeD7yhwPj53FmPBmMMHg_96x96.png');

# MATCHES EXAMPLE DATA INSERT
INSERT INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
    VALUES (1, '2024-06-20 21:00:00', 'MBS-ATL-EEUU', 'Argentina', 'Canadá', 'martin.caraballo@correo.ucu.edu.uy');

INSERT INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
    VALUES (2, '2024-06-21 21:00:00', 'ATS-ARL-EEUU', 'Perú', 'Chile', 'martin.caraballo@correo.ucu.edu.uy');

INSERT INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
    VALUES (3, '2024-06-22 19:00:00', 'LS-SC-EEUU', 'Ecuador', 'Venezuela', 'martin.caraballo@correo.ucu.edu.uy');

INSERT INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
    VALUES (4, '2024-06-22 22:00:00', 'NRG-HSN-EEUU', 'México', 'Jamaica', 'martin.caraballo@correo.ucu.edu.uy');

INSERT INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
    VALUES (5, '2024-06-23 19:00:00', 'ATS-ARL-EEUU', 'Estados Unidos', 'Bolivia', 'martin.caraballo@correo.ucu.edu.uy');

INSERT INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
    VALUES (6, '2024-06-23 22:00:00', 'HRS-MIA-EEUU', 'Uruguay', 'Panamá', 'martin.caraballo@correo.ucu.edu.uy');

# RESULTS EXAMPLE DATA INSERT
INSERT INTO Results(match_id)
    VALUES (1);

INSERT INTO Results(match_id)
    VALUES (2);

INSERT INTO Results(match_id)
    VALUES (3);

INSERT INTO Results(match_id)
    VALUES (4);

INSERT INTO Results(match_id)
    VALUES (5);

INSERT INTO Results(match_id)
    VALUES (6);

# STADIUMS EXAMPLE DATA INSERT
INSERT INTO Stadiums(id, country, state, city, name)
    VALUES (
            'MBS-ATL-EEUU',
            'Estados Unidos',
            'Georgia',
            'Atlanta',
            'Mercedez-Benz-Stadium'
           );

INSERT INTO Stadiums(id, country, state, city, name)
    VALUES (
           'ATS-ARL-EEUU',
           'Estados Unidos',
           'Texas',
           'Arlington',
           'AT&T Stadium'
       );

INSERT INTO Stadiums(id, country, state, city, name)
    VALUES (
           'LS-SC-EEUU',
           'Estados Unidos',
           'California',
           'Santa Clara',
           'Levi\'s Stadium'
       );

INSERT INTO Stadiums(id, country, state, city, name)
    VALUES (
           'NRG-HSN-EEUU',
           'Estados Unidos',
           'Texas',
           'Houston',
           'NRG Stadium'
       );

INSERT INTO Stadiums(id, country, state, city, name)
    VALUES (
           'HRS-MIA-EEUU',
           'Estados Unidos',
           'Florida',
           'Miami',
           'Hard Rock Stadium'
       );

# KNOCKOUT STAGE EXAMPLE DATA INSERT
INSERT INTO Knockout_Stage(id, name)
    VALUES (
            1,
            'Cuartos de final'
           );

INSERT INTO Knockout_Stage(id, name)
    VALUES (
           2,
           'Semifinal'
       );

INSERT INTO Knockout_Stage(id, name)
    VALUES (
           3,
           'Final'
       );