package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.models.Result;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class ResultMapper implements RowMapper<Result> {

    @Override
    public Result mapRow(ResultSet rs, int rowNum) throws SQLException {
        Result result = new Result();
        result.setMatchId(rs.getInt("match_id"));
        result.setTeamScore(rs.getInt("team_score"));
        result.setFacedTeamScore(rs.getInt("faced_team_score"));

        return result;
    }
}
