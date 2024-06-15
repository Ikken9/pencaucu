package com.bd.pencaucu.mappers.models;

import com.bd.pencaucu.domain.models.Result;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class ResultMapper implements RowMapper<Result> {

    @Override
    public Result mapRow(ResultSet rs, int rowNum) throws SQLException {
        Result result = new Result();
        result.setTeamName(rs.getString("team_name"));
        result.setMatchId(rs.getInt("match_id"));
        result.setScore(rs.getInt("score"));

        return result;
    }
}
