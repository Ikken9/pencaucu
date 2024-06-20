package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.models.Match;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class MatchMapper implements RowMapper<Match> {

    @Override
    public Match mapRow(ResultSet rs, int rowNum) throws SQLException {
        Match match = new Match();
        match.setId(rs.getInt("id"));
        match.setDate(rs.getTimestamp("date"));
        match.setAdminEmail(rs.getString("admin_email"));

        return match;
    }
}
