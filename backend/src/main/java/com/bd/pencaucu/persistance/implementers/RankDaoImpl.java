package com.bd.pencaucu.persistance.implementers;

import com.bd.pencaucu.mappers.models.dto.RankDTOMapper;
import com.bd.pencaucu.models.Rank;
import com.bd.pencaucu.models.dto.RankDTO;
import com.bd.pencaucu.persistance.interfaces.RankDao;
import lombok.RequiredArgsConstructor;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
@RequiredArgsConstructor
public class RankDaoImpl implements RankDao {

    private final JdbcTemplate jdbcTemplate;

    @Override
    public List<RankDTO> findPlayerRanks(String id) {
        String sql = "SELECT team_name, final_position FROM Player_Ranks WHERE player_id=?";
        return jdbcTemplate.query(sql, new RankDTOMapper(), id);
    }

    @Override
    public void save(Rank rank) {
        String sql = "INSERT INTO Player_Ranks (player_email, team_name, final_position) VALUES (?, ?, ?)";

        jdbcTemplate.update(sql, rank.getPlayerEmail(), rank.getTeamName(), rank.getFinalPosition());
    }
}
