package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.models.Result;
import com.bd.pencaucu.exceptions.ResourceNotFoundException;
import com.bd.pencaucu.mappers.models.ResultMapper;
import com.bd.pencaucu.persistance.interfaces.ResultDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class ResultDaoImpl implements ResultDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public Result findById(int matchId) throws ResourceNotFoundException {
        String sql = "SELECT match_id, team_score, faced_team_score FROM Results WHERE match_id = ?";
        List<Result> results = jdbcTemplate.query(sql, new ResultMapper(), matchId);

        if (!results.isEmpty()) {
            return results.get(0);
        }

        String RESULT_NOT_FOUND_MSG = "Results with match_id %d not found";
        throw new ResourceNotFoundException(String.format(RESULT_NOT_FOUND_MSG, matchId));
    }

    @Override
    public List<Result> findAll() {
        String sql = "SELECT match_id, team_score, faced_team_score FROM Results";
        List<Result> results = jdbcTemplate.query(sql, new ResultMapper());

        if (!results.isEmpty()) {
            return results;
        }
        throw new ResourceNotFoundException("Results not found");
    }

    @Override
    public void save(Result result) {
        String sql = "INSERT INTO Results(match_id, team_score, faced_team_score) VALUES(?, ?, ?)";

        jdbcTemplate.update(sql,
                result.getMatchId(),
                result.getTeamScore(),
                result.getFacedTeamScore());
    }

    @Override
    public void update(Result result) {
        String sql = "UPDATE Results SET team_score = ?, faced_team_score = ? WHERE match_id = ?";

        jdbcTemplate.update(sql,
                result.getTeamScore(),
                result.getFacedTeamScore(),
                result.getMatchId());
    }

    @Override
    public void delete(int id) {
        String sql = "DELETE FROM Results WHERE match_id = ?";

        jdbcTemplate.update(sql, id);
    }
}
