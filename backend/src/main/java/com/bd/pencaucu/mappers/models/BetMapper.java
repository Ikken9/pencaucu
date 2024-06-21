package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.models.Bet;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class BetMapper implements RowMapper<Bet> {
    @Override
    public Bet mapRow(ResultSet rs, int rowNum) throws SQLException {
        Bet bet = new Bet();
        bet.setPlayerEmail(rs.getString("player_email"));
        bet.setMatchId(rs.getInt("match_id"));
        bet.setTeamScore(rs.getInt("team_score"));
        bet.setFacedTeamScore(rs.getInt("faced_team_score"));

        return bet;
    }
}
