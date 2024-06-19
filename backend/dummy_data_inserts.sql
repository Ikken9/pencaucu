-- CAREER EXAMPLE DATA INSERT
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingeniería en Informática');
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingeniería en Alimentos');
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingeniería en Electrónica');
INSERT IGNORE INTO Careers(career_name) VALUES ('Ingeniería Industrial');

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
VALUES ('colorado.alcahuete@correo.ucu.edu.uy', 'Ingeniería en Informática', 'https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/alcahuete.png');

INSERT IGNORE INTO Players(player_email, career_name, profile_picture)
VALUES ('el.peluca@correo.ucu.edu.uy', 'Ingeniería en Electrónica', 'https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/peluca.jpg');

INSERT IGNORE INTO Players(player_email, career_name, profile_picture)
VALUES ('elca.tador@correo.ucu.edu.uy', 'Ingeniería Industrial', 'https://raw.githubusercontent.com/Ikken9/pencaucu/dev/cdn/catador.png');

INSERT IGNORE INTO Players(player_email, career_name)
VALUES ('flacacho.hernandez@correo.ucu.edu.uy', 'Ingeniería en Electrónica');

INSERT IGNORE INTO Players(player_email, career_name)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Ingeniería en Informática');

-- TEAMS EXAMPLE DATA INSERT
INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Uruguay', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/KnSUdQWiGRoy89q4x85IgA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Panamá', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/JIn8OwxL6KFFiYrKGnL2RQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Argentina', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/1xBWyjjkA6vEWopPK3lIPA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Canadá', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/H23oIEP6qK-zNc3O8abnIA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Perú', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/1ZizIpPB_eo-u8zYYjnFcg_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Chile', 'A', 'https://ssl.gstatic.com/onebox/media/sports/logos/cI5rCchv6SjDgZ5NuEaMMQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Ecuador', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/AKqvkBpIyr-iLOK7Ig7-yQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Venezuela', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/ohjuAvV5dzyPZSEWWNNd_Q_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('México', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/yJF9xqmUGenD8108FJbg9A_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Jamaica', 'B', 'https://ssl.gstatic.com/onebox/media/sports/logos/4HCKfsNJNHDY-vWSEzLbeQ_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Estados Unidos', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/wj9uZvn_vZrelLFGH8fnPA_96x96.png');

INSERT IGNORE INTO Teams(name, group_letter, flag_image)
VALUES ('Bolivia', 'C', 'https://ssl.gstatic.com/onebox/media/sports/logos/SGxeD7yhwPj53FmPBmMMHg_96x96.png');

-- MATCHES EXAMPLE DATA INSERT
INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (1, '2024-06-20 21:00:00', 'MBS-ATL-EEUU', 'Argentina', 'Canadá', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (2, '2024-06-21 21:00:00', 'ATS-ARL-EEUU', 'Perú', 'Chile', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (3, '2024-06-22 19:00:00', 'LS-SC-EEUU', 'Ecuador', 'Venezuela', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (4, '2024-06-22 22:00:00', 'NRG-HSN-EEUU', 'México', 'Jamaica', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (5, '2024-06-23 19:00:00', 'ATS-ARL-EEUU', 'Estados Unidos', 'Bolivia', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (6, '2024-06-23 22:00:00', 'HRS-MIA-EEUU', 'Uruguay', 'Panamá', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (7, '2024-05-20 21:00:00', 'MBS-ATL-EEUU', 'Argentina', 'Uruguay', 'martin.caraballo@correo.ucu.edu.uy');

INSERT IGNORE INTO Matches(id, date, stadium_id, team_name, faced_team_name, admin_email)
VALUES (8, '2024-04-18 20:00:00', 'MBS-ATL-EEUU', 'Uruguay', 'Canadá', 'martin.caraballo@correo.ucu.edu.uy');

-- RESULTS EXAMPLE DATA INSERT
INSERT IGNORE INTO Results(match_id)
VALUES (1), (2), (3), (4), (5), (6);

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

-- KNOCKOUT STAGE EXAMPLE DATA INSERT
INSERT IGNORE INTO Knockout_Stage(id, name)
VALUES (1, 'Cuartos de final');

INSERT IGNORE INTO Knockout_Stage(id, name)
VALUES (2, 'Semifinal');

INSERT IGNORE INTO Knockout_Stage(id, name)
VALUES (3, 'Final');

-- PLAYER RANKS EXAMPLE DATA INSERT
INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Uruguay', 1);

INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Uruguay', 2);

INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('dali.fernandez@correo.ucu.edu.uy', 'Uruguay', 3);

INSERT IGNORE INTO Player_Ranks(player_email, team_name, final_position)
VALUES ('santiago.berrueta@correo.ucu.edu.uy', 'Uruguay', 4);

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