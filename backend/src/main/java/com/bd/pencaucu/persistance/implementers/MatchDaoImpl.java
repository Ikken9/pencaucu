package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.mappers.MatchMapper;
import com.bd.pencaucu.persistance.interfaces.MatchDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class MatchDaoImpl implements MatchDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public Match findById(String id) {
        String sql = "SELECT id, date, admin_email FROM Match WHERE id = ?";
        List<Match> result = jdbcTemplate.query(sql, new MatchMapper(), id);

        if (!result.isEmpty()) {
            return result.get(0);
        }

        return null;
    }

    @Override
    public List<Match> findAll() {
        String sql = "SELECT * FROM Matches";

        return jdbcTemplate.query(sql, new MatchMapper());
    }

    @Override
    public void save(Match match) {
        String sql = "INSERT INTO Matches (id, date, admin_email) VALUES(?, ?, ?)";

        jdbcTemplate.update(sql,
                match.getId(),
                match.getDate(),
                match.getAdminEmail());
    }

    @Override
    public void update(Match match) {
        String sql = "UPDATE Matches SET " +
                "date = ?," +
                "admin_email = ? WHERE id = ?";

        jdbcTemplate.update(sql,
                match.getDate(),
                match.getAdminEmail(),
                match.getId());
    }

    @Override
    public void delete(String id) {
        String sql = "DELETE FROM Matches WHERE id = ?";

        jdbcTemplate.update(sql, id);
    }
}
