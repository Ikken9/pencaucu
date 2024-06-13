package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Result;
import com.bd.pencaucu.exceptions.ResourceNotFoundException;
import com.bd.pencaucu.mappers.ResultMapper;
import com.bd.pencaucu.persistance.interfaces.ResultDao;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class ResultDaoImpl implements ResultDao {

    private final String RESULT_NOT_FOUND_MSG = "Results with %s %s not found";

    @Autowired
    private JdbcTemplate jdbcTemplate;

    @Override
    public List<Result> findByMatchId(int matchId) throws ResourceNotFoundException {
        String sql = "SELECT team_name, match_id, score FROM Matches WHERE match_id = ?";
        List<Result> results = jdbcTemplate.query(sql, new ResultMapper(), matchId);

        if (!results.isEmpty()) {
            return results;
        }

        throw new ResourceNotFoundException(String.format(RESULT_NOT_FOUND_MSG, "match id", matchId));
    }

    @Override
    public List<Result> findByTeamName(String teamName) throws ResourceNotFoundException {
        String sql = "SELECT team_name, match_id, score FROM Matches WHERE team_name = ?";
        List<Result> results = jdbcTemplate.query(sql, new ResultMapper(), teamName);

        if (!results.isEmpty()) {
            return results;
        }

        throw new ResourceNotFoundException(String.format(RESULT_NOT_FOUND_MSG, "team name", teamName));
    }

    @Override
    public void save(Result result) {
        String sql = "INSERT INTO Results(team_name, match_id, score) VALUES(?, ?, ?)";

        jdbcTemplate.update(sql,
                result.getTeamName(),
                result.getMatchId(),
                result.getScore());
    }

    @Override
    public void update(Result result) {
        String sql = "UPDATE Results SET score = ? WHERE team_name = ? AND match_id = ?";

        jdbcTemplate.update(sql,
                result.getScore(),
                result.getTeamName(),
                result.getMatchId());
    }

    @Override
    public void delete(Result result) {
        String sql = "DELETE FROM Results WHERE team_name = ? AND match_id = ?";

        jdbcTemplate.update(sql,
                result.getTeamName(),
                result.getMatchId());

    }
}
