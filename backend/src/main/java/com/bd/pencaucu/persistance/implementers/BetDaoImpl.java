package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.exceptions.ResourceNotFoundException;
import com.bd.pencaucu.mappers.models.BetMapper;
import com.bd.pencaucu.models.Bet;
import com.bd.pencaucu.persistance.interfaces.BetDao;
import lombok.AllArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@AllArgsConstructor
public class BetDaoImpl implements BetDao {

    JdbcTemplate jdbcTemplate;

    @Override
    public List<Bet> findById(String username) {
        String sql =    "SELECT player_email, match_id, team_score, faced_team_score FROM Bets b " +
                            "INNER JOIN Users u ON u.email = b.player_email " +
                        "WHERE u.username = ?";

        return jdbcTemplate.query(sql, new BetMapper(), username);
    }

    @Override
    public void save(Bet bet) {
        String sql = "INSERT INTO Bets(player_email, match_id, team_score, faced_team_score) VALUES(?, ?, ?, ?)";

        jdbcTemplate.update(sql,
                bet.getPlayerEmail(),
                bet.getMatchId(),
                bet.getTeamScore(),
                bet.getFacedTeamScore());
    }

    @Override
    public void update(Bet bet) {
        String sql = "UPDATE Bets SET team_score = ?, faced_team_score = ? WHERE player_email = ? AND match_id = ?";

        jdbcTemplate.update(sql,
                bet.getTeamScore(),
                bet.getFacedTeamScore(),
                bet.getPlayerEmail(),
                bet.getMatchId());
    }

    @Override
    public void delete(Bet bet) {
        String sql = "DELETE FROM FROM Bets WHERE player_email = ? AND match_id = ?";

        jdbcTemplate.update(sql, bet.getPlayerEmail(), bet.getMatchId());
    }
}
