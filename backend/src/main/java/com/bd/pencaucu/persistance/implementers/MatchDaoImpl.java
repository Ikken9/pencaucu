package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.domain.models.Match;
import com.bd.pencaucu.dto.MatchDTO;
import com.bd.pencaucu.mappers.MatchDTOMapper;
import com.bd.pencaucu.persistance.interfaces.MatchDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.*;

@Repository
@RequiredArgsConstructor
public class MatchDaoImpl implements MatchDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public List<MatchDTO> findAll() {
        String sql =    "SELECT m.id, m.date, s.name, m.team_name, m.faced_team_name, " +
                            "r.team_score, r.faced_team_score, " +
                            "t1.flag_image, t2.flag_image " +
                                "FROM Matches m " +
                                    "INNER JOIN Teams t1 ON m.team_name = t1.name " +
                                    "INNER JOIN Teams t2 ON m.faced_team_name = t2.name " +
                                    "INNER JOIN Stadiums s ON m.stadium_id = s.id " +
                                    "INNER JOIN Results r ON r.match_id = m.id; ";

        List<MatchDTO> queryResult = jdbcTemplate.query(sql, new MatchDTOMapper());

        if (!queryResult.isEmpty()) {
            return queryResult;
        }

        return null;
    }

    @Override
    public MatchDTO findById(String id) {
        String sql =    "SELECT m.id, m.date, s.name, m.team_name, m.faced_team_name, " +
                            "r.team_score, r.faced_team_score, " +
                            "t1.flag_image, t2.flag_image " +
                                "FROM Matches m " +
                                    "INNER JOIN Teams t1 ON m.team_name = t1.name " +
                                    "INNER JOIN Teams t2 ON m.faced_team_name = t2.name " +
                                    "INNER JOIN Stadiums s ON m.stadium_id = s.id " +
                                    "INNER JOIN Results r ON r.match_id = m.id " +
                                    "WHERE m.id = ?;";

        List<MatchDTO> queryResult = jdbcTemplate.query(sql, new MatchDTOMapper(), id);

        if (!queryResult.isEmpty()) {
            return queryResult.get(0);
        }

        return null;
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
