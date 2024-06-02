package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.mappers.MatchMapper;
import com.bd.pencaucu.persistance.interfaces.MatchDao;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class MatchDaoImpl implements MatchDao {

    private JdbcTemplate jdbcTemplate;

    @Override
    public Match findById(String id) {
        String sql = "SELECT * FROM Match WHERE id = ?";
        List<Match> result = jdbcTemplate.query(sql, new MatchMapper(), id);

        if (!result.isEmpty()) {
            return result.get(0);
        }

        return null;
    }

    @Override
    public List<Match> findAll() {
        String sql = "SELECT * FROM Match";

        return jdbcTemplate.query(sql, new MatchMapper());
    }

    @Override
    public void save(Match match) {
        String sql = "INSERT INTO Match VALUES(?, ?, ?, ?, ?)";

        jdbcTemplate.update(sql,
                match.getId(),
                match.getDate(),
                match.getAdminEmail());
    }

    @Override
    public void update(Match match) {
        String sql = "UPDATE Match SET " +
                "xd = ?";

        jdbcTemplate.update(sql,
                match.getId(),
                match.getDate(),
                match.getAdminEmail());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Match WHERE id = ?";

        jdbcTemplate.update(sql, id);
    }
}
