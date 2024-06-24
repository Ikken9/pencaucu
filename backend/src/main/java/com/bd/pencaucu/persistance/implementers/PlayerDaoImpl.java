package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.exceptions.ResourceAlreadyExistsException;
import com.bd.pencaucu.mappers.models.EmailMapper;
import com.bd.pencaucu.models.Player;
import com.bd.pencaucu.models.dto.PlayerDTO;
import com.bd.pencaucu.mappers.models.dto.PlayerDTOMapper;
import com.bd.pencaucu.persistance.interfaces.PlayerDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.sql.ResultSet;
import java.util.List;

@Repository
@RequiredArgsConstructor
public class PlayerDaoImpl implements PlayerDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public PlayerDTO findById(String username) throws UsernameNotFoundException, ResourceAlreadyExistsException {
        String sql =    "SELECT p.player_email, u.username, p.profile_picture, " +
                            "COALESCE(SUM(CASE " +
                                "WHEN r.team_score = b.team_score AND r.faced_team_score = b.faced_team_score THEN 4 " +
                                "WHEN (r.team_score > r.faced_team_score) AND (b.team_score >= r.team_score) THEN 2 " +
                                "WHEN (r.team_score = r.faced_team_score) AND (b.team_score = b.faced_team_score) THEN 2 " +
                                "WHEN (r.team_score < r.faced_team_score) AND (b.faced_team_score >= r.faced_team_score) THEN 2 " +
                                "ELSE 0 END), 0) AS player_score " +
                        "FROM " +
                            "Players p " +
                                "LEFT JOIN " +
                            "Bets b ON p.player_email = b.player_email " +
                                "LEFT JOIN " +
                            "Results r ON b.match_id = r.match_id " +
                                "LEFT JOIN " +
                            "Matches m ON b.match_id = m.id " +
                                "LEFT JOIN " +
                            "Users u ON u.email = p.player_email " +
                        "WHERE p.player_email = ? " +
                        "GROUP BY " +
                            "p.player_email, " +
                            "u.username;";

        List<PlayerDTO> players = jdbcTemplate.query(sql, new PlayerDTOMapper(), username);

        if (!players.isEmpty()) {
            return players.get(0);
        }

        String PLAYER_NOT_FOUND_MSG = "Player with username %s not found";
        throw new UsernameNotFoundException(String.format(PLAYER_NOT_FOUND_MSG, username));
    }

    @Override
    public List<PlayerDTO> findAll() {
        String sql =    "SELECT p.player_email, u.username, p.profile_picture, " +
                            "COALESCE(SUM(CASE " +
                                "WHEN r.team_score = b.team_score AND r.faced_team_score = b.faced_team_score THEN 4 " +
                                "WHEN (r.team_score > r.faced_team_score) AND (b.team_score >= r.team_score) THEN 2 " +
                                "WHEN (r.team_score = r.faced_team_score) AND (b.team_score = b.faced_team_score) THEN 2 " +
                                "WHEN (r.team_score < r.faced_team_score) AND (b.faced_team_score >= r.faced_team_score) THEN 2 " +
                                "ELSE 0 END), 0) AS player_score " +
                        "FROM " +
                            "Players p " +
                                "LEFT JOIN " +
                            "Bets b ON p.player_email = b.player_email " +
                                "LEFT JOIN " +
                            "Results r ON b.match_id = r.match_id " +
                                "LEFT JOIN " +
                            "Matches m ON b.match_id = m.id " +
                                "LEFT JOIN " +
                            "Users u ON u.email = p.player_email " +
                        "GROUP BY " +
                            "p.player_email, " +
                            "u.username " +
                        "ORDER BY player_score DESC;";

        return jdbcTemplate.query(sql, new PlayerDTOMapper());
    }

    @Override
    public List<String> findAllPlayersEmails() {
        String sql = "SELECT p.player_email FROM Players p";

        return jdbcTemplate.query(sql, new EmailMapper());
    }

    @Override
    public void save(Player player) {
        String sql = "INSERT INTO Players (player_email, career_name, profile_picture) VALUES (?, ?, ?)";

        jdbcTemplate.update(sql,
                player.getEmail(),
                player.getCareerName(),
                player.getProfilePictureUrl());

    }

    @Override
    public void update(Player player) {

    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Players WHERE player_email = ?";
        jdbcTemplate.update(sql, id);
    }
}
