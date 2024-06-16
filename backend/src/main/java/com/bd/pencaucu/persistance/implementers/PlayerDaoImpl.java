package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Player;
import com.bd.pencaucu.dto.PlayerDTO;
import com.bd.pencaucu.mappers.dtos.PlayerDTOMapper;
import com.bd.pencaucu.mappers.models.PlayerMapper;
import com.bd.pencaucu.persistance.interfaces.PlayerDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class PlayerDaoImpl implements PlayerDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public PlayerDTO findById(String username) throws UsernameNotFoundException {
        String sql =    "SELECT p.player_email, u.username, " +
                            "SUM(CASE " +
                                "WHEN r.team_score = b.team_score AND r.faced_team_score = b.faced_team_score THEN 4 " +
                                "WHEN (r.team_score > r.faced_team_score) AND (b.team_score >= r.team_score) THEN 2 " +
                                "WHEN (r.team_score = r.faced_team_score) AND (b.team_score = b.faced_team_score) THEN 2 " +
                                "WHEN (r.team_score < r.faced_team_score) AND (b.faced_team_score >= r.faced_team_score) THEN 2 " +
                                "ELSE 0 END) AS player_score " +
                        "FROM " +
                            "Players p " +
                                "INNER JOIN " +
                            "Bets b ON p.player_email = b.player_email " +
                                "INNER JOIN " +
                            "Results r ON b.match_id = r.match_id " +
                                "INNER JOIN " +
                            "Matches m ON b.match_id = m.id " +
                                "INNER JOIN " +
                            "Users u ON u.email = p.player_email " +
                        "WHERE u.username = ? " +
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
        String sql =    "SELECT p.player_email, u.username, " +
                            "SUM(CASE " +
                                "WHEN r.team_score = b.team_score AND r.faced_team_score = b.faced_team_score THEN 4 " +
                                "WHEN (r.team_score > r.faced_team_score) AND (b.team_score >= r.team_score) THEN 2 " +
                                "WHEN (r.team_score = r.faced_team_score) AND (b.team_score = b.faced_team_score) THEN 2 " +
                                "WHEN (r.team_score < r.faced_team_score) AND (b.faced_team_score >= r.faced_team_score) THEN 2 " +
                                "ELSE 0 END) AS player_score " +
                        "FROM " +
                            "Players p " +
                                "INNER JOIN " +
                            "Bets b ON p.player_email = b.player_email " +
                                "INNER JOIN " +
                            "Results r ON b.match_id = r.match_id " +
                                "INNER JOIN " +
                            "Matches m ON b.match_id = m.id " +
                                "INNER JOIN " +
                            "Users u ON u.email = p.player_email " +
                        "GROUP BY " +
                            "p.player_email, " +
                            "u.username;";

        return jdbcTemplate.query(sql, new PlayerDTOMapper());
    }

    @Override
    public void save(Player player) {
        String sql = "INSERT INTO Players (player_email, name, career) VALUES (?, ?, ?)";

        jdbcTemplate.update(sql,
                player.getEmail(),
                player.getCareerName());

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
